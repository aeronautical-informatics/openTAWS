use crate::{TawsAlert, TawsAlerts};
use enum_map::{Enum, EnumMap};

/// Alert Source (TAWS functionallities)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Enum)]
pub enum AlertSource {
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

/// TAWS Alert levels
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

/// Represents a TAWS alert
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Alert {
    /// The source resp. the TAWS functionallity which emitted this alert
    pub source: AlertSource,

    /// The alert level of this alert
    pub level: AlertLevel,
}

impl TawsAlert for Alert {
    fn source(&self) -> AlertSource {
        self.source
    }

    fn level(&self) -> AlertLevel {
        self.level
    }
}

impl From<(AlertSource, AlertLevel)> for Alert {
    fn from(alert: (AlertSource, AlertLevel)) -> Self {
        Alert {
            source: alert.0,
            level: alert.1,
        }
    }
}

impl From<Alert> for (AlertSource, AlertLevel) {
    fn from(alert: Alert) -> Self {
        (alert.source, alert.level)
    }
}

/// Represents a set of [Alerts](Alert)
#[derive(Default, Debug)]
pub struct Alerts {
    alerts: EnumMap<AlertSource, Option<Alert>>,
}

impl Alerts {
    /// Inserts the specified alert into the set.
    /// Already existing alerts are replaced if the [new_alert] has a higher alert level.
    /// # Arguments
    /// * `new_alert` - the new alert which is added to the set of alerts
    ///
    /// # Examples
    /// ```
    ///
    /// use opentaws::{*, alerts::*};
    /// let mut alerts = Alerts::default();
    /// alerts.insert((AlertSource::Mode1, AlertLevel::Caution).into());
    /// assert!(alerts.is_alert_active(AlertSource::Mode1, AlertLevel::Annunciation));
    /// assert!(alerts.is_alert_active(AlertSource::Mode1, AlertLevel::Caution));
    ///
    /// ```
    pub fn insert(&mut self, new_alert: Alert) {
        let current_alert = &self.alerts[new_alert.source];

        if current_alert
            .as_ref()
            .map_or(true, |alert| new_alert.level < alert.level)
        {
            self.alerts[new_alert.source].replace(new_alert);
        }
    }
}

impl TawsAlerts for Alerts {
    type Alert = Alert;

    fn is_alert_active(&self, alert_src: AlertSource, min_level: AlertLevel) -> bool {
        let current_alert = &self.alerts[alert_src];

        match current_alert {
            Some(alert) => alert.level <= min_level,
            None => false,
        }
    }
}

impl<'a> IntoIterator for &'a Alerts {
    type Item = &'a Alert;
    type IntoIter = AlertsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AlertsIter {
            alerts: self.alerts.as_slice(),
            index: 0,
        }
    }
}

/// Represents an iterator over all possible [Alerts](Alert) from all [AlertSources](AlertSource).
pub struct AlertsIter<'a> {
    alerts: &'a [Option<Alert>],
    index: usize,
}

impl<'a> Iterator for AlertsIter<'a> {
    type Item = &'a Alert;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.alerts.len() {
                return None;
            }

            let alert = &self.alerts[self.index];
            self.index += 1;

            if alert.is_none() {
                continue;
            }

            return alert.as_ref();
        }
    }
}

mod tests {
    #![allow(unused_imports)] //ToDo: just to satisfy clippy
    use super::*;

    #[test]
    fn alert_source_equality() {
        let a = AlertSource::Mode1;
        let b = AlertSource::Mode1;
        let c = AlertSource::Mode2;

        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn alert_level_ord() {
        assert!(AlertLevel::Warning < AlertLevel::Caution);
        assert!(AlertLevel::Caution < AlertLevel::Annunciation);
    }

    #[test]
    fn alert_from_tuple() {
        let alert: Alert = (AlertSource::Mode1, AlertLevel::Warning).into();
        assert!(alert.source == AlertSource::Mode1);
        assert!(alert.level == AlertLevel::Warning);
    }

    #[test]
    fn alert_to_tuple() {
        let alert_tuple = (AlertSource::Mode1, AlertLevel::Warning);
        let alert: Alert = alert_tuple.into();
        assert!(<Alert as Into<(AlertSource, AlertLevel)>>::into(alert) == alert_tuple);
    }

    #[test]
    fn alert_source() {
        let a: Alert = (AlertSource::Mode1, AlertLevel::Warning).into();
        assert!(a.source() == AlertSource::Mode1);
    }

    #[test]
    fn alert_level() {
        let a: Alert = (AlertSource::Mode1, AlertLevel::Warning).into();
        assert!(a.level() == AlertLevel::Warning);
    }

    #[test]
    fn alert_eq() {
        let a: Alert = (AlertSource::Mode1, AlertLevel::Warning).into();
        let b: Alert = (AlertSource::Mode1, AlertLevel::Warning).into();
        let c: Alert = (AlertSource::Mode3, AlertLevel::Warning).into();
        let d: Alert = (AlertSource::Mode1, AlertLevel::Caution).into();

        assert!(a == b);
        assert!(a != c);
        assert!(a != d);
        assert!(c != d);
    }

    #[test]
    fn alerts_default() {
        let alerts: Alerts = Alerts::default();
        assert!(alerts.alerts[AlertSource::Mode1] == None)
    }

    #[test]
    fn alerts_insert() {
        let mut alerts: Alerts = Alerts::default();
        let alert: Alert = (AlertSource::Mode1, AlertLevel::Caution).into();
        alerts.insert(alert);
        assert!(alerts.alerts[AlertSource::Mode1] == Some(alert));
    }

    #[test]
    fn alerts_is_active() {
        let mut alerts: Alerts = Alerts::default();
        let alert: Alert = (AlertSource::Mode1, AlertLevel::Caution).into();
        alerts.insert(alert);
        assert!(alerts.is_alert_active(AlertSource::Mode1, AlertLevel::Annunciation));
        assert!(alerts.is_alert_active(AlertSource::Mode1, AlertLevel::Caution));
        assert!(!alerts.is_alert_active(AlertSource::Mode1, AlertLevel::Warning));
    }
}
