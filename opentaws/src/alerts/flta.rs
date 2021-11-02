use super::*;

#[derive(Debug)]
pub struct Flta {
    armed: bool,
    inhibited: bool,
}

impl<'a> AlertSystem<'a> for Flta {
    fn new(_config: &'a TawsConfig) -> Self {
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
