use crate::{
    aircraft_state::FlightSegment,
    envelope::{Envelope, INVALID_ENVELOPE},
    Alert, AlertLevel, AlertSource, TawsFunctionality,
};
use lazy_static::lazy_static;
use nalgebra::Vector2;
use uom::{
    num_traits::Zero,
    si::{f64::Length, length::foot, velocity::foot_per_minute},
};

pub struct Mode3 {
    alert_src: AlertSource,
    armed: bool,
    inhibited: bool,

    max_altitude_ground: Length,
}

impl Default for Mode3 {
    fn default() -> Self {
        Self {
            alert_src: AlertSource::Mode3,
            armed: false,
            inhibited: false,

            max_altitude_ground: Length::zero(),
        }
    }
}

impl TawsFunctionality for Mode3 {
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
            .map(|s| matches!(s, FlightSegment::TakeOff | FlightSegment::GoAround))
            .ok_or(())?;

        if let (false, true) = (self.armed, armed) {
            self.max_altitude_ground = state.altitude_ground()
        }

        self.armed = armed;
        if !self.armed {
            return Ok(None);
        }

        self.max_altitude_ground = Length::max(self.max_altitude_ground, state.altitude_ground());

        let alt_loss = (self.max_altitude_ground - state.altitude_ground()).get::<foot>();
        let altitude = state.altitude_ground().get::<foot>();
        let rod = -state.climb_rate().get::<foot_per_minute>();

        let result = if METHODE_1_CAUTION_ENVELOPE.contains(rod, altitude)?
            || METHODE_2_CAUTION_ENVELOPE.contains(alt_loss, altitude)?
        {
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
    static ref METHODE_1_CAUTION_ENVELOPE: Envelope::<4> = Envelope::try_new([
        Vector2::new(100_000.0, 60.0),
        Vector2::new(207.0, 60.0),
        Vector2::new(533.0, 600.0),
        Vector2::new(100_000.0, 600.0)
    ])
    .expect(INVALID_ENVELOPE);
    static ref METHODE_2_CAUTION_ENVELOPE: Envelope::<4> = Envelope::try_new([
        Vector2::new(100_000.0, 60.0),
        Vector2::new(26.0, 60.0),
        Vector2::new(80.0, 600.0),
        Vector2::new(100_000.0, 600.0)
    ])
    .expect(INVALID_ENVELOPE);
}
