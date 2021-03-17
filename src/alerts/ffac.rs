use super::*;
use crate::prelude::*;

#[derive(Debug)]
pub struct Ffac {
    armed: bool,
    inhibited: bool,
    last_height: Length,
}

impl AlertSystem for Ffac {
    fn new(_config: &TawsConfig) -> Self {
        Self {
            armed: true,
            inhibited: false,
            last_height: Length::new::<foot>(0.0),
        }
    }

    arm_inhibit!();

    fn process(&mut self, state: &AircraftState) -> Option<AlertLevel> {
        let fivehundred = Length::new::<foot>(500.0);

        if self.last_height >= fivehundred && state.altitude_ground < fivehundred {
            return Some(AlertLevel::Annunciation);
        }

        // TODO check against runway elevation (MOPS_292)
        //

        None
    }
}
