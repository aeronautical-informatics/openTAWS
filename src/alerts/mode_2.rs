use super::*;

#[derive(Debug)]
pub struct Mode2 {
    armed: bool,
    inhibited: bool,
}

impl AlertSystem for Mode2 {
    fn new(_config: &TAWSConfig) -> Self {
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
