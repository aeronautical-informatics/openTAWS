use crate::{
    envelope::{Envelope, INVALID_ENVELOPE},
    Alert, AlertLevel, AlertSource, TawsFunctionality,
};
use lazy_static::lazy_static;
use nalgebra::Vector2;
use uom::si::length::foot;

pub struct PDA {
    alert_src: AlertSource,
    armed: bool,
    inhibited: bool,
}

impl Default for PDA {
    fn default() -> Self {
        Self {
            alert_src: AlertSource::Pda,
            armed: false,
            inhibited: false,
        }
    }
}

impl TawsFunctionality for PDA {
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
        // ToDo
        let altitude_gnd_foot = state.altitude_ground().get::<foot>();
        let distance_to_nearest_airport_nm = 3.0;

        self.armed = distance_to_nearest_airport_nm <= 5.0;
        if !self.armed {
            return Ok(None);
        }

        let result: Result<Option<Alert>, ()> =
            if CAUTION_ENVELOPE.contains(distance_to_nearest_airport_nm, altitude_gnd_foot)? {
                Ok(Some(Alert::new(self.alert_src, AlertLevel::Caution)))
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

lazy_static! {
    static ref CAUTION_ENVELOPE: Envelope::<6> = Envelope::try_new([
        Vector2::new(5.0, 10.0),
        Vector2::new(1.0, 10.0),
        Vector2::new(1.0, 80.0),
        Vector2::new(1.8, 150.0),
        Vector2::new(2.3, 170.0),
        Vector2::new(5.0, 170.0)
    ])
    .expect(INVALID_ENVELOPE);
}
