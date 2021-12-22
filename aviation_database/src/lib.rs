mod consts;
/// Contains a reference implementation from the
pub mod reference;

// use opentaws::prelude::*;
use core::{
    fmt::Debug,
    iter,
};
use uom::{
    si::{angle::degree, f64::*, length::foot},
};
use taws_primitives::{Airport, Position};

use crate::consts::AIRPORTS;

/// Describes the interface to a TerrainDatabase
///
/// Each implementation of this type can be use as TerrainDatabase, providing elevation data for a
/// given `Position`.
pub trait TerrainDatabase {
    /// Estimate elevation at a specific position
    fn elevation_at(&self, position: Position) -> Length;
}

/// Describes the interface of a AirportDatabase
///
/// Each implementation of this type can be used as AirportDatabase, providing information about
/// `Airports` and `Runways`
pub trait AirportDatabase: Send + Sync + Debug {
    type RunwayIterator: core::iter::Iterator<Item = Runway>
        + From<core::iter::Empty<Runway>>
        + Send
        + Sync;

    /// Find the airport nearest to a given `Position`
    fn nearest_airport(&self, position: &Position) -> Airport {
        // For ~50000 Airports
        // Linear Search took 7ms
        // KD-Tree Search took 29µs
        // CPU: Ryzen 5 5600X

        // Old slow Linear Search
        //AIRPORTS.nodes
        //    .iter()
        //    .map(|n| Airport::from(n.payload()))
        //    .map(|a| (a.clone(), a.pos.relative_distance(position)))
        //    .reduce(|(c_a, c_d), (n_a, n_d)| match c_d.cmp(&n_d) {
        //        Ordering::Greater => (n_a, n_d),
        //        _ => (c_a, c_d),
        //    })
        //    .unwrap()
        //    .0;

        // New fast KD-Tree Search
        Airport::from(AIRPORTS.search(&position.cartesian_f64()).payload())
    }

    fn runways(&self, airport: &Airport) -> Self::RunwayIterator {
        iter::empty().into()
    }
    /*
    /// Find all Airports in a given perimeter
    fn airports_in_perimeter<P:Position>(&self, position: P, perimeter: Length)->
    */
}

pub struct AirportEntry {
    pub icao: [u8; 4],
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
}

impl From<&AirportEntry> for Position {
    fn from(entry: &AirportEntry) -> Self {
        Self {
            lat: Angle::new::<degree>(entry.lat),
            lon: Angle::new::<degree>(entry.lon),
            alt: Length::new::<foot>(entry.alt),
        }
    }
}

impl From<&AirportEntry> for Airport {
    fn from(a: &AirportEntry) -> Self {
        Airport {
            icao: a.icao,
            pos: a.into(),
        }
    }
}

impl From<AirportEntry> for Position {
    fn from(entry: AirportEntry) -> Self {
        Self {
            lat: Angle::new::<degree>(entry.lat),
            lon: Angle::new::<degree>(entry.lon),
            alt: Length::new::<foot>(entry.alt),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Runway();
