use crate::{alerts::*, envelope::*, prelude::*};

use super::ClassC_Source;

use ::uom::num_traits::Zero;
use lazy_static::lazy_static;
use nalgebra::Vector2;

use super::ClassCError;

#[derive(Clone, Debug)]
pub struct Mode3 {
    armed: bool,
    inhibited: bool,

    max_altitude_ground: Length,
}

impl Default for Mode3 {
    fn default() -> Self {
        Self {
            armed: false,
            inhibited: false,

            max_altitude_ground: Length::zero(),
        }
    }
}

impl Mode3 {
    const ALERT_SOURCE: ClassC_Source = ClassC_Source::Mode3;
    const ALERT: <Self as TawsFunctionality>::Alert =
        <Self as TawsFunctionality>::Alert::new(Self::ALERT_SOURCE, AlertLevel::Caution);
}

impl TawsFunctionality for Mode3 {
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
        let armed = state
            .situation()
            .map(|sit| matches!(sit, FlightSegment::TakeOff | FlightSegment::GoAround))
            .ok_or(ClassCError::InvalidAircraftState)?;

        if let (false, true) = (self.armed, armed) {
            self.max_altitude_ground = state.altitude_ground()
        }

        self.armed = armed;
        if !self.armed {
            return Ok(None);
        }

        self.max_altitude_ground = Length::max(self.max_altitude_ground, state.altitude_ground());

        let altitude_loss = (self.max_altitude_ground - state.altitude_ground()).get::<foot>();
        let rod = -state.climb_rate().get::<foot_per_minute>();
        let altitude_gnd = state.altitude_ground().get::<foot>();

        let method1_res = METHODE_1_CAUTION_ENVELOPE.contains(rod, altitude_gnd)?;
        let method2_res = METHODE_2_CAUTION_ENVELOPE.contains(altitude_loss, altitude_gnd)?;
        let result = (method1_res || method2_res).then_some(Self::ALERT);

        if self.inhibited {
            return Ok(None);
        }

        Ok(result)
    }
}

lazy_static! {
    static ref METHOD1_LIMITS: Rect = Rect::new(
        // Min/Max climb/descent rate: +-Mach 10 = +-675197.0 ft/min
        // Max altitude above terrain: 100km = 328084 ft
        Vector2::new(-680_000.0, 0.0),
        Vector2::new(680_000.0, 330_000.0)
    );

    static ref METHOD2_LIMITS: Rect = Rect::new(
        // Min/Max alitude loss: +-100km = +-328084 ft
        // Max altitude above terrain: 100km = 328084 ft
        Vector2::new(-330_000.0, 0.0),
        Vector2::new(330_000.0, 330_000.0)
    );

	// Envelopes enlarged by d=10, to prevent floating pointing problems.
    static ref METHODE_1_CAUTION_ENVELOPE: Envelope::<4> = Envelope::new(
        *METHOD1_LIMITS,
        &[
            Vector2::new(680000.000,  50.000),
            Vector2::new(   198.291,  55.085),
            Vector2::new(   528.085, 608.709),
            Vector2::new(680000.000, 610.000)
        ]
    ).unwrap();

    static ref METHODE_2_CAUTION_ENVELOPE: Envelope::<4> = Envelope::new(
        *METHOD2_LIMITS,
        &[
            Vector2::new(330000.000,  50.000),
            Vector2::new(    18.585,  53.290),
            Vector2::new(    73.290, 607.415),
            Vector2::new(330000.000, 610.000)
        ]
    ).unwrap();
}
