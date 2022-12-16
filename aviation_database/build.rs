use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rayon::prelude::*;
use serde::Deserialize;
use std::{env, fs, io::BufRead, path::Path};

/// Data type of the JSON based airport database
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Airport {
    pub continent: String,
    pub coordinates: String,
    pub elevation_ft: Option<String>,
    pub gps_code: Option<String>,
    pub iata_code: Option<String>,
    pub ident: String,
    pub iso_country: String,
    pub iso_region: String,
    pub local_code: Option<String>,
    pub municipality: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

/// Convert an [Airport] to the type compatible with the K-d tree implementation
fn parse_airport_kd(airport: Airport) -> ([f64; 3], Airport) {
    let mut coord = airport.coordinates.split(", ");
    let lon: f64 = coord.next().map(|l| l.parse().ok()).flatten().unwrap();
    let lat: f64 = coord.next().map(|l| l.parse().ok()).flatten().unwrap();
    let alt: f64 = airport
        .elevation_ft
        .as_ref()
        .map_or_else(|| Some(0.0), |e| e.parse().ok())
        .unwrap();

    let x = alt * lat.cos() * lon.cos();
    let y = alt * lat.cos() * lon.sin();
    let z = alt * lat.sin();
    ([x, y, z], airport)
}

// allow an [Airport] to be appended to an existing [TokenStream]
impl ToTokens for Airport {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut coord = self.coordinates.split(", ");
        let long: Option<f64> = coord.next().map(|l| l.parse().ok()).flatten();
        let lat: Option<f64> = coord.next().map(|l| l.parse().ok()).flatten();
        let alt: Option<f64> = self
            .elevation_ft
            .as_ref()
            .map_or_else(|| Some(0.0), |e| e.parse().ok());
        let icao = self
            .ident
            .chars() // convert to characters
            .chain(std::iter::repeat('\0')) // fill with null bytes ...
            .take(4); // ... if necessary to get exactly 4 bytes

        // discard if the airport has no position
        if long.is_none() || lat.is_none() || alt.is_none() {
            return;
        }

        tokens.extend(quote! {
        AirportEntry {
            icao: [ #(#icao as u8),* ],
            lat: #lat,
            lon: #long,
            alt: #alt,
            }
        });
    }
}

fn main() {
    let file_env = "AIRPORTS_JSON_FILE";
    let url_env = "AIRPORTS_JSON_URL";

    // if the file was specified as environment variable, use the file
    let reader: Box<dyn BufRead> = if let Ok(airports_file) = std::env::var(file_env) {
        let f = std::fs::File::open(&airports_file)
            .expect(&format!("file {} does not exist", airports_file));
        Box::new(std::io::BufReader::new(f))
    } else {
        // or else download the data
        let airports_url = std::env::var(url_env)
            .unwrap_or("https://datahub.io/core/airport-codes/r/airport-codes.json".into());
        let r = ureq::get(&airports_url)
            .call()
            .expect(&format!(
                "unable to automatically download from {}",
                airports_url
            ))
            .into_reader();
        Box::new(std::io::BufReader::new(r))
    };

    // parse the file to a Vec of `Airport`
    let airports: Vec<Airport> = serde_json::from_reader(reader).unwrap();

    let num_airports = airports.len();
    // convert `Airport`s to K-d tree compatible type and build the tree
    let airports: Vec<([f64; 3], Airport)> =
        airports.into_par_iter().map(parse_airport_kd).collect();
    let airports = kd_tree_sort::sort(airports);

    // convert the structured K-d tree to a [TokenStream]
    let airports_tokenstream = airports.into_iter().map(|([x, y, z], ap)| {
        quote!(
                Node::new(
                     [ #x, #y, #z ],
                    #ap
                )
        )
    });

    let max_level = (num_airports as f64).log2() as usize + 2;

    // join the [TokenStream]s to everything that goes into one file
    let complete_file = quote!(
        pub const NODES: [Node<f64, AirportEntry, 3>; #num_airports] = [ #(#airports_tokenstream),* ];
        pub const AIRPORTS: Tree::<f64, AirportEntry, 3, #max_level> = Tree::new( &NODES );
    );

    // write the file
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("airports.rs");
    let syn_file: syn::File = syn::parse2(complete_file).unwrap();
    let rust_code = prettyplease::unparse(&syn_file).into_bytes();
    fs::write(&dest_path, rust_code).unwrap();

    println!(
        "cargo:rerun-if-changed=build.rs
        cargo:rerun-if-env-changed={url_env}
        cargo:rerun-if-env-changed={file_env}"
    );
}
