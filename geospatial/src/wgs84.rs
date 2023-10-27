use lazy_static::lazy_static;
use uom::{
    si::{f64::Length, f64::Ratio, length::meter, ratio::ratio},
    typenum::P2,
};

lazy_static! {
    pub static ref ONE: Ratio = Ratio::new::<ratio>(1.0);

    // WGS 84 Defining constants
    pub static ref SEMI_MAJOR_AXIS: Length = Length::new::<meter>(6_378_137.0);
    pub static ref INVERSE_FLATTENING: Ratio = Ratio::new::<ratio>(298.257223563);

    // WGS 84 Derived constants
    pub static ref SEMI_MINOR_AXIS: Length = (*ONE - (*ONE / *INVERSE_FLATTENING)) * (*SEMI_MAJOR_AXIS);
    pub static ref AXIS_RATIO: Ratio = *SEMI_MINOR_AXIS / *SEMI_MAJOR_AXIS;
    pub static ref AXIS_RATIO_SQUARED: Ratio = (AXIS_RATIO).powi(P2::new());
    pub static ref ECCENTRICITY_SQUARED: Ratio = *ONE - *AXIS_RATIO_SQUARED;
    pub static ref ECCENTRICITY: Ratio = (ECCENTRICITY_SQUARED).sqrt();
}
