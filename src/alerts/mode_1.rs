use uom::si::{length::foot, velocity::foot_per_minute};

use crate::envelope::Envelope;
use crate::types::*;

use super::*;

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

impl AlertSystem for Mode1 {
    fn new(_config: &TawsConfig) -> Self {
        Self {
            armed: true,
            inhibited: false,
        }
    }

    fn process(&mut self, state: &AircraftState) -> Option<AlertLevel> {
        let altitude = state.altitude_ground.get::<foot>();
        let rod = -state.climb_rate.get::<foot_per_minute>();

        match state.steep_approach {
            true if WARNING_ENVELOPE_STEEP_APPROACH.contains(rod, altitude) => {
                Some(AlertLevel::Warning)
            }
            true if CAUTION_ENVELOPE_STEEP_APPROACH.contains(rod, altitude) => {
                Some(AlertLevel::Caution)
            }
            false if WARNING_ENVELOPE.contains(rod, altitude) => Some(AlertLevel::Warning),
            false if CAUTION_ENVELOPE.contains(rod, altitude) => Some(AlertLevel::Caution),

            //self.caution_envelope
            _ => None,
        }
    }

    arm_inhibit!();
}

lazy_static::lazy_static! {

static ref CAUTION_ENVELOPE: Envelope<4> = Envelope::new([
            (1560.0, 100.0),
            (2200.0, 630.0),
            (5700.0, 2200.0),
            (5701.0, 2200.0),
        ])
        .unwrap();

        static ref CAUTION_ENVELOPE_STEEP_APPROACH: Envelope<5> = Envelope::new([
            (1798.0, 150.0),
            (1944.0, 300.0),
            (3233.0, 1078.0),
            (6226.0, 2075.0),
            (6227.0, 2075.0),
        ])
        .unwrap();

        static ref WARNING_ENVELOPE: Envelope<4> = Envelope::new([
            (1600.0, 100.0),
            (1850.0, 300.0),
            (10100.0, 1958.0),
            (10101.0, 1958.0),
        ])
        .unwrap();

        static ref WARNING_ENVELOPE_STEEP_APPROACH: Envelope<4> = Envelope::new([
            (1908.0, 150.0),
            (2050.0, 300.0),
            (10300.0, 1958.0),
            (10301.0, 1958.0),
        ])
        .unwrap();
}
