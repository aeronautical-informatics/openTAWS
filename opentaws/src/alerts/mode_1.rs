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

impl<'a> AlertSystem<'a> for Mode1 {
    fn new(_config: &TawsConfig) -> Self {
        Self {
            armed: true,
            inhibited: false,
        }
    }

    //TODO add real priority
    fn process(&mut self, state: &AircraftState) -> Option<(AlertLevel, u8)> {
        let altitude = state.altitude_ground.get::<foot>();
        let rod = -state.climb_rate.get::<foot_per_minute>();

        let result = match state.steep_approach {
            true if WARNING_ENVELOPE_STEEP_APPROACH.contains(rod, altitude) => {
                Some((AlertLevel::Warning, u8::default()))
            }
            true if CAUTION_ENVELOPE_STEEP_APPROACH.contains(rod, altitude) => {
                Some((AlertLevel::Caution, u8::default()))
            }
            false if WARNING_ENVELOPE.contains(rod, altitude) => {
                Some((AlertLevel::Warning, u8::default()))
            }
            false if CAUTION_ENVELOPE.contains(rod, altitude) => {
                Some((AlertLevel::Caution, u8::default()))
            }

            //self.caution_envelope
            _ => None,
        };

        armed_return!(self, result)
    }

    arm_inhibit!();
}

lazy_static::lazy_static! {
    static ref CAUTION_ENVELOPE: Envelope<5> = Envelope::new([
        (1200.0, 11.0),
        (2550.0, 1550.0),
        (4800.0, 2900.0),
        (10000.0, 4000.0),
        (10001.0, 4000.0),
    ])
    .unwrap();

        static ref CAUTION_ENVELOPE_STEEP_APPROACH: Envelope<5> = Envelope::new([
        (1350.0, 11.0),
        (2700.0, 1550.0),
        (5300.0, 2900.0),
        (10000.0, 4000.0),
        (10001.0, 4000.0),
    ])
    .unwrap();

    static ref WARNING_ENVELOPE: Envelope<5> = Envelope::new([
        (1400.0, 11.0),
        (2500.0, 1300.0),
        (7500.0, 2500.0),
        (11000.0, 3000.0),
        (11001.0, 3000.0),
    ])
    .unwrap();

    static ref WARNING_ENVELOPE_STEEP_APPROACH: Envelope<5> = Envelope::new([
        (1550.0, 11.0),
        (2650.0, 1300.0),
        (8000.0, 2500.0),
        (11000.0, 3000.0),
        (11001.0, 3000.0),
    ])
    .unwrap();
}
