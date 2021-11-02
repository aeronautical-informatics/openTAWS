use super::*;

#[derive(Debug)]
pub struct Mode5 {
    armed: bool,
    inhibited: bool,
}

impl<'a> AlertSystem<'a> for Mode5 {
    fn new(_config: &TawsConfig) -> Self {
        Self {
            armed: false,
            inhibited: false,
        }
    }

    arm_inhibit!();

    fn process(&mut self, _state: &AircraftState) -> Option<(AlertLevel, u8)> {
        armed_return!(self, None)
    }
}
