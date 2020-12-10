//! This is a proof of concept TAWS as described in DO-367. It is not even close to fulfilling
//! DO-367 C, the simplest TAWS class. It exists to learn about using BDD (Cucumber & Gherkin in
//! particular) for implementing avionic.
//!
//! # Using openTAWS
//!
//! Currently it is only possible to use openTAWS from Rust. We've looked briefly into WASM-WASI
//! and C ABI as addiotional targets, but this did not lead anywehre usable _so far_. We are very
//! open to suggestions, so please open an issue if you have some feedback.

#![deny(unsafe_code)]

use prelude::*;
use std::collections::HashMap;
use std::panic::UnwindSafe;

mod alerts;
mod envelope;
pub mod prelude;
mod types;


pub use alerts::{Alert, AlertLevel, AlertState};
pub use types::*;

/// Represents one instance of a TAWS
#[derive(Debug)]
pub struct TAWS {
    /// `true` if the TAWS is armed
    ///
    /// There is no specific condition for changing this to `false`.
    pub armed: bool,
    functions: HashMap<Alert, Box<dyn AlertSystem + UnwindSafe>>,
    config: TAWSConfig,
}

impl Clone for TAWS {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl TAWS {
    /// Create a new instance of `TAWS`
    ///  
    /// # Arguments
    ///
    /// * `config` - The configuration which this TAWS instance shall use
    ///
    /// # Example
    ///
    /// ```
    /// use opentaws::prelude::*;
    ///
    /// let config = TAWSConfig::default();
    /// let taws = TAWS::new(config);
    /// ```
    pub fn new(config: TAWSConfig) -> Self {
        use alerts::*;

        let mut functions = HashMap::new();
        let b: Box<dyn AlertSystem + UnwindSafe> = Box::new(Mode1::default());
        functions.insert(Alert::Mode1, b);

        //functions.insert(Alert::Mode2,  Box::new(Mode2()));
        //functions.insert(Alert::Mode3,  Box::new(Mode3()));
        //functions.insert(Alert::Mode4,  Box::new(Mode4()));
        //functions.insert(Alert::Mode5,  Box::new(Mode5()));
        //functions.insert(Alert::FLTA,  Box::new(FLTA()));
        //functions.insert(Alert::PDA,  Box::new(PDA()));
        //functions.insert(Alert::FFAC,  Box::new(FFAC()));

        Self {
            armed: true,
            functions,
            config,
        }
    }

    /// Returns `true` if the alert system is armed
    ///
    /// # Arguments
    ///
    /// * `alert_system` - The alert system whiches armed state shall be checked
    ///
    /// # Example
    ///
    /// ```
    /// # use opentaws::prelude::*;
    /// # let config = TAWSConfig::default();
    /// # let taws = TAWS::new(config);
    /// if taws.is_armed(Alert::Mode1) {
    ///     // ...
    /// }
    /// ```
    pub fn is_armed(&self, alert_system: Alert) -> bool {
        self.functions.get(&alert_system).unwrap().is_armed()
    }

    /// Returns `true` if the alert system is inhibited
    ///
    /// # Arguments
    ///
    /// * `alert_system` - The alert system whiches inhibited state shall be checked
    ///
    /// # Example
    ///
    /// ```
    /// # use opentaws::prelude::*;
    /// # let config = TAWSConfig::default();
    /// # let taws = TAWS::new(config);
    /// if taws.is_inhibited(Alert::Mode1) {
    ///     // ...
    /// }
    /// ```

    pub fn is_inhibited(&self, alert_system: Alert) -> bool {
        self.functions.get(&alert_system).unwrap().is_inhibited()
    }

    /// Inhibit a specific alert system
    ///
    /// # Arguments
    ///
    /// * `alert_system` - The alert system which shall be inhibited
    ///
    /// # Example
    ///
    /// ```
    /// # use opentaws::prelude::*;
    /// # let config = TAWSConfig::default();
    /// # let mut taws = TAWS::new(config);
    /// taws.inhibit(Alert::Mode1);
    ///
    /// assert!(taws.is_inhibited(Alert::Mode1));
    /// ```
    pub fn inhibit(&mut self, alert_system: Alert) {
        self.functions.get_mut(&alert_system).unwrap().inhibit()
    }

    /// Uninhibit a specific alert system
    ///
    /// # Arguments
    ///
    /// * `alert_system` - The alert system which shall be uninhibited
    ///
    /// # Example
    ///
    /// ```
    /// # use opentaws::prelude::*;
    /// # let config = TAWSConfig::default();
    /// # let mut taws = TAWS::new(config);
    /// taws.uninhibit(Alert::Mode1);
    ///
    /// assert_eq!(taws.is_inhibited(Alert::Mode1), false);
    /// ```
    pub fn uninhibit(&mut self, alert_system: Alert) {
        self.functions.get_mut(&alert_system).unwrap().uninhibit()
    }

    /// Process a new aircraft state
    ///
    /// This method must be called regularly for the TAWS to function properly!
    /// No warnings will be emitted without calling this function.
    ///
    /// # Arguments
    ///
    /// * `alert_system` - The alert system which shall be uninhibited
    ///
    /// # Example
    ///
    /// ```
    /// # use opentaws::prelude::*;
    /// # let config = TAWSConfig::default();
    /// # let mut taws = TAWS::new(config);
    /// let aicraft_state = AircraftState::default();
    ///
    /// let alert_state = taws.process(&aicraft_state);
    /// println!("Received AlertState: {:?}", alert_state);
    /// ```
    pub fn process(&mut self, state: &AircraftState) -> AlertState {
        let mut alert_state = alerts::AlertState::default();

        let alerts = self
            .functions
            .iter_mut()
            .filter_map(|(_k, v)| v.process(state));

        alert_state.update(alerts);
        alert_state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore] // TODO enable this test once all alert systems are implemented
    #[test]
    fn check_all_alert_systems() {
        let taws = TAWS::new(Default::default());
        let _ = taws.is_armed(Alert::FFAC);
        let _ = taws.is_armed(Alert::FLTA);
        let _ = taws.is_armed(Alert::Mode1);
        let _ = taws.is_armed(Alert::Mode2);
        let _ = taws.is_armed(Alert::Mode3);
        let _ = taws.is_armed(Alert::Mode4);
        let _ = taws.is_armed(Alert::Mode5);
        let _ = taws.is_armed(Alert::PDA);
    }
}
