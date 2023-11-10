use crate::prelude::*;
use crate::{alerts::*, envelope::*};

use super::ClassC_Source;

use lazy_static::lazy_static;

#[derive(Clone, Debug, Default)]
pub struct Pda {
    armed: bool,
    inhibited: bool,
}

impl Pda {
    const ALERT_SOURCE: ClassC_Source = ClassC_Source::Pda;
    const ALERT: <Self as TawsFunctionality>::Alert =
        <Self as TawsFunctionality>::Alert::new(Self::ALERT_SOURCE, AlertLevel::Caution);
}

impl TawsFunctionality for Pda {
    type AlertSource = ClassC_Source;
    type Alert = Alert<Self::AlertSource>;

    fn alert_source(&self) -> Self::AlertSource {
        Self::ALERT_SOURCE
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
        state: &NormalizedAircraftState,
    ) -> Result<Option<Alert<ClassC_Source>>, &'static dyn TawsError> {
        let altitude_gnd_foot = state.altitude_ground().get::<length::foot>();
        let distance_to_nearest_airport_nm = 3.0; //ToDo

        self.armed = distance_to_nearest_airport_nm <= 5.0;
        if !self.armed {
            return Ok(None);
        }

        let result = CAUTION_ENVELOPE
            .contains(distance_to_nearest_airport_nm, altitude_gnd_foot)?
            .then_some(Self::ALERT);

        if self.inhibited {
            return Ok(None);
        }

        Ok(result)
    }
}

lazy_static! {
    static ref LIMITS: Rect = Rect::new(
        Vector2::zeros(),
        // Max. distance to runway: Great circle distance (0, 0) deg to (0, 180) deg = 10819.33045 nm
        // Max altitude: 100km = 328084 ft
        Vector2::new(11_000.0, 330_000.0)
    );

    // Envelopes enlarged by d=0.1, to prevent floating pointing problems.
    static ref CAUTION_ENVELOPE: Envelope::<6> = Envelope::new(
        *LIMITS,
        &[
            Vector2::new(5.000,   9.900),
            Vector2::new(0.929,   9.929),
            Vector2::new(0.900,  80.001),
            Vector2::new(1.700, 150.002),
            Vector2::new(2.230, 170.072),
            Vector2::new(5.000, 170.100)
        ]
    ).unwrap();
}
