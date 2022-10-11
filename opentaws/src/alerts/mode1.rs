use crate::{
    aircraft_state::FlightSegment,
    envelope::{Envelope, INVALID_ENVELOPE},
    Alert, AlertLevel, AlertSource, TawsFunctionality,
};
use lazy_static::lazy_static;
use nalgebra::Vector2;
use uom::si::{length::foot, velocity::foot_per_minute};

#[derive(Clone, Debug)]
pub struct Mode1 {
    alert_src: AlertSource,
    armed: bool,
    inhibited: bool,
}

impl Default for Mode1 {
    fn default() -> Self {
        Self {
            alert_src: AlertSource::Mode1,
            armed: true,
            inhibited: false,
        }
    }
}

impl TawsFunctionality for Mode1 {
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
        let altitude = state.altitude_ground().get::<foot>();
        let rod = -state.climb_rate().get::<foot_per_minute>();
        let steep_approach = state
            .situation()
            .map(|s| match s {
                FlightSegment::Landing { steep_approach, .. } => *steep_approach,
                _ => false,
            })
            .ok_or(())?;

        let result = match steep_approach {
            true if STEEP_WARNING_ENVELOPE.contains(rod, altitude)? => {
                Ok(Some(Alert::new(self.alert_src, AlertLevel::Warning)))
            }
            true if STEEP_CAUTION_ENVELOPE.contains(rod, altitude)? => {
                Ok(Some(Alert::new(self.alert_src, AlertLevel::Caution)))
            }
            false if WARNING_ENVELOPE.contains(rod, altitude)? => {
                Ok(Some(Alert::new(self.alert_src, AlertLevel::Warning)))
            }
            false if CAUTION_ENVELOPE.contains(rod, altitude)? => {
                Ok(Some(Alert::new(self.alert_src, AlertLevel::Caution)))
            }
            _ => Err(()),
        };

        if !self.inhibited {
            result
        } else {
            Ok(None)
        }
    }
}

lazy_static! {
    static ref CAUTION_ENVELOPE: Envelope<5> = Envelope::try_new([
        Vector2::new(100_000.0, 100.0),
        Vector2::new(1560.0, 100.0),
        Vector2::new(2200.0, 630.0),
        Vector2::new(5700.0, 2200.0),
        Vector2::new(100_000.0, 2200.0)
    ])
    .expect(INVALID_ENVELOPE);
    static ref STEEP_CAUTION_ENVELOPE: Envelope<6> = Envelope::try_new([
        Vector2::new(100_000.0, 150.0),
        Vector2::new(1798.0, 150.0),
        Vector2::new(1944.0, 300.0),
        Vector2::new(3233.0, 1078.0),
        Vector2::new(6225.0, 2075.0),
        Vector2::new(100_000.0, 2075.0)
    ])
    .expect(INVALID_ENVELOPE);
    static ref WARNING_ENVELOPE: Envelope<5> = Envelope::try_new([
        Vector2::new(100_000.0, 100.0),
        Vector2::new(1600.0, 100.0),
        Vector2::new(1850.0, 300.0),
        Vector2::new(10100.0, 1958.0),
        Vector2::new(100_000.0, 1958.0)
    ])
    .expect(INVALID_ENVELOPE);
    static ref STEEP_WARNING_ENVELOPE: Envelope<5> = Envelope::try_new([
        Vector2::new(100_000.0, 150.0),
        Vector2::new(1908.0, 150.0),
        Vector2::new(2050.0, 300.0),
        Vector2::new(10300.0, 1958.0),
        Vector2::new(100_000.0, 1958.0)
    ])
    .expect(INVALID_ENVELOPE);
}
