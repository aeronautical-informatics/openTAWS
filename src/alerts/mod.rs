use std::collections::{HashMap, HashSet};

use crate::types::AircraftState;

pub mod mode_1;

pub type Alert = (Functionality, AlertLevel);

/// Available alerts from the TAWS
#[derive(Clone, Copy, Debug, PartialEq, Hash)]
#[derive(strum::EnumString)]
pub enum Functionality {
    /// Forward Lookig Terrain Avoidance
    FLTA,

    /// Premature Descent Alerting
    PDA,

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
impl Eq for Functionality {}

/// Severity level of an alert
///
/// Orderd by high priority to low priority
#[derive(Clone, Copy, Debug, PartialEq, Hash)]
#[derive(strum::EnumString)]
#[strum(serialize_all = "kebab_case")]
pub enum AlertLevel {
    /// The level or category of alert for conditions that require immediate flight crew awareness
    /// and immediate flight crew response.
    Warning,

    /// The level or category of alert for conditions that require immediate flight crew awareness
    /// and a less urgent subsequent flight crew response than a warning alert.  
    Caution,
}
impl Eq for AlertLevel {}

/// Collection of a all alerts which are currently present
#[derive(Debug, Default, PartialEq)]
pub struct AlertState {
    /// Alerts which are to be displayed to the crew
    pub alerts: HashSet<Alert>,

    /// Alerts which are not to be disclosed to the crew to avoid nuisance
    pub nuisance_alerts: HashSet<Alert>,
}

impl AlertState {
    pub fn alerts_total_count(&self) -> usize {
        self.alerts.union(&self.nuisance_alerts).count()
    }

    pub fn alerts_count(&self, level: AlertLevel) -> usize {
        self.alerts.iter().filter(|e| e.1 == level).count()
    }

    pub fn mode_alert_level(&self, mode: Functionality) -> Option<AlertLevel> {
        self.alerts
            .union(&self.nuisance_alerts)
            .find(|e| e.0 == mode)
            .map(|alert| alert.1)
    }

    /// udpates internal alerts with new alerts, removing all old alerts. Prioritizes as well.
    pub fn update<'a, I>(&mut self, new_alerts: I)
    where
        I: Iterator<Item = Alert>,
    {
        let mut map = HashMap::new();

        for (function, new_alert_level) in new_alerts {
            map.entry(function).and_modify(|old_alert_level| {
                if (new_alert_level as isize) < (*old_alert_level as isize) {
                    self.nuisance_alerts.insert((function, *old_alert_level));
                    *old_alert_level = new_alert_level;
                }
            });
        }

        self.alerts = map.drain().collect();
    }
}

/// Trait which is to be fulfilled by all functionalities
pub trait FunctionalityProcessor: std::fmt::Debug + Send {
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

    /// Process a new AircraftState, emit alerts if appropiate
    fn process(&mut self, state: &AircraftState) -> Option<Alert>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn strum_test() {
        let mut key = String::from("Mode 1");
        key.retain(|c| !c.is_whitespace());

        let f: Functionality = key
            .parse()
            .expect(&format!("Unable to parse {} as Functionality ", key));
    }

    #[test]
    pub fn alert_state_usage() {
        let alts = AlertState::default();

        // using hashset contains
        if alts
            .alerts
            .contains(&(Functionality::Mode1, AlertLevel::Caution))
        {
            // do important stuff
        }

        // using hashset any
        if alts.alerts.iter().any(|e| e.1 == AlertLevel::Caution) {
            // do important stuff
        }

        // using match
        for x in &alts.alerts {
            match x {
                (Functionality::Mode1, AlertLevel::Caution) if 1 > 0 => {}

                (Functionality::Mode1, AlertLevel::Caution) => {}
                (Functionality::Mode1, AlertLevel::Warning) => {}
                _ => {}
            }
        }

        // using match lazily
        for x in &alts.alerts {
            match x {
                (_, AlertLevel::Caution) if 1 > 0 => {}
                _ => {}
            }
        }
    }
}
