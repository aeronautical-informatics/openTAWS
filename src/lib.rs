#![deny(unsafe_code)]

use std::collections::HashMap;
use std::panic::UnwindSafe;

mod alerts;
mod envelope;
mod types;
pub use alerts::*;
pub use types::*;

#[cfg(feature = "capi")]
pub mod capi;

#[cfg(feature = "wasi")]
pub mod wasi;

// How many alarms at max?
// How do we prioritize?
// Alarms Type, which contains all alarms but
//
// An array of all alams
// An array of the important alarms
pub trait AircraftStateReceiver {
    /// Push new attitude data
    fn push(&mut self, state: &AircraftState) -> AlertState;
}

pub struct TAWS {
    armed: bool,
    functions: HashMap<Functionality, Box<dyn FunctionalityProcessor + UnwindSafe>>,
}

impl TAWS {
    pub fn new(config: TAWSConfig) -> Self {
        use alerts::*;
        let mut functions = HashMap::new();
        let b: Box<dyn FunctionalityProcessor + UnwindSafe> = Box::new(mode_1::Mode1::default());
        functions.insert(Functionality::Mode1, b);

        Self {
            armed: true,
            functions,
        }
    }

    pub fn is_armed(&self) -> bool {
        self.armed
    }

    pub fn function_is_armed(&self, function: &Functionality) -> bool {
        self.functions.get(function).unwrap().is_armed()
    }

    pub fn function_is_inhibited(&self, function: &Functionality) -> bool {
        self.functions.get(function).unwrap().is_inhibited()
    }

    pub fn function_inhibit(&mut self, function: &Functionality) {
        self.functions.get_mut(function).unwrap().inhibit()
    }

    pub fn function_uninhibit(&mut self, function: &Functionality) {
        self.functions.get_mut(function).unwrap().uninhibit()
    }
}

impl AircraftStateReceiver for TAWS {
    fn push(&mut self, state: &AircraftState) -> alerts::AlertState {
        let mut alert_state = alerts::AlertState::default();

        let alerts = self
            .functions
            .iter_mut()
            .filter_map(|(_k, v)| v.process(state));

        alert_state.update(alerts);
        alert_state
    }
}
