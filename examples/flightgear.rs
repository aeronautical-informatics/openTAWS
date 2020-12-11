//! This demo illustrates how the openTAWS system can be integrated with Flightgear

use std::env;
use std::error::Error;
use std::io::{self, ErrorKind};
use std::time::Duration;

use tokio::{
    prelude::*,
    time::sleep,
};
use futures::prelude::*;

use opentaws::prelude::*;
use reqwest::{Client, Url};
use serde::Deserialize;
use uom::si::{angle::degree, length::foot, time::second, velocity::knot};

struct FlightgearConnection {
    base_url: Url,
    client: Client,
    //leafs: Vec<PropertyTreeLeaf>,
}

/// Yields AircrafStates from a Flightgear http/json connection
impl FlightgearConnection {
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            client: Client::new(),
            //leafs: KEYS.iter().map(|key| PropertyTreeLeaf{path:key.to_string(), ts:0.0, value:0.0}).collect()
        }
    }

    pub async fn poll(&self) -> Result<AircraftState, Box<dyn Error>> {
        let response_futures: Result<Vec<_>, Box<dyn Error>> = KEYS
            .iter()
            .map(|key| {
                let url = self.generate_url(key)?;
                let req = self.client.get(url).build()?;
                Ok(self.client.execute(req))
            })
            .collect();

        let responses: Vec<Result<_, _>> = future::join_all(response_futures?).await;

        let mut aircraft_state = AircraftState::default();
        let mut timestamp_avg: f64 = 0.0;

        for response in responses {
            let leaf: PropertyTreeLeaf = response?.json().await?;
            timestamp_avg += leaf.ts / KEYS.len() as f64;
            match leaf.path.as_str() {
                "/velocities/groundspeed-kt" => {
                    aircraft_state.speed = Velocity::new::<knot>(leaf.value)
                }
                "/position/longitude-deg" => {
                    aircraft_state.position_lon = Angle::new::<degree>(leaf.value)
                }
                "/position/latitude-deg" => {
                    aircraft_state.position_lat = Angle::new::<degree>(leaf.value)
                }
                "/position/altitude-ft" => {
                    aircraft_state.altitude_sea = Length::new::<foot>(leaf.value)
                }
                "/position/altitude-agl-ft" => {
                    aircraft_state.altitude_ground = Length::new::<foot>(leaf.value)
                }
                "/orientation/pitch-deg" => {
                    aircraft_state.attitude.pitch = Angle::new::<degree>(leaf.value)
                }
                "/orientation/roll-deg" => {
                    aircraft_state.attitude.roll = Angle::new::<degree>(leaf.value)
                }
                "/orientation/heading-deg" => {
                    aircraft_state.heading = Angle::new::<degree>(leaf.value)
                }
                _ => {
                    return Err(Box::new(io::Error::new(
                        ErrorKind::InvalidData,
                        "received an unknown path in property tree leaf",
                    )))
                }
            }
        }

        aircraft_state.timestamp = Time::new::<second>(timestamp_avg);

        Ok(aircraft_state)
    }

    fn generate_url(&self, key: &str) -> Result<Url, Box<dyn Error>> {
        let mut url = self.base_url.join(&format!("json/{}", key))?;
        url.set_query(Some("t=y"));

        Ok(url)
    }
}

#[derive(Deserialize)]
struct PropertyTreeLeaf {
    pub path: String,
    pub ts: f64,
    pub value: f64,
}

const KEYS: &'static [&'static str] = &[
    "/velocities/groundspeed-kt",
    "/position/longitude-deg",
    "/position/latitude-deg",
    "/position/altitude-ft",
    "/position/altitude-agl-ft",
    "/orientation/pitch-deg",
    "/orientation/roll-deg",
    "/orientation/heading-deg",
];

const USAGE: &'static str = "usage: <Flightgear base url> <poll rate in Hz>";

// http://localhost:5400/json/velocities?i=y&t=y&d=3
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let base_url: Url = args
        .get(1)
        .expect(USAGE)
        .parse()
        .expect("unable to parse $1 as url");

    let mut taws = TAWS::new(Default::default());
    let fgconn = FlightgearConnection::new(base_url);
    let mut frames: u128 = 0;

    let frequency: f64 = args
        .get(2)
        .expect(USAGE)
        .parse()
        .expect("unable to parse $2 as f64");

    loop {
        let new_aircraft_state = fgconn.poll().await?;

        let alert_state = taws.process(&new_aircraft_state);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        frames += 1;
        println!("Processed frame: {}", frames);
        println!("{:#?}", alert_state);

        sleep(Duration::from_secs_f64(1.0 / frequency)).await;
    }
}
