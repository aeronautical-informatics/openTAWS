pub use self::uom::*;
pub use crate::aircraft_state::{AircraftState, FlightSegment, NormalizedAircraftState};
pub use crate::alerts::AlertLevel;
pub use crate::{
    Taws, TawsAlert, TawsAlertSource, TawsAlerts, TawsError, TawsFunctionalities, TawsFunctionality,
};

mod uom {
    pub use uom::si::f64::{Angle, Length, Time, Velocity};
    pub use uom::si::{
        angle::{degree, minute as angular_minute, revolution, second as angular_second},
        length::{foot, kilometer, meter, nautical_mile},
        time::second as time_second,
        velocity::{foot_per_minute, kilometer_per_hour, knot, meter_per_second},
    };
}
