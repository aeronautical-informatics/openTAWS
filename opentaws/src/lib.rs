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

#[cfg(test)]
#[macro_use]
extern crate std;

mod aircraft_state;
pub mod alerts;
pub mod class_c;
pub mod envelope;
pub mod prelude;

use core::fmt::Display;

use crate::prelude::*;

/// Abstraction for a TAWS system
pub trait Taws
where
    for<'a> &'a Self::Alerts: IntoIterator<Item = &'a Self::Alert>,
{
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

/// Abstraction for a set of `TawsAlert`s.
pub trait TawsAlerts
where
    for<'a> &'a Self: IntoIterator<Item = &'a Self::Alert>,
{
    type AlertSource: TawsAlertSource;

    /// Alert type
    type Alert: TawsAlert<AlertSource = Self::AlertSource>;

    /// Returns the alert with the specified `alert_src`, if one exists; otherwise `None`.
    /// # Arguments
    /// * `alert_src` - The `Self::AlertSource` to look for.
    fn get(&self, alert_src: Self::AlertSource) -> Option<&Self::Alert>;

    /// Returns the alert with the specified `alert_src` if one exists and if the alert has an `AlertLevel`
    /// greater than or equal to `min_level`; otherwise `None`.
    /// # Arguments
    /// * `alert_src` - The `Self::AlertSource` to look for.
    /// * `min_level`- Minimum `AlertLevel` to look for.
    fn get_min(&self, alert_src: Self::AlertSource, min_level: AlertLevel) -> Option<&Self::Alert> {
        self.get(alert_src)
            .and_then(|alert| (alert.level() <= min_level).then_some(alert))
    }

    /// Checks whether an alert with the specified `alert_src` exists that has a minimum `AlertLevel` of `min_level`.
    /// # Arguments
    /// * `alert_src` - The `Self::AlertSource` to check.
    /// * `min_level` - The minimum `AlertLevel`.
    fn is_alert_active(&self, alert_src: Self::AlertSource, min_level: AlertLevel) -> bool {
        self.get_min(alert_src, min_level).is_some()
    }

    /// Inserts the specified alert into the set.
    /// Already existing alerts are replaced if the [new_alert] has a higher alert level.
    /// # Arguments
    /// * `new_alert` - the new alert which is added to the set of alerts
    fn insert(&mut self, new_alert: Self::Alert);
}

/// Extension trait for `TawsAlerts` that implements alert prioritization
/// if the associated `TawsAlertSource` implements `TawsAlertSourcePrioritization`.
pub trait TawsAlertsPrioritizationExt: TawsAlerts
where
    for<'a> &'a Self: IntoIterator<Item = &'a Self::Alert>,
{
    type PrioritizedAlerts<'a>: TawsPrioritizedAlerts<'a, Alert = Self::Alert>
    where
        Self: 'a;

    /// Prioritize alerts by the associated `TawsAlertSourcePrioritization` implementation.
    fn prioritize(&self) -> Self::PrioritizedAlerts<'_>;
}

/// Abstraction for a sorted set of `TawsAlert`s.
pub trait TawsPrioritizedAlerts<'a> {
    type Alert: TawsAlert;

    /// Gets the alert at index `idx` from the prioritized (sorted) set of alerts,
    /// if one exists at the given `idx`; otherwise `None`.<br/>
    /// Low indices describe high priority. Index 0 contains the alert with the highest priority
    /// or `None` if no alert matches any of the rules in `TawsAlertSourcePrioritization`.<br/>
    /// It is important to note that an empty set here does not imply absent of alerts in general.
    /// It only mean that no alert matched any of the prioritization rules.
    /// # Arguments
    /// * `idx` - The index at which the prioritized (sorted) set of alerts is accessed.
    fn index(&self, idx: usize) -> Option<&'a Self::Alert>;
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

/// Maximum number of supported alert sources.
pub const MAX_NUM_ALERT_SOURCES: usize = 64;

/// Abstraction for an alert source
pub trait TawsAlertSource: Clone + Copy + Eq + 'static {
    const ALERT_SOURCES: &'static [Self];
}

/// Alert prioritization trait. Describes how `TawsAlert`s should be prioritized.
pub trait TawsAlertSourcePrioritization: TawsAlertSource {
    /// Prioritization rules that assign priorities to `TawsAlert`s.
    /// # Example
    /// ```
    /// # use opentaws::prelude::*;
    /// # enum AlertSource { Mode1, Mode3, Flta, Pda, Ffac }
    /// &[
    ///     (AlertSource::Mode1, AlertLevel::Warning),
    ///     (AlertSource::Flta, AlertLevel::Warning),
    ///     (AlertSource::Flta, AlertLevel::Caution),
    ///     (AlertSource::Pda, AlertLevel::Caution),
    ///     (AlertSource::Ffac, AlertLevel::Annunciation),
    ///     (AlertSource::Mode1, AlertLevel::Caution),
    ///     (AlertSource::Mode3, AlertLevel::Caution),
    /// ];
    /// ```
    ///
    /// * All Mode1 alerts with level Warning or higher will be assigned priority 0 (highest priority).
    /// * All Flta alerts with level Warning or higher will be assigned priority 1.
    /// * All Mode1 alerts with level Caution or higher will be assigned priority 5.
    ///
    /// For example these alerts: <br/>
    /// `{(Mode3, Annunciation), (Mode1, Caution), (Flta, Warning)}`<br/>
    /// will be sorted into this order: <br/>
    /// `[(Flta, Warning), (Mode1, Caution)]`
    ///
    /// * `(Flta, Warning)` has a higher priority than the other alerts because of rule 2.
    /// * `(Mode1, Caution)` has a lower priority than the Flta alert because of rule 6 but a higher priority than `(Mode3, Annunciation)`.
    /// * `(Mode3, Annunciation)` has no priotity because it does not match any rule and is therefore ignored.
    ///
    /// Alerts that do not fit into any rule cannot be prioritized, because the prioritization would be arbitrary and thus not reliable.<br/>
    /// If all alerts should be present in the sorted collection,
    /// consider adding a catch-all rules for all alert sources at the end with the lowest possible alert level (Annunciation).<br/>
    /// By doing this all alert sources get a priority assigned and the arbitrariness is prevented.
    const PRIORITIZATION: &'static [(Self, AlertLevel)];
}

pub trait TawsError: core::fmt::Debug + Display {}
