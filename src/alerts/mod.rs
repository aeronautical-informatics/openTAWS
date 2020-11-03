use std::collections::HashSet;

pub mod mode_1;

/// Available alerts from the TAWS
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "wasi", derive(serde::Serialize))]
pub enum Alert {
    /// Forward Lookig Terrain Avoidance
    FLTA,

    /// Premature Descent Alerting
    PDA,

    /// Excessive Rate of Descent
    Mode1(AlertLevel),

    /// Excessive ClosureRate to Terrain
    Mode2(AlertLevel),

    /// Negative Climb Rate or Altitude Loss after Take-off or Go Around
    Mode3(AlertLevel),

    /// Flight Near Terrain when Not in Landing Configuration
    Mode4(AlertLevel),

    /// Excessive Downward Deviation from an ILS Glideslope or LPV/GLS Glidepath
    Mode5(AlertLevel),
    // TODO add more
}
impl Eq for Alert {}

/// Severity level of an alert
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "wasi", derive(serde::Serialize))]
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
    pub alerts: HashSet<Alert>,

    /// Alerts which are not to be disclosed to the crew to avoid nuisance
    pub nuisance_alerts: HashSet<Alert>,
}

impl AlertState {}
