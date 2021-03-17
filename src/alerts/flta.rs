use super::*;

#[derive(Debug)]
pub struct Flta {
    armed: bool,
    inhibited: bool,
}

impl AlertSystem for Flta {
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
