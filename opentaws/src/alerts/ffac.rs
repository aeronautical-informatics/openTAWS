use super::*;
use crate::prelude::*;

#[derive(Debug)]
pub struct Ffac {
    armed: bool,
    inhibited: bool,
    last_height: Length,
}

impl<'a> AlertSystem<'a> for Ffac {
    fn new(_config: &TawsConfig) -> Self {
        Self {
            armed: true,
            inhibited: false,
            last_height: Length::new::<foot>(0.0),
        }
    }

    arm_inhibit!();

    /// __Notes__:
    /// + Per the text in chapter 2.2.1.1.13.2 only "one of the following altitude callouts" must
    ///   be generated
    /// + MOPS_125 speaks of  non-precision approaches. However, per Note 3 of chapter
    ///   2.2.1.1.13.2, emitation of FFAC "is recommended for all approaches"
    fn process(&mut self, state: &AircraftState) -> Option<(AlertLevel, u8)> {
        let fivehundred = Length::new::<foot>(500.0);
        let mut result = None;

        // TODO Do we have to check against nearest runway?
        // TODO Is it possible to disarm the FFAC?

        if self.last_height >= fivehundred && state.altitude_ground < fivehundred {
            result = Some((AlertLevel::Annunciation, u8::default()));
        }

        self.last_height = state.altitude_ground;

        armed_return!(self, result)
    }
}
