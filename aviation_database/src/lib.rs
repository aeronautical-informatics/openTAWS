mod consts;
/// Contains a reference implementation from the
pub mod reference;

// use opentaws::prelude::*;
use core::{
    fmt::Debug,
    iter::{self, Iterator},
};
use ordered_float::OrderedFloat;
use uom::{
    num_traits::Pow,
    si::{angle::degree, f64::*, length::foot, length::kilometer},
};

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

    /// Just a mockup, never returns anything
    fn runways(&self, _airport: &Airport) -> Self::RunwayIterator {
        iter::empty().into()
    }
    /*
    /// Find all Airports in a given perimeter
    fn airports_in_perimeter<P:Position>(&self, position: P, perimeter: Length)->
    */
}

/// Describes a positon in three dimensional room
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub lat: Angle,
    pub lon: Angle,
    pub alt: Length,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Airport {
    pub icao: [u8; 4],
    pub pos: Position,
}

impl Position {
    const MEAN_EARTH_RADIUS_KILOMETER: f64 = 6371.001;
    /// Converts the `Position` to cartesian coordinates. Nothing orientation is guaranteed, thus
    /// this may only be used for relative distances etc.
    pub fn cartesian(&self) -> [Length; 3] {
        let x = self.alt * self.lat.cos() * self.lon.cos();
        let y = self.alt * self.lat.cos() * self.lon.sin();
        let z = self.alt * self.lat.sin();
        [x, y, z]
    }

    /// Converts the `Position` to cartesian coordinates. Nothing orientation is guaranteed, thus
    /// this may only be used for relative distances etc.
    pub fn cartesian_f64(&self) -> [f64; 3] {
        let x = self.alt.get::<foot>()
            * self.lat.get::<degree>().cos()
            * self.lon.get::<degree>().cos();
        let y = self.alt.get::<foot>()
            * self.lat.get::<degree>().cos()
            * self.lon.get::<degree>().sin();
        let z = self.alt.get::<foot>() * self.lat.get::<degree>().sin();
        [x, y, z]
    }

    /// Get the relative distance of two points
    ///
    /// This can be used for comparing distances
    pub fn relative_distance(&self, other: &Position) -> RelativeDistance {
        RelativeDistance(
            self.cartesian()
                .iter()
                .zip(&other.cartesian())
                .fold(OrderedFloat(0.0), |r, (a, b)| {
                    r + (a.value - b.value).pow(2)
                }),
        )
    }

    /// As we have two different altitudes, this function ignores the altitude of `other` and
    /// calculates the great circle distance from `self` at the altitude of `self` to lat/lon of
    /// `other`
    ///
    /// This implies, that this operation __is not commutatitve__. E.g. the following code will
    /// panic:
    ///
    /// ```
    /// use aviation_database::Position;
    /// use uom::si::{
    ///     f64::{Angle, Length},
    ///     angle::degree,
    ///     length::meter,
    /// };
    ///
    /// let p1 = Position{
    ///     lat: Angle::new::<uom::si::angle::degree>(54.0),
    ///     lon: Angle::new::<uom::si::angle::degree>(10.0),
    ///     alt: Length::new::<uom::si::length::foot>(1000.0),
    /// };
    /// let p2 = Position{
    ///     lat: Angle::new::<uom::si::angle::degree>(52.0),
    ///     lon: Angle::new::<uom::si::angle::degree>(8.0),
    ///     alt: Length::new::<uom::si::length::foot>(1000.0),
    /// };
    ///
    /// let distance_delta = p1.great_circle(&p2) - p2.great_circle(&p1);
    /// assert!(distance_delta.abs() < Length::new::<uom::si::length::millimeter>(1.0));
    /// ```
    #[allow(non_snake_case)]
    pub fn great_circle(&self, other: &Position) -> Length {
        // borrows formula from https://en.wikipedia.org/wiki/Great-circle_distance
        let λ1 = self.lon;
        let λ2 = other.lon;
        let Δλ = λ1 - λ2;
        let Φ1 = self.lat;
        let Φ2 = other.lat;

        let a = Φ2.cos() * Δλ.sin();
        let b = Φ1.cos() * Φ2.sin() - Φ1.sin() * Φ2.cos() * Δλ.cos();
        let c = Φ1.sin() * Φ2.sin() + Φ1.cos() * Φ2.cos() * Δλ.cos();

        let radius = Length::new::<kilometer>(Self::MEAN_EARTH_RADIUS_KILOMETER) + self.alt;

        radius * ((a * a + b * b) / c).sqrt().atan()
    }
}

/// Distance between two points
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct RelativeDistance(OrderedFloat<f64>);

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

impl AsRef<Position> for Airport {
    fn as_ref(&self) -> &Position {
        &self.pos
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
