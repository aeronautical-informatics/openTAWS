use crate::envelope::Envelope;
use crate::types::*;
use crate::{alerts::AlertState, AircraftStateReceiver, TAWSFunctionality};

pub struct Mode1 {
    caution_envelope: Envelope,
    caution_envelope_steep_approach: Envelope,

    warning_envelope: Envelope,
    warning_envelope_steep_approach: Envelope,
    inhibited: bool,
}

impl Default for Mode1 {
    fn default() -> Self {
        let caution_envelope = Envelope::new(vec![
            (1560.0, 100.0),
            (2200.0, 630.0),
            (5700.0, 2200.0),
            (5701.0, 2200.0),
        ])
        .unwrap();

        let caution_envelope_steep_approach = Envelope::new(vec![
            (1798.0, 150.0),
            (1944.0, 300.0),
            (3233.0, 1078.0),
            (6226.0, 2075.0),
            (6227.0, 2075.0),
        ])
        .unwrap();

        let warning_envelope = Envelope::new(vec![
            (1600.0, 100.0),
            (1850.0, 300.0),
            (10100.0, 1958.0),
            (10101.0, 1958.0),
        ])
        .unwrap();

        let warning_envelope_steep_approach = Envelope::new(vec![
            (1908.0, 150.0),
            (2050.0, 300.0),
            (10300.0, 1958.0),
            (10301.0, 1958.0),
        ])
        .unwrap();

        Self {
            caution_envelope,
            caution_envelope_steep_approach,
            warning_envelope,
            warning_envelope_steep_approach,
            inhibited: false,
        }
    }
}

impl AircraftStateReceiver for Mode1 {
    fn push(&mut self, state: &AircraftState) -> AlertState {
        todo!();
    }
}

impl TAWSFunctionality for Mode1 {
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
}
