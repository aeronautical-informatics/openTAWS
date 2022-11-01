//! This is a proof of concept TAWS as described in DO-367. It is not even close to fulfilling
//! DO-367 C, the simplest TAWS class. It exists to learn about using BDD (Cucumber & Gherkin in
//! particular) for implementing avionic.
//!
//! # Using openTAWS
//!
//! Currently it is only possible to use openTAWS from Rust. We've looked briefly into WASM-WASI
//! and C ABI as addiotional targets, but this did not lead anywehre usable _so far_. We are very
//! open to suggestions, so please open an issue if you have some feedback.

//#![feature(return_position_impl_trait_in_trait)]
#![no_std]
#![deny(unsafe_code)]
#![allow(dead_code)]

#[cfg(test)]
#[macro_use]
extern crate std;

mod aircraft_state;
pub mod alerts;
pub mod class_c;
pub mod envelope;
pub mod prelude;

use prelude::*;

use core::fmt::Display;

/// Abstraction for a TAWS system
pub trait Taws {
    /// Alert source type
    type AlertSource: TawsAlertSource;

    /// Alert type
    type Alert: TawsAlert<AlertSource = Self::AlertSource>;

    /// Alert-set type
    type Alerts: TawsAlerts<Alert = Self::Alert> + Default;

    type Functionalities: TawsFunctionalities;

    fn functionalities(
        &self,
    ) -> &dyn TawsFunctionalities<AlertSource = Self::AlertSource, Alert = Self::Alert>;

    fn functionalities_mut(
        &mut self,
    ) -> &mut dyn TawsFunctionalities<AlertSource = Self::AlertSource, Alert = Self::Alert>;

    /// Returns whether the specified alert source (TAWS functionality) is currently armed.
    /// # Arguments
    /// * `alert_src` - The alert source of which the armed state is returned.
    fn is_armed(&self, alert_src: Self::AlertSource) -> bool {
        self.functionalities().is_armed(alert_src)
    }

    /// Inhibits the output of the specified alert source.
    /// # Arguements
    /// * `alert_src` - The alert source to inhibit.
    fn inhibit(&mut self, alert_src: Self::AlertSource) {
        self.functionalities_mut().inhibit(alert_src)
    }

    /// Un-inhibits the output of the specified alert source.
    /// # Arguements
    /// * `alert_src` - The alert source to un-inhibit.
    fn uninhibit(&mut self, alert_src: Self::AlertSource) {
        self.functionalities_mut().uninhibit(alert_src)
    }

    /// Returns whether the specified alert source is currently inhibited.
    /// # Arguments
    /// * `alert_src` - The alert source of which the inhibit state is returned.
    fn is_inhibited(&self, alert_src: Self::AlertSource) -> bool {
        self.functionalities().is_inhibited(alert_src)
    }

    /// Processes a normalized [AircraftState] and
    /// returns an alert for each alert source if the related conditions for this TAWS functionality are given.
    /// # Arguments
    /// * `state` - The normalized [AircraftState] to process.
    fn process(
        &mut self,
        state: &NormalizedAircraftState,
    ) -> Result<Self::Alerts, &'static dyn TawsError> {
        let mut alerts = Self::Alerts::default();
        let funcs = self.functionalities_mut();
        for alert_src in <Self::AlertSource as TawsAlertSource>::ALERT_SOURCES {
            let alert = funcs.functionality_mut(*alert_src).process(state)?;
            if let Some(alert) = alert {
                alerts.insert(alert);
            }
        }

        Ok(alerts)
    }
}

pub trait TawsFunctionalities {
    type AlertSource: TawsAlertSource;
    type Alert: TawsAlert<AlertSource = Self::AlertSource>;

    fn functionality(
        &self,
        alert_src: Self::AlertSource,
    ) -> &dyn TawsFunctionality<AlertSource = Self::AlertSource, Alert = Self::Alert>;

    fn functionality_mut(
        &mut self,
        alert_src: Self::AlertSource,
    ) -> &mut dyn TawsFunctionality<AlertSource = Self::AlertSource, Alert = Self::Alert>;

