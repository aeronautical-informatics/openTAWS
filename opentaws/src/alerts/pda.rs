use super::*;
use crate::envelope::Envelope;
use crate::prelude::*;
use uom::si::length::nautical_mile;

#[derive(Debug)]
pub struct Pda<'a> {
    armed: bool,
    inhibited: bool,
    taws_config: &'a TawsConfig<'a>,
}

impl<'a> AlertSystem<'a> for Pda<'a> {
    fn new(config: &'a TawsConfig<'a>) -> Self {
        Self {
            armed: false,
            inhibited: false,
            taws_config: config,
        }
    }

    arm_inhibit!();

    fn process(&mut self, state: &AircraftState) -> Option<(AlertLevel, u8)> {
        let position = state.position();
        let nearest_airport = self.taws_config.terrain_server.nearest_airport(&position);
        let distance = position.great_circle(&nearest_airport.pos);
        let distance_nm = distance.get::<nautical_mile>();

        // MOPS_263 requires PDA to be armed within 5NM of an airport
        // But since it may be armed beyond that, we arm/disarm it with a
        // distance of 8NM
        self.armed = distance_nm <= 8.0;

        // In 2.2.3.1.7.2 some considerations are presented for reducing nuisance alerts
        // All of them are satisfied by our CAUTION_ENVELOPE
        let result = if self.armed
            && CAUTION_ENVELOPE.contains(distance_nm, state.altitude_ground.get::<foot>())
        {
            Some((AlertLevel::Caution, u8::default()))
        } else {
            None
        };

        armed_return!(self, result)
    }
}

/// CAUTION Envelope for PDA
/// X-Axis is Distance to Airport in nautical miles
/// Y-Axis is Height above Terrain in foot
const CAUTION_ENVELOPE: Envelope<f64, f64, f64, 5> = Envelope::new([
    (0.9, 5.0),
    (0.9, 85.0),
    (1.8, 155.0),
    (2.3, 175.0),
    (2.4, 175.0),
]);
