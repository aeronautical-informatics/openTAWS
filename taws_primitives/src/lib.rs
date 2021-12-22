use ordered_float::OrderedFloat;
use uom::si::angle::degree;
use uom::si::f64::{Angle, Length};
use uom::si::length::{foot, kilometer};
use uom::num_traits::Pow;

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
    /// use crate::reference::PositionImpl;
    /// let p1 = PositionImpl::new(54, 10, 0);
    /// let p1 = PositionImpl::new(52, 8, 10);
    ///
    /// assert_eq!(p1.great_circle(p2), p2.great_circle(p1));
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

impl AsRef<Position> for Airport {
    fn as_ref(&self) -> &Position {
        &self.pos
    }
}