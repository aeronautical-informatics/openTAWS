use core::convert::{From,Into};

use crate::prelude::*;


pub trait TerrainServer {
    fn elevation<T: Into<Position>>(&self, position:T)->Length;
    fn nearest_runway<T: Into<Position>>(&self, position:T)->Runway;
}

pub struct Position {
    latitude: Angle,
    longitude: Angle,
    altitude_sea: Length,
}

impl From<&AircraftState> for Position {
    fn from(aircraft_state: AircraftState)->Self{
        Position{
            longitude: aircraft_state.longitude,
            latitude: aircraft_state.latitude,
            altitude_sea: aircraft_state.altitude_sea,
        }
    }
}

pub struct Runway {
    location: Position,
    length: Length,
    name: String,
    azimuth: Angle,
}