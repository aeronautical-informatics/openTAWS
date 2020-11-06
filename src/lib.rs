#![deny(unsafe_code)]

use std::collections::HashMap;
use std::panic::UnwindSafe;

pub mod alerts;
mod envelope;
pub mod types;
use alerts::*;
use types::*;

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
    fn push(&mut self, position: &AircraftState) -> AlertState;
}

pub trait TAWSFunctionality: AircraftStateReceiver {
    /// Returns whether this alarm is armed.
    ///
    /// Arming refers to the automatic switching on of a function by
    /// the Equipment (DO-367 Chapter 1.9).
    fn is_armed(&self) -> bool;

    /// Dismiss this alert
    fn inhibit(&mut self);

    /// Enable this alert
    fn uninhibit(&mut self);

    /// Returns whether this alarm is inhibited
    fn is_inhibited(&self) -> bool;
}

pub struct TAWS {
    armed: bool,
    functions: HashMap<String, Box<dyn TAWSFunctionality + UnwindSafe>>,
}

impl TAWS {
    pub fn new(config: TAWSConfig) -> Self {
        let mut functions = HashMap::new();

        Self {
            armed: true,
            functions,
        }
    }

    pub fn is_armed(&self) -> bool {
        self.armed
    }

    pub fn function_is_armed(&self, function: &str) -> bool {
        // function identified by string?
        todo!();
    }
}

impl AircraftStateReceiver for TAWS {
    fn push(&mut self, position: &AircraftState) -> alerts::AlertState {
        let mut alert_state = alerts::AlertState::default();

        for f in &mut self.functions.values_mut() {
            let mut new_alert_state = f.push(position);
            new_alert_state.alerts.drain().for_each(|a| {
                alert_state.alerts.insert(a);
            });
            new_alert_state.nuisance_alerts.drain().for_each(|a| {
                alert_state.nuisance_alerts.insert(a);
            });
        }

        alert_state
    }
}
