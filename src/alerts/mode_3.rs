use super::*;

#[derive(Debug)]
pub struct Mode3();

impl AlertSystem for Mode3 {
    fn new(_config: &TAWSConfig) -> Self {
        Self()
    }

    fn is_armed(&self) -> bool {
        false
    }

    fn is_inhibited(&self) -> bool {
        unimplemented!()
    }

    fn inhibit(&mut self) {
        unimplemented!()
    }

    fn uninhibit(&mut self) {
        unimplemented!()
    }

    fn process(&mut self, _state: &AircraftState) -> Option<AlertLevel> {
        None
    }
}
