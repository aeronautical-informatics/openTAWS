#![deny(unsafe_code)]

pub mod types;
pub mod alarms;

pub trait AircraftStateReceiver {
    /// Push new attitude data
    fn push(&mut self, position: &types::LonLatAlt) -> Vec<alarms::Report>;
}

pub trait TAWS: AircraftStateReceiver {
    fn new() -> Self;
    fn is_armed(&self) -> bool;
}

pub trait Alarm: AircraftStateReceiver {
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

use std::collections::HashMap;

struct TawsState {
    alarms: HashMap<alarms::Report, Box<dyn Alarm>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
