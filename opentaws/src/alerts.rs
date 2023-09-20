use core::hash::Hash;
use heapless::FnvIndexMap;

use crate::prelude::*;

/// TAWS Alert levels
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Alert<AlertSource: TawsAlertSource> {
    /// The source resp. the TAWS functionallity which emitted this alert
    pub source: AlertSource,

    /// The alert level of this alert
    pub level: AlertLevel,
}

impl<AlertSource: TawsAlertSource> Alert<AlertSource> {
    /// Creates a new alert with the specified source and level.
    /// # Arguments
    /// `source` - The source of the alert.
    /// `level` - The level of the alert.
    pub const fn new(source: AlertSource, level: AlertLevel) -> Self {
        Alert { source, level }
    }
}

impl<AlertSource: TawsAlertSource> TawsAlert for Alert<AlertSource> {
    type AlertSource = AlertSource;

    fn source(&self) -> AlertSource {
        self.source
    }

    fn level(&self) -> AlertLevel {
        self.level
    }
}

impl<AlertSource: TawsAlertSource> From<(AlertSource, AlertLevel)> for Alert<AlertSource> {
    fn from(alert: (AlertSource, AlertLevel)) -> Self {
        Self::new(alert.0, alert.1)
    }
}

impl<AlertSource: TawsAlertSource> From<Alert<AlertSource>> for (AlertSource, AlertLevel) {
    fn from(alert: Alert<AlertSource>) -> Self {
        (alert.source, alert.level)
    }
}

/// Represents a set of [Alerts](Alert) by their [AlertSource](Alert::AlertSource)
#[derive(Debug)]
pub struct Alerts<Alert: TawsAlert>
where
    Alert::AlertSource: Hash,
{
    alerts: FnvIndexMap<
        Alert::AlertSource,
        Alert,
        64, /*ToDo: use <Alert::AlertSource>::NUM_ALERT_SOURCES or count the available alert sources with a macro*/
    >,
}

impl<Alert: TawsAlert> Default for Alerts<Alert>
where
    Alert::AlertSource: Hash,
{
    fn default() -> Self {
        Self {
            alerts: Default::default(),
        }
    }
}

impl<Alert: TawsAlert> Alerts<Alert>
where
    Alert::AlertSource: Hash,
{
    /// Returns an iterator over all active alerts.
    pub fn alerts(&self) -> impl Iterator<Item = &Alert> + '_ {
        self.alerts.values()
    }
}

impl<Alert: TawsAlert> TawsAlerts for Alerts<Alert>
where
    Alert::AlertSource: Hash,
{
    type Alert = Alert;
    type AlertSource = Alert::AlertSource;

    fn insert(&mut self, new_alert: Alert) {
        let current_alert = self.alerts.get(&new_alert.source());

        if current_alert.map_or(true, |alert| new_alert.level() < alert.level()) {
            self.alerts
                .insert(new_alert.source(), new_alert)
                .map_err(|_| ())
                .unwrap(); //ToDo
        }
    }

    fn is_alert_active(&self, alert_src: Self::AlertSource, min_level: AlertLevel) -> bool {
        self.alerts.contains_key(&alert_src);

        match self.alerts.get(&alert_src) {
            Some(alert) => alert.level() <= min_level,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::slice::Iter;

    use super::*;

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    enum TestClass {
        A,
        B,
        C,
    }

    impl TawsAlertSource for TestClass {
        const NUM_ALERT_SOURCES: usize = 3;
        const ALERT_SOURCES: &'static [Self] = &[];
    }

    impl IntoIterator for TestClass {
        type Item = &'static TestClass;

        type IntoIter = Iter<'static, TestClass>;

        fn into_iter(self) -> Self::IntoIter {
            [Self::A, Self::B, Self::C].iter()
        }
    }

    type TestAlert = Alert<TestClass>;
    type TestAlerts = Alerts<TestAlert>;

    #[test]
    fn alert_level_eq() {
        assert!(AlertLevel::Warning == AlertLevel::Warning);
        assert!(AlertLevel::Warning != AlertLevel::Caution);
        assert!(AlertLevel::Warning < AlertLevel::Caution);
        assert!(AlertLevel::Caution < AlertLevel::Annunciation);
    }

    #[test]
    fn alert_eq() {
        let alert1: TestAlert = (TestClass::A, AlertLevel::Warning).into();
        let alert2: TestAlert = (TestClass::A, AlertLevel::Warning).into();
        let alert3: TestAlert = (TestClass::B, AlertLevel::Warning).into();
        let alert4: TestAlert = (TestClass::A, AlertLevel::Annunciation).into();

        assert!(alert1 == alert1);
        assert!(alert1 == alert2);
        assert!(alert1 != alert3);
        assert!(alert1 != alert4);
    }

    #[test]
    fn alerts_insert() {
        let mut alerts = TestAlerts::default();
        assert!(!alerts.alerts.contains_key(&TestClass::A));

        let alert1: TestAlert = (TestClass::A, AlertLevel::Warning).into();
        let alert2: TestAlert = (TestClass::A, AlertLevel::Caution).into();

        alerts.insert(alert1);
        assert!(alerts.alerts.contains_key(&TestClass::A));

        alerts.insert(alert2);
        assert!(alerts.alerts.contains_key(&TestClass::A));
    }

    #[test]
    fn alerts_is_active() {
        let mut alerts = TestAlerts::default();
        let alert1: TestAlert = (TestClass::A, AlertLevel::Caution).into();

        alerts.insert(alert1);

        assert!(alerts.is_alert_active(TestClass::A, AlertLevel::Annunciation));
        assert!(alerts.is_alert_active(TestClass::A, AlertLevel::Caution));
        assert!(!alerts.is_alert_active(TestClass::A, AlertLevel::Warning));

        let alert2: TestAlert = (TestClass::A, AlertLevel::Annunciation).into();
        alerts.insert(alert2);
        assert!(alerts.is_alert_active(TestClass::A, AlertLevel::Caution));

        let alert3: TestAlert = (TestClass::A, AlertLevel::Warning).into();
        alerts.insert(alert3);
        assert!(alerts.is_alert_active(TestClass::A, AlertLevel::Warning));
    }

    #[test]
    fn alerts_get() {
        let mut alerts = TestAlerts::default();
        let alert1: TestAlert = (TestClass::A, AlertLevel::Annunciation).into();
        let alert2: TestAlert = (TestClass::B, AlertLevel::Caution).into();
        let alert3: TestAlert = (TestClass::C, AlertLevel::Warning).into();

        alerts.insert(alert1);
        alerts.insert(alert2);
        alerts.insert(alert3);

        assert!(alerts.alerts().count() == 3)
    }
}