    /// Returns whether the specified alert source (TAWS functionality) is currently armed.
    /// # Arguments
    /// * `alert_src` - The alert source of which the armed state is returned.
    //	ToDo: return -> Result<bool, impl TawsError> when feature(return_position_impl_trait_in_trait) is stable.
    fn is_armed(&self, alert_src: Self::AlertSource) -> bool {
        self.functionality(alert_src).is_armed()
    }

    /// Inhibits the output of the specified alert source.
    /// # Arguements
    /// * `alert_src` - The alert source to inhibit.
    fn inhibit(&mut self, alert_src: Self::AlertSource) {
        self.functionality_mut(alert_src).inhibit();
    }

    /// Un-inhibits the output of the specified alert source.
    /// # Arguements
    /// * `alert_src` - The alert source to un-inhibit.
    fn uninhibit(&mut self, alert_src: Self::AlertSource) {
        self.functionality_mut(alert_src).uninhibit();
    }

    /// Returns whether the specified alert source is currently inhibited.
    /// # Arguments
    /// * `alert_src` - The alert source of which the inhibit state is returned.
    fn is_inhibited(&self, alert_src: Self::AlertSource) -> bool {
        self.functionality(alert_src).is_inhibited()
    }

    /// Processes a normalized [AircraftState] and
    /// returns an alert for each alert source if the related conditions for this TAWS functionality are given.
    /// # Arguments
    /// * `alert_src` - The alert source to process.
    /// * `state` - The normalized [AircraftState] to process.
    fn process(
        &mut self,
        alert_src: Self::AlertSource,
        state: &NormalizedAircraftState,
    ) -> Result<Option<Self::Alert>, &'static dyn TawsError> {
        self.functionality_mut(alert_src).process(state)
    }
}

/// Represents a TAWS functionality
pub trait TawsFunctionality {
    type AlertSource: TawsAlertSource;

    /// Alert type
    type Alert: TawsAlert<AlertSource = Self::AlertSource>;

    /// The associated alert source of this functionality.
    //const ALERT_SOURCE: <Self::Alert as TawsAlert>::AlertSource;

    fn alert_source(&self) -> Self::AlertSource;

    /// Returns whether the functionality is armed.
    fn is_armed(&self) -> bool;

    /// Inhibits the functionality.
    fn inhibit(&mut self);

    /// Un-inhibits the functionality.
    fn uninhibit(&mut self);

    /// Returns whether the functionality is currently inhibited.
    fn is_inhibited(&self) -> bool;

    /// Processes a normalized [AircraftState] and returns an alert result or an error.
    /// # Arguments
    /// * `state` - The normalized [AircraftState] to process.
    fn process(
        &mut self,
        state: &NormalizedAircraftState,
    ) -> Result<Option<Self::Alert>, &'static dyn TawsError>; // Result ?
}

/// Abstraction for a set of TAWS alerts
pub trait TawsAlerts {
    type AlertSource: TawsAlertSource;

    /// Alert type
    type Alert: TawsAlert<AlertSource = Self::AlertSource>;

    /// Inserts the specified alert into the set.
    /// Already existing alerts are replaced if the [new_alert] has a higher alert level.
    /// # Arguments
    /// * `new_alert` - the new alert which is added to the set of alerts
    fn insert(&mut self, new_alert: Self::Alert);

    /// Returns whether an alert from the given source with at least the specified alert level is active.
    /// # Arguments
    /// * `alert_src` - The alert source to check for.
    /// * `min_level` - The inclusive min level to check for.
    fn is_alert_active(&self, alert_src: Self::AlertSource, min_level: AlertLevel) -> bool;
}

/// Abstraction for a TAWS alert
pub trait TawsAlert {
    /// Alert source
    type AlertSource: TawsAlertSource;

    /// Returns the alert source to which this alert belongs.
    fn source(&self) -> Self::AlertSource;

    /// Returns the alert level of this alert.
    fn level(&self) -> AlertLevel;
}

/// Abstraction for an alert source
pub trait TawsAlertSource: Clone + Copy + Eq + 'static {
    const NUM_ALERT_SOURCES: usize;

    const ALERT_SOURCES: &'static [Self];
}

pub trait TawsError: core::fmt::Debug + Display {}
