use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use serde::{de, Deserialize, Deserializer};
use serde_with::serde_as;
use uom::si::{
    angle::degree,
    f64::{Angle, Length},
    length::foot,
};

fn main() {
    const FILE_ENV: &str = "AIRPORTS_JSON_FILE";
    const URL_ENV: &str = "AIRPORTS_JSON_URL";
    const DEFAULT_URL: &str = "https://datahub.io/core/airport-codes/r/airport-codes.json";

    let file_path = std::env::var(FILE_ENV).ok();
    let url = std::env::var(URL_ENV).unwrap_or_else(|_| DEFAULT_URL.to_string());

    let reader = match file_path {
        Some(file_path) => read_file(file_path),
        None => download_file(url),
    };

    let airports = get_airports(reader);

    println!("{:?}", airports)
}

fn read_file<S: AsRef<str>>(file_path: S) -> Box<dyn BufRead> {
    let file_path = file_path.as_ref();
    let file =
        File::open(file_path).unwrap_or_else(|_| panic!("Failed to open: \"{}\"", file_path));

    Box::new(BufReader::new(file))
}

fn download_file<S: AsRef<str>>(url: S) -> Box<dyn BufRead> {
    let url = url.as_ref();
    let reader = ureq::get(url)
        .call()
        .unwrap_or_else(|_| panic!("Failed to download: \"{}\"", url))
        .into_reader();

    Box::new(BufReader::new(reader))
}

fn get_airports(reader: impl BufRead) -> Vec<Airport> {
    let airports: Airports = serde_json::from_reader(reader).unwrap();
    airports.0
}

#[serde_as]
#[derive(Debug, serde::Deserialize)]
struct Airports(#[serde_as(as = "serde_with::VecSkipError<_>")] Vec<Airport>);

#[derive(Debug, Deserialize)]
struct Airport {
    #[serde(deserialize_with = "read_name")]
    name: Option<String>,
    #[serde(deserialize_with = "read_municipality")]
    municipality: Option<String>,
    #[serde(deserialize_with = "read_iata_code")]
    iata_code: Option<String>,
    #[serde(deserialize_with = "read_continent")]
    continent: Option<String>,
    #[serde(deserialize_with = "read_iso_country")]
    iso_country: Option<String>,
    #[serde(deserialize_with = "read_iso_region")]
    iso_region: Option<String>,
    #[serde(deserialize_with = "read_coordinates")]
    coordinates: Option<(Angle, Angle)>,
    #[serde(deserialize_with = "read_elevation_ft")]
    elevation_ft: Option<Length>,
    #[serde(rename = "type", deserialize_with = "read_airport_type")]
    typ: Option<AirportType>,
}

#[derive(Debug)]
enum AirportType {
    Closed,
    Balloonport,
    Heliport,
    LargeAirport,
    MediumAirport,
    SmallAirport,
    SeaplaneBase,
}

impl FromStr for AirportType {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.trim().to_ascii_lowercase().as_str() {
            "closed" => Ok(AirportType::Closed),
            "balloonport" => Ok(AirportType::Balloonport),
            "heliport" => Ok(AirportType::Heliport),
            "large_airport" => Ok(AirportType::LargeAirport),
            "medium_airport" => Ok(AirportType::MediumAirport),
            "small_airport" => Ok(AirportType::SmallAirport),
            "seaplane_base" => Ok(AirportType::SeaplaneBase),
            _ => Err(format!("Unknown airport type: {}", str)),
        }
    }
}

fn read_name<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(name) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    Ok(Some(name.trim().to_string()))
}

fn read_municipality<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(municipality) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    Ok(Some(municipality.trim().to_string()))
}

fn read_iata_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(iata_code) = String::deserialize(deserializer) else {
		return Ok(None);
	};

    let err = || -> D::Error {
        de::Error::custom(format!("Not a valid IATA airport code: \"{}\"", iata_code))
    };

    const IATA_CODE_LENGTH: usize = 3;
    if iata_code.chars().count() != IATA_CODE_LENGTH {
        return Err(err());
    }

    if !iata_code.chars().all(char::is_alphanumeric) {
        return Err(err());
    }

    Ok(Some(iata_code))
}

fn read_continent<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(continent) = String::deserialize(deserializer) else {
		return Ok(None);
	};

    let continent = continent.trim().to_ascii_uppercase();

    const CONTINENTS: [&str; 7] = ["AF", "NA", "OC", "AN", "AS", "EU", "SA"];
    if !CONTINENTS.contains(&continent.as_str()) {
        return Err(de::Error::custom(format!(
            "Not valid continent: \"{}\"",
            continent
        )));
    }

    Ok(Some(continent))
}

fn read_iso_country<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(iso_country) = String::deserialize(deserializer) else {
		return Ok(None);
	};

    let iso_country = iso_country.trim().to_ascii_uppercase();

    let err = || -> D::Error {
        de::Error::custom(format!("Not valid ISO country code: \"{}\"", iso_country))
    };

    const ISO_COUNTRY_LENGTH: usize = 2;
    if iso_country.chars().count() != ISO_COUNTRY_LENGTH {
        return Err(err());
    }

    if !iso_country.chars().all(char::is_alphabetic) {
        return Err(err());
    }

    Ok(Some(iso_country))
}

fn read_iso_region<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(iso_region) = String::deserialize(deserializer) else {
		return Ok(None);
	};

    lazy_static::lazy_static! {
        static ref ISO_REGION_PATTERN: regex::Regex =
            regex::Regex::new(r"^[A-Z]{2}-[A-Z0-9]{1, 3}$")
                .expect("ISO_REGION_PATTERN is not a valid regexpr.");
    }

    if !ISO_REGION_PATTERN.is_match(&iso_region) {
        return Err(de::Error::custom(format!(
            "Not valid ISO region code: \"{}\"",
            iso_region
        )));
    }

    Ok(Some(iso_region))
}

fn read_coordinates<'de, D>(deserializer: D) -> Result<Option<(Angle, Angle)>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(coords_str) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    let mut coords = coords_str.trim().split(", ");

    let err = || -> D::Error {
        de::Error::custom(format!("Could not parse coordinates: \"{}\"", coords_str))
    };

    let lon = coords
        .next()
        .ok_or_else(err)?
        .parse::<f64>()
        .map_err(|_| err())?;

    let lat = coords
        .next()
        .ok_or_else(err)?
        .parse::<f64>()
        .map_err(|_| err())?;

    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Err(de::Error::custom(format!(
            "Not valid coordinates: \"({}, {})\"",
            lat, lon
        )));
    }

    Ok(Some((Angle::new::<degree>(lat), Angle::new::<degree>(lon))))
}

fn read_elevation_ft<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(elevation_ft_str) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    let err = || -> D::Error {
        de::Error::custom(format!(
            "Could not parse elevation_ft: \"{}\"",
            elevation_ft_str
        ))
    };

    let elevation_ft = elevation_ft_str.parse::<f64>().map_err(|_| err())?;

    if !(-10000.0..=40000.0).contains(&elevation_ft) {
        return Err(de::Error::custom(format!(
            "Not valid elevation: \"{}\"",
            elevation_ft
        )));
    }

    Ok(Some(Length::new::<foot>(elevation_ft)))
}

fn read_airport_type<'de, D>(deserializer: D) -> Result<Option<AirportType>, D::Error>
where
    D: Deserializer<'de>,
{
    let Ok(airport_type_str) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    let airport_type = airport_type_str
        .parse::<AirportType>()
        .map_err(de::Error::custom)?;

    Ok(Some(airport_type))
}
