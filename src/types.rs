use std::fmt;

use uom::fmt::DisplayStyle::Abbreviation;

use uom::si::f64::*;
use uom::si::{
    acceleration::foot_per_second_squared,
    angle::degree,
    length::foot,
    time::second,
    velocity::{foot_per_minute, knot},
};

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

/// This configuration holds various details about the aircraft in use. These are necessary for
/// example when estimating path trajectories for FLTA.
#[derive(Clone, Debug)]
pub struct TAWSConfig {
    pub max_climbrate: Velocity,
    pub max_climbrate_change: Acceleration,
}

impl std::panic::UnwindSafe for AircraftState {}

impl fmt::Display for AircraftState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = Time::format_args(second, Abbreviation);
        let ft = Length::format_args(foot, Abbreviation);
        let dg = Angle::format_args(degree, Abbreviation);
        let fpm = Velocity::format_args(foot_per_minute, Abbreviation);
        let kt = Velocity::format_args(knot, Abbreviation);

        write!(
            f,
            "AircrafState:
  timestamp: {timestamp:.4}
  altitude_sea: {altitude_sea:.2}
  altitude_ground: {altitude_ground:.2}
  climb_rate: {climb_rate:.2}
  position_lat: {position_lat:.7}
  position_lon: {position_lon:.7}
  heading: {heading:.2}
  speed: {speed:.2}
  pitch_angle: {pitch_angle:.2}
  roll_angle: {roll_angle:.2}
  steep_approach: {steep_approach}\n",
            timestamp = s.with(self.timestamp),
            altitude_sea = ft.with(self.altitude_sea),
            altitude_ground = ft.with(self.altitude_ground),
            climb_rate = fpm.with(self.climb_rate),
            position_lat = dg.with(self.position_lat),
            position_lon = dg.with(self.position_lon),
            heading = dg.with(self.heading),
            speed = kt.with(self.speed),
            pitch_angle = dg.with(self.attitude.pitch),
            roll_angle = dg.with(self.attitude.roll),
            steep_approach = self.steep_approach,
        )
    }
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

    use uom::si::length::foot;

    #[test]
    pub fn negative_altitude() {
        let mut state = AircraftState::default();
        state.altitude_ground = Length::new::<foot>(-12.0);
    }
}
