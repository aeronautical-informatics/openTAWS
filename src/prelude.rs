//! The prelude is a collection of all traits and commonly used types in this crate
//!
//! For normal use of this crate it is sufficient to glob import only this moduel, e.g. `use
//! opentaws::prelude::*`.

pub use crate::{
    alerts::{Alert, AlertLevel, AlertState, AlertSystem},
    types::{AircraftState, Attitude, TAWSConfig},
    TAWS,
};

pub use uom::si::{
    acceleration::foot_per_second_squared,
    angle::degree,
    f64::*,
    length::foot,
    velocity::{foot_per_minute, knot},
};
