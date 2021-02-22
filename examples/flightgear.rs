//! This demo illustrates how the openTAWS system can be integrated with Flightgear

use std::{env, error::Error, time::Instant};

use futures::prelude::*;

use async_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};
use uom::si::velocity::foot_per_second;

use opentaws::prelude::*;

#[derive(Serialize)]
struct FlightgearCommand {
    command: String,
    node: String,
}

/// Yields AircraftStates from a Flightgear http/json connection
///
/// # Arguments
/// `base_uri` - The base URI of the Flightgear http interface. Something like `localhost:5400`.
async fn new_flightgear_stream(
    base_uri: &str,
) -> Result<impl Stream<Item = Result<PropertyTreeLeaf, Box<dyn Error>>>, Box<dyn Error>> {
    let url = format!("ws://{}/PropertyListener", base_uri);
    let (mut stream, _) = async_tungstenite::async_std::connect_async(url).await?;

    for node in KEYS {
        let sub = FlightgearCommand {
            command: "addListener".to_string(),
            node: node.to_string(),
        };
        stream
            .send(Message::Binary(serde_json::to_vec(&sub)?))
            .await?;
    }

    Ok(stream
        .map(|msg| -> Result<_, Box<dyn Error>> { Ok(serde_json::from_slice(&msg?.into_data())?) }))
}

#[derive(Deserialize)]
struct PropertyTreeLeaf {
    pub path: String,
    pub ts: f64,
    pub value: f64,
}

const KEYS: &'static [&'static str] = &[
    "/velocities/groundspeed-kt",
    "/velocities/vertical-speed-fps",
    "/position/longitude-deg",
    "/position/latitude-deg",
    "/position/altitude-ft",
    "/position/altitude-agl-ft",
    "/orientation/pitch-deg",
    "/orientation/roll-deg",
    "/orientation/heading-deg",
];

const USAGE: &'static str = "usage: <Flightgear base url>";

// http://localhost:5400/json/velocities?i=y&t=y&d=3

fn main() -> Result<(), Box<dyn Error>> {
    smol::block_on(async {
        let args: Vec<String> = env::args().collect();
        let base_uri = args.get(1).expect(USAGE);

        let mut taws = Taws::new(Default::default());
        let mut fg_stream = new_flightgear_stream(base_uri.as_str()).await?;
        let mut frames: u128 = 0;

        let mut aircraft_state = AircraftState::default();

        loop {
            let leaf = fg_stream.next().await.unwrap()?;
            let now = Instant::now();
            let ts = Time::new::<second>(leaf.ts);

            // Next frame begins
            if ts > aircraft_state.timestamp {
                let alert_state = taws.process(&aircraft_state);
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                frames += 1;
                println!(
                    "Processed frame: {}, time consumed: {:?}",
                    frames,
                    now.elapsed(),
                );
                println!("{}\n{:#?}", aircraft_state, alert_state);
            }
            aircraft_state.timestamp = ts;

            match leaf.path.as_str() {
                "/velocities/groundspeed-kt" => {
                    aircraft_state.speed_ground = Velocity::new::<knot>(leaf.value)
                }
                "/velocities/vertical-speed-fps" => {
                    aircraft_state.climb_rate = Velocity::new::<foot_per_second>(leaf.value)
                }
                "/position/longitude-deg" => {
                    aircraft_state.position_lon = Angle::new::<degree>(leaf.value)
                }
                "/position/latitude-deg" => {
                    aircraft_state.position_lat = Angle::new::<degree>(leaf.value)
                }
                "/position/altitude-ft" => {
                    aircraft_state.altitude = Length::new::<foot>(leaf.value)
                }
                "/position/altitude-agl-ft" => {
                    aircraft_state.altitude_ground = Length::new::<foot>(leaf.value)
                }
                "/orientation/pitch-deg" => aircraft_state.pitch = Angle::new::<degree>(leaf.value),
                "/orientation/roll-deg" => aircraft_state.roll = Angle::new::<degree>(leaf.value),
                "/orientation/heading-deg" => {
                    aircraft_state.heading = Angle::new::<degree>(leaf.value)
                }
                _ => {}
            }
        }
    })
}
