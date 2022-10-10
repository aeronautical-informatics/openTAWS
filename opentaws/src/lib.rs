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
#![allow(dead_code)]

extern crate enum_map;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod aircraft_state;
pub mod alerts;
mod envelope;

pub use aircraft_state::{AircraftState, NormalizedAircraftState};
pub use alerts::{Alert, AlertLevel, AlertSource};

/// Abstraction for a TAWS system
pub trait Taws {
    /// Alert set type
    type Alerts: TawsAlerts + Default;

    //fn arm(&mut self, alert_src: AlertSource);
    //fn disarm(&mut self, alert_src: AlertSource);

    /// Returns whether the specified alert source (TAWS functionallity) is currently armed.
    /// # Arguments
    /// * `alert_src` - The alert source of which the armed state is returned.
    fn is_armed(&self, alert_src: AlertSource) -> bool;

    /// Inhibits the output of the specified alert source.
    /// # Arguements
    /// * `alert_src` - The alert source to inhibit.
    fn inhibit(&mut self, alert_src: AlertSource);

    /// Un-inhibits the output of the specified alert source.
    /// # Arguements
    /// * `alert_src` - The alert source to un-inhibit.
    fn uninhibit(&mut self, alert_src: AlertSource);

    /// Returns whether the specified alert source is currently inhibited.
    /// # Arguments
    /// * `alert_src` - The alert source of which the inhibit state is returned.
    fn is_inhibited(&self, alert_src: AlertSource) -> bool;

    /// Processes an normalized [AircraftState] and
    /// returns an alert for each alert source if the related conditions for this TAWS functionallity are given.
    /// # Arguments
    /// * `state` - The normalized [AircraftState] to process.
    fn process(&mut self, state: NormalizedAircraftState) -> Self::Alerts;
}

/// Abstraction for a TAWS alert
pub trait TawsAlert: Into<(AlertSource, AlertLevel)> + Eq {
    /// Returns the TAWS functionallity which emitted this alert
    fn source(&self) -> AlertSource;

    /// Returns the alert level of this alert
    fn level(&self) -> AlertLevel;
}

/// Abstraction for a set of TAWS alerts
pub trait TawsAlerts {
    /// Alert type
    type Alert: TawsAlert;

    /// Returns whether an alert from the given source with at least the specified alert level is active.
    /// # Arguments
    /// * `alert_src` - The alert source (TAWS functionallity) to check for
    /// * `min_level` - The inclusive min level to check for
    fn is_alert_active(&self, alert_src: AlertSource, min_level: AlertLevel) -> bool;
}
