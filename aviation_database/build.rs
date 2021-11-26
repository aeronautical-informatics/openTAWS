use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};
use rayon::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::process::Command;

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

impl ToTokens for Airport {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut coord = self.coordinates.split(", ");
        let long: Option<f64> = coord.next().map(|l| l.parse().ok()).flatten();
        let lat: Option<f64> = coord.next().map(|l| l.parse().ok()).flatten();
        let alt: Option<f64> = self
            .elevation_ft
            .as_ref()
            .map_or_else(|| Some(0.0), |e| e.parse().ok());
        let icao = format!(
            "b\"{}\"",
            self.ident
                .chars()
                .chain(std::iter::repeat('\0'))
                .take(4)
                .collect::<String>()
        )
        .parse::<Literal>()
        .unwrap();
        if long.is_none() || lat.is_none() || alt.is_none() {
            return;
        }

        tokens.extend(quote! {
        AirportEntry {
            icao: *#icao,
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

    let reader: Box<dyn BufRead> = if let Ok(airports_file) = std::env::var(file_env) {
        let f = std::fs::File::open(&airports_file)
            .expect(&format!("file {} does not exist", airports_file));
        Box::new(std::io::BufReader::new(f))
    } else {
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

    let mut airports: Vec<Airport> = serde_json::from_reader(reader).unwrap();

    let num_airports = airports.len();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("airports.rs");
    let airports: Vec<([f64; 3], Airport)> = airports.par_drain(..).map(parse_airport_kd).collect();
    let airports = kd_tree_sort::sort(airports);
    let airports: String = airports
        .iter()
        .fold(String::new(), |mut s, ([x, y, z], a)| {
            //s.push_str(&quote!(#a).to_string());
            // Node<T, V, DIM>
            // new(val: [T; DIM], v: V)
            s.push_str(
                &quote! {
                    Node::new(
                         [ #x, #y, #z ],
                        #a
                    )
                }
                .to_string(),
            );
            s.push(',');
            s
        });
    let max_level = (num_airports as f64).log2() as usize + 2;
    fs::write(
        &dest_path,
        format!(
            "pub const NODES: [Node<f64, AirportEntry, 3>; {}] = [ {} ];\
            pub const AIRPORTS: Tree::<f64, AirportEntry, 3, {}> = Tree::new( &NODES );",
            num_airports, airports, max_level
        ),
    )
    .unwrap();

    // format code
    if let Err(e) = Command::new("rustfmt")
        .arg(dest_path.as_os_str())
        .current_dir(&out_dir)
        .status()
    {
        eprintln!("{}", e)
    }

    println!(
        "cargo:rerun-if-changed=build.rs
    cargo:rerun-if-env-changed={}
    cargo:rerun-if-env-changed={}
    ",
        url_env, file_env
    );
}
