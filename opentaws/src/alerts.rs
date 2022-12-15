use core::fmt;

use crate::types::{AircraftState, TawsConfig};

use heapless::Vec;

mod ffac;
mod flta;
mod mode_1;
mod mode_2;
mod mode_3;
mod mode_4;
mod mode_5;
mod pda;

pub mod functionalities {
    use super::*;

    pub use ffac::*;
    pub use flta::*;
    pub use mode_1::*;
    pub use mode_2::*;
    pub use mode_3::*;
    pub use mode_4::*;
    pub use mode_5::*;
    pub use pda::*;
}

/// Available alerts from the TAWS.
#[derive(Clone, Copy, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Alert {
    /// Forward Lookig Terrain Avoidance
    Flta,

    /// Five Hundred foot altitude Callout
    Ffac,

    /// Premature Descent Alerting
    Pda,

    /// Excessive Rate of Descent
    Mode1,

    /// Excessive ClosureRate to Terrain
    Mode2,

    /// Negative Climb Rate or Altitude Loss after Take-off or Go Around
    Mode3,

    /// Flight Near Terrain when Not in Landing Configuration
    Mode4,

    /// Excessive Downward Deviation from an ILS Glideslope or LPV/GLS Glidepath
    Mode5,
    // TODO add more
}
impl Eq for Alert {}

/// Importance level of an alert
///
/// Orderd by high priority to low priority (top to bottom)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlertLevel {
    /// The level or category of alert for conditions that require immediate flight crew awareness
    /// and immediate flight crew response.
    Warning,

    /// The level or category of alert for conditions that require immediate flight crew awareness
    /// and a less urgent subsequent flight crew response than a warning alert.  
    Caution,

    /// The level or category of an annunciation which does not represent a threat but still
    /// requires awareness by the crew
    Annunciation,
}
impl Eq for AlertLevel {}

/// Get the priority of a n (Alert, AlertLevel) tupel
///
/// A low value means a high priority.
/// TODO move to every alert file
pub fn priority(alert: Alert, alert_level: AlertLevel) -> u8 {
    use Alert::*;
    use AlertLevel::*;

    match (alert, alert_level) {
        (Mode1, Warning) => 2,
        (Mode2, Warning) => 3,
        (Flta, Warning) => 6,
        (Mode2, Caution) => 9,
        (Flta, Caution) => 11,
        (Mode4, Caution) => 13, // Terrain caution
        (Pda, Caution) => 14,
        //(Mode4, Caution)=>16 // Gear caution
        //(Mode4, Caution)=>17 // Flaps caution
        (Mode1, Caution) => 18,
        (Mode3, Caution) => 19,
        (Mode5, Caution) => 20,
        _ => u8::MAX, // TODO is this a safe assumption
    }
}

/// Collection of a all alerts which are currently present in the TAWS
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AlertState<const N: usize> {
    /// Alerts which are not to be disclosed to the crew to avoid nuisance, but still where triggered
    // Workaround until generic arrays are allowed in serde
    // https://github.com/serde-rs/serde/issues/1937
    all_alerts: Vec<(Alert, AlertLevel), N>,
}

impl<const N: usize> AlertState<N> {
    pub fn alerts_total_count(&self) -> usize {
        self.all_alerts.len()
    }

    pub fn priority_alert(&self) -> Option<(Alert, AlertLevel)> {
        self.all_alerts
            .iter()
            .map(|(alert, alert_level)| (priority(*alert, *alert_level), (*alert, *alert_level)))
            .min_by_key(|(p, _)| *p)
            .map(|(_, alert_stuff)| alert_stuff)
    }

    /// Get an iterator to the alerts
    pub fn iter(&self) -> impl Iterator<Item = (Alert, AlertLevel)> {
        self.into_iter()
    }

