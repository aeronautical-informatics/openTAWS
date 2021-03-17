use std::ops::{Add, Rem, Sub};

use arbitrary::{Arbitrary, Unstructured};
use rand::RngCore;

use uom::{
    num::Signed,
    si::{f64::*, length::foot, time::second, velocity::foot_per_minute},
};

use opentaws::prelude::*;

#[derive(Debug, Clone)]
struct AircraftStateWrapper(AircraftState);

impl<'a> Arbitrary<'a> for AircraftStateWrapper {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(AircraftStateWrapper(AircraftState {
            timestamp: Time::new::<second>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            altitude: Length::new::<foot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            altitude_ground: Length::new::<foot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            climb_rate: Velocity::new::<foot_per_minute>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            position_lat: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            position_lon: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            speed_ground: Velocity::new::<knot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            speed_air: Velocity::new::<knot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            heading: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            pitch: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            roll: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            steep_approach: u.arbitrary()?,
        }))
    }

    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (std::mem::size_of::<AircraftStateWrapper>(), None)
    }
}

// Parser magic
pub fn parse_alert<T: AsRef<str>>(from: &T) -> Alert {
    let mut input_word = from.as_ref().to_lowercase();
    input_word.retain(|c| !c.is_whitespace());
    match input_word.as_str() {
        "ffac" => Alert::Ffac,
        "flta" => Alert::Flta,
        "mode1" => Alert::Mode1,
        "mode2" => Alert::Mode2,
        "mode3" => Alert::Mode3,
        "mode4" => Alert::Mode4,
        "mode5" => Alert::Mode5,
        "pda" => Alert::Pda,
        _ => {
            panic!(
                "unable to convert {} into a variant of `Alert`",
                from.as_ref()
            );
        }
    }
}

pub fn parse_level<T: AsRef<str>>(from: &T) -> AlertLevel {
    let mut input_word = from.as_ref().to_lowercase();
    input_word.retain(|c| !c.is_whitespace());
    match input_word.as_str() {
        "warning" => AlertLevel::Warning,
        "caution" => AlertLevel::Caution,
        "annunciation" => AlertLevel::Annunciation,
        _ => {
            panic!(
                "unable to convert {} into a variant of `Alert`",
                from.as_ref()
            );
        }
    }
}

pub fn parse_alert_level<T: AsRef<str>>(from: &T) -> (Alert, AlertLevel) {
    let word_vec: Vec<_> = from.as_ref().rsplitn(2, ' ').collect();
    (parse_alert(&word_vec[1]), parse_level(&word_vec[0]))
}

// AircraftState generator
type Prng = rand_pcg::Mcg128Xsl64;
pub struct AircraftStateGenerator(pub Prng);

impl Default for AircraftStateGenerator {
    fn default() -> Self {
        Self(Prng::new(0xcafef00dd15ea5e5))
    }
}

impl Iterator for AircraftStateGenerator {
    type Item = AircraftState;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes_needed = AircraftStateWrapper::size_hint(0).0;
        let mut buf = Vec::with_capacity(bytes_needed);
        while buf.len() < bytes_needed {
            buf.extend_from_slice(&self.0.next_u64().to_le_bytes());
        }
        let mut u = Unstructured::new(&mut buf);

        Some(AircraftStateWrapper::arbitrary(&mut u).unwrap().0) // the unwrap is safe, we guarantee that enough bytes are available
    }
}

// for the lack of a better word
pub trait PressMould<T> {
    fn at_least(&mut self, value: &mut T, at_least: T);
    fn at_most(&mut self, value: &mut T, at_most: T);
    fn in_range(&mut self, value: &mut T, at_least: T, at_most: T);
    fn not_in_range(&mut self, value: &mut T, range_from: T, range_to: T);
}

// Stupid
pub struct BouncingClamp();

impl<T> PressMould<T> for BouncingClamp
where
    T: Copy
        + Clone
        + std::fmt::Debug
        + PartialOrd
        + Add<Output = T>
        + Rem<Output = T>
        + Sub<Output = T>
        + Abs,
{
    fn at_least(&mut self, value: &mut T, at_least: T) {
        if *value < at_least {
            *value = at_least + (at_least - *value)
        }
        assert!(*value >= at_least);
    }

    fn at_most(&mut self, value: &mut T, at_most: T) {
        if *value > at_most {
            *value = at_most - (*value - at_most)
        }
        assert!(*value <= at_most);
    }

    fn in_range(&mut self, value: &mut T, at_least: T, at_most: T) {
        assert!(at_least <= at_most);

        if at_least == at_most {
            *value = at_least;
            return;
        }

        let modulo = |a: T, b: T| ((a % b) + b) % b;

        let span = at_most - at_least;
        let bounced = (modulo(*value + span, span + span) - span).abs();
        *value = bounced + at_least;

        assert!(at_least <= *value && *value <= at_most);
    }

    fn not_in_range(&mut self, value: &mut T, at_most: T, at_least: T) {
        assert!(at_most <= at_least);
        if *value > at_most && *value < at_least {
            *value = *value + (at_least - at_most);
        }
        assert!(*value < at_least || at_most < *value);
    }
}

pub trait Abs: Sized {
    fn abs(self) -> Self;
}

impl Abs for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl<D: ?Sized, U: ?Sized, V: ?Sized> Abs for uom::si::Quantity<D, U, V>
where
    D: uom::si::Dimension,
    U: uom::si::Units<V>,
    V: uom::num_traits::Num + uom::Conversion<V> + Signed,
{
    fn abs(self) -> Self {
        self.abs()
    }
}
