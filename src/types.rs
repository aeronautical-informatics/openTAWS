use uom::si::f64::*;

/// Represents the attitude of an aircraft
#[derive(Clone, Debug, Default)]
pub struct Attitude {
    /// The angle on the pitch axis. A positive value means the aircraft's nose points upwards
    /// compared to the horizon.
    pub pitch: Angle,
    /// The angle on the roll axis. A positive value means the aircraft's left wing points upwards
    /// while the right wing points downwards compared to the horizon. Another way of phrasing it:
    /// a positive value means the aircraft is rotated clockwise (as seen from behind).
    pub roll: Angle,
}

/// Represents the current state of an aircraft
#[derive(Clone, Debug, Default)]
pub struct AircraftState {
    /// Time when this aircraft state was emitted
    pub timestamp: Time,

    /// Height above sea level in foot
    pub altitude_sea: Length,

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

/// This configuration holds various details about the aircraft in use. These are necessary for
/// example when estimating path trajectories for FLTA.
#[derive(Clone, Debug)]
pub struct TAWSConfig {
    pub max_climbrate: Velocity,
    pub max_climbrate_change: Acceleration,
}

use uom::si::{acceleration::foot_per_second_squared, velocity::foot_per_minute};

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

    use uom::si::length::foot;

    #[test]
    pub fn negative_altitude() {
        let mut state = AircraftState::default();
        state.altitude_ground = Length::new::<foot>(-12.0);
    }
}
