use super::*;

#[derive(Debug)]
pub struct Pda {
    armed: bool,
    inhibited: bool,
}

impl AlertSystem for Pda {
    fn new(_config: &TawsConfig) -> Self {
        Self {
            armed: false,
            inhibited: false,
        }
    }

    arm_inhibit!();

    fn process(&mut self, _state: &AircraftState) -> Option<AlertLevel> {
        None
    }
}
