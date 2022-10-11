use crate::{aircraft_state::FlightSegment, Alert, AlertLevel, AlertSource, TawsFunctionality};

use uom::{
    num_traits::Zero,
    si::{f64::Length, length::foot},
};

pub struct Ffac {
    alert_src: AlertSource,
    armed: bool,
    inhibited: bool,

    last_altitude: Length,
}

impl Default for Ffac {
    fn default() -> Self {
        Self {
            alert_src: AlertSource::Ffac,
            armed: true,
            inhibited: false,
            last_altitude: Length::zero(),
        }
    }
}

impl TawsFunctionality for Ffac {
    type Alert = Alert;

    fn alert_source(&self) -> AlertSource {
        self.alert_src
    }

    fn is_armed(&self) -> bool {
        self.armed
    }

    fn inhibit(&mut self) {
        self.inhibited = true;
    }

    fn uninhibit(&mut self) {
        self.inhibited = false;
    }

    fn is_inhibited(&self) -> bool {
        self.inhibited
    }

    fn process(
        &mut self,
        state: crate::NormalizedAircraftState,
    ) -> Result<Option<Self::Alert>, ()> {
        let armed = state
            .situation()
            .map(|s| !matches!(s, FlightSegment::TakeOff | FlightSegment::GoAround))
            .ok_or(())?;

        self.armed = armed;
        if !self.armed {
            return Ok(None);
        }

        let fivehundred = Length::new::<foot>(500.0);
        let result: Result<Option<Alert>, ()> =
            if self.last_altitude >= fivehundred && state.altitude_ground() < fivehundred {
                Ok(Some(Alert::new(self.alert_src, AlertLevel::Annunciation)))
            } else {
                Ok(None)
            };

        if !self.inhibited {
            result
        } else {
            Ok(None)
        }
    }
}
