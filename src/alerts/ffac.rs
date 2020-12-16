use super::*;
use crate::prelude::*;

#[derive(Debug)]
pub struct FFAC {
    inhibited: bool,
    last_height: Length,
}

impl AlertSystem for FFAC {
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

impl Default for FFAC {
    fn default() -> Self {
        Self {
            inhibited: false,
            last_height: Length::new::<foot>(0.0),
        }
    }
}

// TODO is this sound usage?
impl std::panic::UnwindSafe for FFAC {}
