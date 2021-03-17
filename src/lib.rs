//! This is a proof of concept TAWS as described in DO-367. It is not even close to fulfilling
//! DO-367 C, the simplest TAWS class. It exists to learn about using BDD (Cucumber & Gherkin in
//! particular) for implementing avionic.
//!
//! # Using openTAWS
//!
//! Currently it is only possible to use openTAWS from Rust. We've looked briefly into WASM-WASI
//! and C ABI as addiotional targets, but this did not lead anywehre usable _so far_. We are very
//! open to suggestions, so please open an issue if you have some feedback.

#![no_std]
#![deny(unsafe_code)]

pub use alerts::{functionalities, Alert, AlertLevel, AlertState};
use prelude::*;
pub use types::*;

#[macro_use]
mod macros;

mod alerts;
mod envelope;
pub mod prelude;
mod types;

/// Represents one instance of a TAWS
#[derive(Debug)]
pub struct Taws {
    /// `true` if the TAWS is armed
    ///
    /// There is no specific condition for changing this to `false`.
    pub armed: bool,
    config: TawsConfig,
    ffac: functionalities::Ffac,
    flta: functionalities::Flta,
    mode1: functionalities::Mode1,
    mode2: functionalities::Mode2,
    mode3: functionalities::Mode3,
    mode4: functionalities::Mode4,
    mode5: functionalities::Mode5,
    pda: functionalities::Pda,
}

impl Taws {
    functionalities![Ffac, Flta, Mode1, Mode2, Mode3, Mode4, Mode5, Pda];

    /// Create a new instance of `Taws`
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
    /// let config = TawsConfig::default();
    /// let taws = Taws::new(config);
    /// ```
    pub fn new(config: TawsConfig) -> Self {
        use alerts::*;

        let ffac = functionalities::Ffac::new(&config);
        let flta = functionalities::Flta::new(&config);
        let mode1 = functionalities::Mode1::new(&config);
        let mode2 = functionalities::Mode2::new(&config);
        let mode3 = functionalities::Mode3::new(&config);
        let mode4 = functionalities::Mode4::new(&config);
        let mode5 = functionalities::Mode5::new(&config);
        let pda = functionalities::Pda::new(&config);

        Self {
            armed: true,
            config,
            ffac,
            flta,
            mode1,
            mode2,
            mode3,
            mode4,
            mode5,
            pda,
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
    /// # let config = TawsConfig::default();
    /// # let taws = Taws::new(config);
    /// if taws.is_armed(Alert::Mode1) {
    ///     // ...
    /// }
    /// ```
    pub fn is_armed(&self, alert_system: Alert) -> bool {
        self.get_functionality(alert_system).is_armed()
    }

    /// Arms a specific alert system
    ///
    /// # Arguments
    ///
    /// * `alert_system` - The alert system which shall be inhibited
    ///
    /// # Example
    ///
    /// ```
    /// # use opentaws::prelude::*;
    /// # let config = TawsConfig::default();
    /// # let mut taws = Taws::new(config);
    /// taws.arm(Alert::Mode1);
    ///
    /// assert!(taws.is_armed(Alert::Mode1));
    /// ```
    pub fn arm(&mut self, alert_system: Alert) {
        self.get_mut_functionality(alert_system).arm()
    }

    /// Disarms a specific alert system
    ///
    /// # Arguments
    ///
    /// * `alert_system` - The alert system which shall be inhibited
    ///
    /// # Example
    ///
    /// ```
    /// # use opentaws::prelude::*;
    /// # let config = TawsConfig::default();
    /// # let mut taws = Taws::new(config);
    /// taws.disarm(Alert::Mode1);
    ///
    /// assert_eq!(taws.is_armed(Alert::Mode1), false);
    /// ```
    pub fn disarm(&mut self, alert_system: Alert) {
        self.get_mut_functionality(alert_system).disarm()
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
    /// # let config = TawsConfig::default();
    /// # let taws = Taws::new(config);
    /// if taws.is_inhibited(Alert::Mode1) {
    ///     // ...
    /// }
    /// ```
    pub fn is_inhibited(&self, alert_system: Alert) -> bool {
        self.get_functionality(alert_system).is_inhibited()
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
    /// # let config = TawsConfig::default();
    /// # let mut taws = Taws::new(config);
    /// taws.inhibit(Alert::Mode1);
    ///
    /// assert!(taws.is_inhibited(Alert::Mode1));
    /// ```
    pub fn inhibit(&mut self, alert_system: Alert) {
        self.get_mut_functionality(alert_system).inhibit()
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
    /// # let config = TawsConfig::default();
    /// # let mut taws = Taws::new(config);
    /// taws.uninhibit(Alert::Mode1);
    ///
    /// assert_eq!(taws.is_inhibited(Alert::Mode1), false);
    /// ```
    pub fn uninhibit(&mut self, alert_system: Alert) {
        self.get_mut_functionality(alert_system).uninhibit()
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
    /// # let config = TawsConfig::default();
    /// # let mut taws = Taws::new(config);
    /// let aicraft_state = AircraftState::default();
    ///
    /// let alert_state = taws.process(&aicraft_state);
    /// println!("Received AlertState: {:?}", alert_state);
    /// ```
    pub fn process(&mut self, state: &AircraftState) -> AlertState {
        let mut alert_state = alerts::AlertState::default();

        for (alert, alert_system) in self
            .functionality_mut_array()
            .iter_mut()
            .filter(|(_, alert_system)| !alert_system.is_inhibited())
        {
            if let Some(alert_level) = alert_system.process(state) {
                alert_state.insert(*alert, alert_level);
            }
        }

        alert_state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_all_alert_systems() {
        let taws = Taws::new(Default::default());
        let _ = taws.is_armed(Alert::Ffac);
        let _ = taws.is_armed(Alert::Flta);
        let _ = taws.is_armed(Alert::Mode1);
        let _ = taws.is_armed(Alert::Mode2);
        let _ = taws.is_armed(Alert::Mode3);
        let _ = taws.is_armed(Alert::Mode4);
        let _ = taws.is_armed(Alert::Mode5);
        let _ = taws.is_armed(Alert::Pda);
    }
}
