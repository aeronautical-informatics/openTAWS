use super::*;
use uom::num::Zero;
use uom::si::f64::Length;

use crate::envelope::Envelope;
use crate::prelude::*;

#[derive(Debug)]
pub struct Mode3 {
    armed: bool,
    inhibited: bool,
    max_altitude_ground: Length,
}

impl<'a> AlertSystem<'a> for Mode3 {
    fn new(_config: &TawsConfig) -> Self {
        Self {
            armed: false,
            inhibited: false,
            max_altitude_ground: Length::zero(),
        }
    }

    arm_inhibit!();

    fn process(&mut self, state: &AircraftState) -> Option<(AlertLevel, u8)> {
        let armed = state.go_around || state.take_off;
        if let (false, true) = (self.armed, armed) {
            self.max_altitude_ground = state.altitude_ground
        }
        self.armed = armed;

        let mut result = None;
        if armed {
            if self.max_altitude_ground < state.altitude_ground {
                self.max_altitude_ground = state.altitude_ground;
            }
            let alt_loss = (self.max_altitude_ground - state.altitude_ground).get::<foot>();
            let altitude = state.altitude_ground.get::<foot>();
            let rod = -state.climb_rate.get::<foot_per_minute>();

            if CAUTION_ENVELOPE_METHODE_1.contains(rod, altitude)
                || CAUTION_ENVELOPE_METHODE_2.contains(alt_loss, altitude)
            {
                result = Some((AlertLevel::Caution, u8::default()));
            }
        };

        armed_return!(self, result)
    }
}

lazy_static::lazy_static! {
    /// CAUTION Envelope for Methode 1
    /// X-Axis is Rate of Decent in foot per minute
    /// Y-Axis is Height above Terrain in foot
    static ref CAUTION_ENVELOPE_METHODE_1: Envelope<3> = Envelope::new([
        (125.0, 50.0),
        (480.0, 680.0),
        (481.0, 680.0),
    ])
    .unwrap();

    /// CAUTION Envelope for Methode 2
    /// X-Axis is Altitude Loss in foot
    /// Y-Axis is Height above Terrain in foot
    static ref CAUTION_ENVELOPE_METHODE_2: Envelope<3> = Envelope::new([
        (11.0, 35.0),
        (104.0, 1100.0),
        (104.0, 1100.0),
    ])
    .unwrap();
}
