use super::*;
use crate::prelude::*;

#[derive(Debug)]
pub struct FFAC {
    inhibited: bool,
    last_height: Length,
}

impl AlertSystem for FFAC {
    fn new(_config: &TAWSConfig) -> Self {
        Self {
            inhibited: false,
            last_height: Length::new::<foot>(0.0),
        }
    }

    fn is_armed(&self) -> bool {
        true
    }

    fn is_inhibited(&self) -> bool {
        self.inhibited
    }

    fn inhibit(&mut self) {
        self.inhibited = true;
    }

    fn uninhibit(&mut self) {
        self.inhibited = false;
    }

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
