use crate::{alerts::*, prelude::*};

use super::{ClassCError, ClassC_Source};

use ::uom::num_traits::Zero;
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Ffac {
    armed: bool,
    inhibited: bool,

    last_altitude: Length,
}

impl Default for Ffac {
    fn default() -> Self {
        Self {
            armed: true,
            inhibited: false,
            last_altitude: Length::zero(),
        }
    }
}

impl Ffac {
    const ALERT_SOURCE: ClassC_Source = ClassC_Source::Ffac;
    const ALERT: <Self as TawsFunctionality>::Alert =
        <Self as TawsFunctionality>::Alert::new(Self::ALERT_SOURCE, AlertLevel::Annunciation);
}

impl TawsFunctionality for Ffac {
    type AlertSource = ClassC_Source;
    type Alert = Alert<Self::AlertSource>;
    //const ALERT_SOURCE: <Self::Alert as TawsAlert>::AlertSource = ClassC_Source::Ffac;

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
        let armed = state
            .situation()
            .map(|s| !matches!(s, FlightSegment::TakeOff | FlightSegment::GoAround))
            .ok_or(ClassCError::InvalidAircraftState)?;

        self.armed = armed;
        if !self.armed {
            return Ok(None);
        }

        let result = (self.last_altitude >= *FIVE_HUNDRED
            && state.altitude_ground() < *FIVE_HUNDRED)
            .then_some(Self::ALERT);

        self.last_altitude = state.altitude_ground();

        if self.inhibited {
            return Ok(None);
        }

        Ok(result)
    }
}

lazy_static! {
    static ref FIVE_HUNDRED: Length = Length::new::<foot>(500.0); //ToDo make const
}
