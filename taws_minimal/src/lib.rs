use opentaws::prelude::*;

#[macro_use]
mod macros;

/// This is the number of different alerts in an alert_state for MinimalTAWS
const ALERT_STATE_SIZE: usize = 8;

/// Represents one instance of a TAWS
#[derive(Debug)]
pub struct MinimalTaws<'a> {
    /// `true` if the TAWS is armed
    ///
    /// There is no specific condition for changing this to `false`.
    pub armed: bool,
    config: &'a TawsConfig<'a>,
    ffac: functionalities::Ffac,
    flta: functionalities::Flta,
    mode1: functionalities::Mode1,
    mode2: functionalities::Mode2,
    mode3: functionalities::Mode3,
    mode4: functionalities::Mode4,
    mode5: functionalities::Mode5,
    pda: functionalities::Pda<'a>,
}

impl<'a> MinimalTaws<'a> {
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
    /// use taws_minimal::MinimalTaws;
    ///
    /// let config = TawsConfig::default();
    /// let taws = MinimalTaws::new(&config);
    /// ```
    pub fn new(config: &'a TawsConfig) -> Self {
        let ffac = functionalities::Ffac::new(config);
        let flta = functionalities::Flta::new(config);
        let mode1 = functionalities::Mode1::new(config);
        let mode2 = functionalities::Mode2::new(config);
        let mode3 = functionalities::Mode3::new(config);
        let mode4 = functionalities::Mode4::new(config);
        let mode5 = functionalities::Mode5::new(config);
        let pda = functionalities::Pda::new(config);

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
}

impl<'a> Taws<ALERT_STATE_SIZE> for MinimalTaws<'a> {
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
    /// # use taws_minimal::MinimalTaws;
    /// # let config = TawsConfig::default();
    /// # let taws = MinimalTaws::new(&config);
    /// if taws.is_armed(Alert::Mode1) {
    ///     // ...
    /// }
    /// ```
    fn is_armed(&self, alert_system: Alert) -> bool {
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
    /// # use taws_minimal::MinimalTaws;
    /// # let config = TawsConfig::default();
    /// # let mut taws = MinimalTaws::new(&config);
    /// taws.arm(Alert::Mode1);
    ///
    /// assert!(taws.is_armed(Alert::Mode1));
    /// ```
    fn arm(&mut self, alert_system: Alert) {
        self.get_mut_functionality(alert_system).arm();
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
    /// # use taws_minimal::MinimalTaws;
    /// # let config = TawsConfig::default();
    /// # let mut taws = MinimalTaws::new(&config);
    /// taws.disarm(Alert::Mode1);
    ///
    /// assert_eq!(taws.is_armed(Alert::Mode1), false);
    /// ```
    fn disarm(&mut self, alert_system: Alert) {
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
    /// # use taws_minimal::MinimalTaws;
    /// # let config = TawsConfig::default();
    /// # let taws = MinimalTaws::new(&config);
    /// if taws.is_inhibited(Alert::Mode1) {
    ///     // ...
    /// }
    /// ```
    fn is_inhibited(&self, alert_system: Alert) -> bool {
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
    /// # use taws_minimal::MinimalTaws;
    /// # let config = TawsConfig::default();
    /// # let mut taws = MinimalTaws::new(&config);
    /// taws.inhibit(Alert::Mode1);
    ///
    /// assert!(taws.is_inhibited(Alert::Mode1));
    /// ```
    fn inhibit(&mut self, alert_system: Alert) {
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
    /// # use taws_minimal::MinimalTaws;
    /// # let config = TawsConfig::default();
    /// # let mut taws = MinimalTaws::new(&config);
    /// taws.uninhibit(Alert::Mode1);
    ///
    /// assert_eq!(taws.is_inhibited(Alert::Mode1), false);
    /// ```
    fn uninhibit(&mut self, alert_system: Alert) {
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
    /// # use taws_minimal::MinimalTaws;
    /// # let config = TawsConfig::default();
    /// # let mut taws = MinimalTaws::new(&config);
    /// let aicraft_state = AircraftState::default();
    ///
    /// let alert_state = taws.process(&aicraft_state);
    /// println!("Received AlertState: {:?}", alert_state);
    /// ```
    fn process(&mut self, state: &AircraftState) -> AlertState<ALERT_STATE_SIZE> {
        let mut alert_state = AlertState::default();

        for (alert, alert_system) in self
            .functionality_mut_array()
            .iter_mut()
            .filter(|(_, alert_system)| !alert_system.is_inhibited())
        {
            if let Some((alert_level, _priority)) = alert_system.process(state) {
                alert_state.insert(*alert, alert_level);
            }
        }

        alert_state
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aviation_database::reference::AirportDatabaseImpl;

    #[test]
    fn check_all_alert_systems() {
        let airport_database = AirportDatabaseImpl::default();
        let config = TawsConfig {
            terrain_server: &airport_database,
            max_climbrate: Velocity::new::<foot_per_minute>(700.0),
            max_climbrate_change: Acceleration::new::<foot_per_second_squared>(100.0),
        };
        let taws = MinimalTaws::new(&config);
        let _ = taws.is_armed(Alert::Ffac);
        let _ = taws.is_armed(Alert::Flta);
        let _ = taws.is_armed(Alert::Mode1);
        let _ = taws.is_armed(Alert::Mode2);
        let _ = taws.is_armed(Alert::Mode3);
        let _ = taws.is_armed(Alert::Mode4);
        let _ = taws.is_armed(Alert::Mode5);
        let _ = taws.is_armed(Alert::Pda);
    }

    #[test]
    fn check_edge_case_1() {
        use uom::si::{
            angle::{degree, revolution},
            length::meter,
            time::second,
            velocity::meter_per_second,
        };

        let airport_database = AirportDatabaseImpl::default();
        let config = TawsConfig {
            terrain_server: &airport_database,
            max_climbrate: Velocity::new::<foot_per_minute>(700.0),
            max_climbrate_change: Acceleration::new::<foot_per_second_squared>(100.0),
        };
        let mut taws = MinimalTaws::new(&config);
        //Angle::new::<revolution>(1.0)

        let input = AircraftState {
            timestamp: Time::new::<second>(-1192551353.0),
            altitude: Length::new::<meter>(4608227.6136),
            altitude_ground: Length::new::<meter>(45.72),
            climb_rate: Velocity::new::<meter_per_second>(-3533302.6819200004),
            position_lat: Angle::new::<revolution>(-24748089.039895818),
            position_lon: Angle::new::<revolution>(-19226236.894961454),
            speed_ground: Velocity::new::<meter_per_second>(1060688081.2833334),
            speed_air: Velocity::new::<meter_per_second>(-87219196.62777779),
            heading: Angle::new::<degree>(-31215539.612156395),
            pitch: Angle::new::<revolution>(-15680644.065499812),
            roll: Angle::new::<revolution>(10762607.494661978),
            steep_approach: true,
            precision_approach: false,
            go_around: false,
            take_off: false,
        };
        let output = taws.process(&input);

        assert_eq!(
            output.priority_alert(),
            Some((Alert::Mode1, AlertLevel::Warning))
        );
    }
    // ———— [!] Step failed: ——————————————————————————  p_tester/src/tester.rs:220:25
    //  Aicraft state that violated the scenario: AircraftState {
    //      timestamp: -1192551353.0 s^1,
    //      altitude: 4608227.6136 m^1,
    //      altitude_ground: 45.72 m^1,
    //      climb_rate: -3533302.6819200004 m^1 s^-1,
    //      position_lat: -24748089.039895818,
    //      position_lon: -19226236.894961454,
    //      speed_ground: 1060688081.2833334 m^1 s^-1,
    //      speed_air: -87219196.62777779 m^1 s^-1,
    //      heading: -31215539.612156395,
    //      pitch: -15680644.065499812,
    //      roll: 10762607.494661978,
    //      steep_approach: true,
    //  }
    //  alerts emitted: AlertState {
    //      all_alerts: [
    //          None,
    //          None,
    //          None,
    //          None,
    //          None,
    //          None,
    //          None,
    //          None,
    //      ],
    //  }
}
