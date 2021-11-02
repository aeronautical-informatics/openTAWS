//! The prelude is a collection of all traits and commonly used types in this crate
//!
//! For normal use of this crate it is sufficient to glob import only this moduel, e.g. `use
//! opentaws::prelude::*`.

pub use crate::{
    alerts::{functionalities, Alert, Alert::*, AlertLevel, AlertState, AlertSystem},
    types::{AircraftState, TawsConfig},
    Taws,
};

#[macro_use]
pub use crate::macros::*;

pub use uom::si::{
    acceleration::foot_per_second_squared,
    angle::degree,
    f64::*,
    length::foot,
    time::second,
    velocity::{foot_per_minute, knot},
};
