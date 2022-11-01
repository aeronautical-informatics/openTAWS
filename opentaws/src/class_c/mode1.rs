use crate::{alerts::*, envelope::*, prelude::*};

use super::ClassC_Source;

use lazy_static::lazy_static;
use nalgebra::{Vector, Vector2};

#[derive(Clone, Debug)]
pub struct Mode1 {
    armed: bool,
    inhibited: bool,
}

impl Default for Mode1 {
    fn default() -> Self {
        Self {
            armed: true,
            inhibited: false,
        }
    }
}

impl Mode1 {
    const ALERT_SOURCE: ClassC_Source = ClassC_Source::Mode1;
    const ALERT_WARNING: <Self as TawsFunctionality>::Alert =
        <Self as TawsFunctionality>::Alert::new(Self::ALERT_SOURCE, AlertLevel::Warning);

    const ALERT_CAUTION: <Self as TawsFunctionality>::Alert =
        <Self as TawsFunctionality>::Alert::new(Self::ALERT_SOURCE, AlertLevel::Caution);
}

impl TawsFunctionality for Mode1 {
    type AlertSource = ClassC_Source;
    type Alert = Alert<Self::AlertSource>;

    fn alert_source(&self) -> Self::AlertSource {
        Self::ALERT_SOURCE
    }

    fn is_armed(&self) -> bool {
        self.armed
    }

    fn inhibit(&mut self) {
        self.inhibited = true;
    }

    fn uninhibit(&mut self) {
        self.inhibited = false;
    }

    fn is_inhibited(&self) -> bool {
        self.inhibited
    }

    fn process(
        &mut self,
        state: &NormalizedAircraftState,
    ) -> Result<Option<Self::Alert>, &'static dyn TawsError> {
        let rod = -state.climb_rate().get::<foot_per_minute>();
        let altitude_gnd = state.altitude_ground().get::<foot>();

        if !LIMITS.contains(Vector2::new(rod, altitude_gnd)) {
            let _x = 437;
        }

        let steep_approach = match state.situation() {
            Some(FlightSegment::Landing { steep_approach, .. }) => *steep_approach,
            _ => false, //Steep envelopes are more relaxed; assume steep envelope is not selected.
        };

        let result = if steep_approach {
            let caution = STEEP_CAUTION_ENVELOPE
                .contains(rod, altitude_gnd)?
                .then_some(Self::ALERT_CAUTION);
            let warning = STEEP_WARNING_ENVELOPE
                .contains(rod, altitude_gnd)?
                .then_some(Self::ALERT_WARNING);
            warning.or(caution)
        } else {
            let caution = CAUTION_ENVELOPE
                .contains(rod, altitude_gnd)?
                .then_some(Self::ALERT_CAUTION);
            let warning = WARNING_ENVELOPE
                .contains(rod, altitude_gnd)?
                .then_some(Self::ALERT_WARNING);
            warning.or(caution)
        };

        if self.inhibited {
            return Ok(None);
        }

        Ok(result)
    }
}

lazy_static! {
    static ref LIMITS: Rect = Rect::new(
        // Min/Max climb/descent rate: Mach 10 = +-675197.0 ft/min
        // Max altitude above terrain: 100km = 328084 ft
        Vector2::new(-680_000.0, 0.0),
        Vector2::new(680_000.0, 330000.0)
    );

    // Envelopes enlarged by d=10, to prevent floating pointing problems.
    static ref CAUTION_ENVELOPE: Envelope<5> = Envelope::new(
        *LIMITS,
        &[
            Vector2::new(680000.000,   90.000),
            Vector2::new(  1550.592,   96.610),
            Vector2::new(  2194.716,  638.490),
            Vector2::new(  5697.907, 2209.779),
            Vector2::new(680000.000, 2210.000)
        ]
    ).unwrap();

    static ref STEEP_CAUTION_ENVELOPE: Envelope<6> = Envelope::new(
        *LIMITS,
        &[
            Vector2::new(680000.000,  140.000),
            Vector2::new(  1788.787,  146.111),
            Vector2::new(  1937.782,  307.832),
            Vector2::new(  3228.810, 1087.080),
            Vector2::new(  6223.399, 2084.871),
            Vector2::new(680000.000, 2085.000)
        ]
    ).unwrap();

    static ref WARNING_ENVELOPE: Envelope<5> = Envelope::new(
        *LIMITS,
        &[
            Vector2::new(680000.000,   90.000),
            Vector2::new(  1590.564,   96.690),
            Vector2::new(  1845.772,  309.062),
            Vector2::new( 10099.010, 1967.951),
            Vector2::new(680000.000, 1968.000)
        ]
    ).unwrap();

    static ref STEEP_WARNING_ENVELOPE: Envelope<5> = Envelope::new(
        *LIMITS,
        &[
            Vector2::new(680000.000,  140.000),
            Vector2::new(  1898.814,  146.047),
            Vector2::new(  2045.157,  308.749),
            Vector2::new( 10299.010, 1967.951),
            Vector2::new(680000.000, 1968.000)
        ]
    ).unwrap();
}
