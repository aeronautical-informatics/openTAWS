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

    fn is_armed(&self) -> bool {
        false
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

    fn process(&mut self, _state: &AircraftState) -> Option<AlertLevel> {
        None
    }
}