    /// updates internal alerts with new alerts, removing all old alerts. Prioritizes as well.
    pub fn insert(&mut self, new_alert: Alert, new_alert_level: AlertLevel) {
        let mut already_present = false;

        for (existing_alert, ref mut existing_alert_level) in &mut self.all_alerts {
            // check if alert is already present
            if *existing_alert == new_alert {
                // promote alerts of lower priority to higher priority
                if new_alert_level < *existing_alert_level {
                    *existing_alert_level = new_alert_level;
                }
                already_present = true;
            }
        }

        // lets find a free spot
        if !already_present {
            self.all_alerts.push((new_alert, new_alert_level)).expect(
                "failed to push new alert to alerts vector, this should be impossible to fail",
            );
        }
    }
}

impl<const N: usize> Default for AlertState<N> {
    fn default() -> Self {
        Self {
            all_alerts: Vec::new(),
        }
    }
}

/// An iterator over an `AlertState`
pub struct AlertStateIter<const N: usize> {
    sorted_alerts: Vec<(Alert, AlertLevel), N>,
    index: usize,
}

impl<const N: usize> Iterator for AlertStateIter<N> {
    type Item = (Alert, AlertLevel);

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_item = self.sorted_alerts.get(self.index);
        if maybe_item.is_some() {
            self.index += 1;
        }

        maybe_item.copied()
    }
}

impl<const N: usize> IntoIterator for &AlertState<N> {
    type Item = (Alert, AlertLevel);
    type IntoIter = AlertStateIter<N>;
    fn into_iter(self) -> Self::IntoIter {
        let mut alerts = self.all_alerts.clone();

        alerts.sort_by_key(|(a, l)| priority(*a, *l));
        AlertStateIter {
            sorted_alerts: alerts,
            index: 0,
        }
    }
}

/// Trait which is to be fulfilled by all functionalities
pub trait AlertSystem<'a>: fmt::Debug + Send {
    /// Allows this system to be instantiated
    fn new(config: &'a TawsConfig<'a>) -> Self
    where
        Self: Sized;

    /// Returns whether this alarm is armed.
    ///
    /// Arming refers to the automatic switching on of a function by
    /// the Equipment (DO-367 Chapter 1.9).
    fn is_armed(&self) -> bool;

    /// Arm this alert
    fn arm(&mut self);

    /// Disarm this alert
    fn disarm(&mut self);

    /// Dismiss this alert
    fn inhibit(&mut self);

    /// Enable this alert
    fn uninhibit(&mut self);

    /// Returns whether this alarm is inhibited
    fn is_inhibited(&self) -> bool;

    /// Process a new AircraftState, emit alerts if appropiate
    ///
    /// The `u8` refers to the Priority of this AlertFunctionality
    fn process(&mut self, state: &AircraftState) -> Option<(AlertLevel, u8)>;
}

#[cfg(test)]
mod test {
    use super::*;

    /// This is the maximum number of different alerts in an alert_state
    const ALERT_STATE_SIZE: usize = 8;

    #[test]
    pub fn alert_state_propagates_alerts() {
        let mut alert_state = AlertState::<ALERT_STATE_SIZE>::default();
        let test_alerts = [(Alert::Mode3, AlertLevel::Caution)];
        for (new_alert, new_alert_level) in &test_alerts {
            alert_state.insert(*new_alert, *new_alert_level);
        }

        assert_eq!(test_alerts.len(), alert_state.alerts_total_count())
    }

    #[test]
    pub fn alert_state_usage() {
        let alts = AlertState::<ALERT_STATE_SIZE>::default();

        // using match
        for x in &alts {
            match x {
                (Alert::Mode1, AlertLevel::Caution) if 1 > 0 => {}

                (Alert::Mode1, AlertLevel::Caution) => {}
                (Alert::Mode1, AlertLevel::Warning) => {}
                _ => {}
            }
        }

        // using match lazily
        for x in &alts {
            match x {
                (_, AlertLevel::Caution) if 1 > 0 => {}
                _ => {}
            }
        }
    }
}
