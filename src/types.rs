pub use uom::si::{
    acceleration::foot_per_second_squared,
    angle::degree,
    length::foot,
    velocity::{foot_per_minute, knot},
};

use uom::si::f64::*;

#[derive(Clone, Debug, Default)]
pub struct Attitude {
    ///
    pitch: Angle,
    roll: Angle,
}

/// Structure describing the current state of an Aicraft
#[derive(Clone, Debug, Default)]
pub struct AircraftState {
    /// Time when this aircraft state was emitted
    pub timestamp: Time,

    /// Height above sealevel in foot
    pub altitude_sealevel: Length,

    /// Height above current terrain in foot
    pub altitude_ground: Length,

    /// Rate of descent
    pub climb_rate: Velocity,

    /// Geographic Latitude, specifying the north-south position
    pub position_lat: Angle,

    /// Geographic Longitude, specifying the east-west position
    pub position_lon: Angle,

    /// Angle in degrees (clockwise) between north and the direction to the
    /// destination or nav aid
    //pub bearing: degree,

    /// Angle in degrees (clockwise) between north and the direction where the
    /// aircrafts nose is pointing
    pub heading: Angle,

    /// Estimated aicraft speed
    pub speed: Velocity,

    /// Attitude of the aircraft including pitch & roll
    pub attitude: Attitude,

    /// Whether steep approach is selected
    pub steep_approach: bool,
}

impl std::panic::UnwindSafe for AircraftState {}

#[derive(Clone)]
pub struct TAWSConfig {
    pub max_climbrate: Velocity,
    pub max_climbrate_change: Acceleration,
}

impl Default for TAWSConfig {
    fn default() -> Self {
        Self {
            max_climbrate: Velocity::new::<foot_per_minute>(700.0),
            max_climbrate_change: Acceleration::new::<foot_per_second_squared>(100.0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn negative_altitude() {
        let mut state = AircraftState::default();
        state.altitude_ground = Length::new::<foot>(-12.0);
    }
}
