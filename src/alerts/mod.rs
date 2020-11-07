use std::collections::HashSet;

pub mod mode_1;

/// Available alerts from the TAWS
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "wasi", derive(serde::Serialize))]
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
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "wasi", derive(serde::Serialize))]
#[derive(strum::EnumString)]
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
#[cfg_attr(feature = "wasi", derive(serde::Serialize))]
pub struct AlertState {
    /// Alerts which are to be displayed to the crew
    pub alerts: HashSet<(Functionality, AlertLevel)>,

    /// Alerts which are not to be disclosed to the crew to avoid nuisance
    pub nuisance_alerts: HashSet<(Functionality, AlertLevel)>,
}

impl AlertState {
    pub fn count(&self, level: AlertLevel) -> usize {
        self.alerts.iter().filter(|e| e.1 == level).count()
    }
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
