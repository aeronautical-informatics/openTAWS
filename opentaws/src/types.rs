use core::{
    fmt,
    ops::{Add, Rem},
};

use uom::{
    fmt::DisplayStyle::Abbreviation,
    si::f64::*,
    si::{
        acceleration::foot_per_second_squared,
        angle::{degree, revolution},
        length::foot,
        time::second,
        velocity::{foot_per_minute, knot},
    },
};

use aviation_database::{reference::AirportDatabaseImpl, AirportDatabase, Position, Runway};

/// Represents the current state of an aircraft
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AircraftState {
    /// Time when this aircraft state was emitted
    pub timestamp: Time,

    /// Height above sea level in foot
    pub altitude: Length,

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

    ///  Aircraft speed as measured by GPS
    pub speed_ground: Velocity,

    /// Airspeed as measured by pressure sensors
    pub speed_air: Velocity,

    /// Angle in degrees (clockwise) between north and the direction where the
    /// aircrafts nose is pointing
    pub heading: Angle,

    /// The angle on the pitch axis. A positive value means the aircraft's nose points upwards
    /// compared to the horizon.
    pub pitch: Angle,

    /// The angle on the roll axis. A positive value means the aircraft's left wing points upwards
    /// while the right wing points downwards compared to the horizon. Another way of phrasing it:
    /// a positive value means the aircraft is rotated clockwise (as seen from behind).
    pub roll: Angle,

    /// Whether steep approach is selected
    pub steep_approach: bool,

    /// Whether precision approach is selected
    pub precision_approach: bool,

    /// Whether go around is selected
    pub go_around: bool,

    /// Whether take-off is selected
    pub take_off: bool,
}

/// This configuration holds various details about the aircraft in use. These are necessary for
/// example when estimating path trajectories for FLTA.
///
/// TODO adjust doc string to reflect the existence of terrainserver
#[derive(Clone, Debug)]
pub struct TawsConfig<'a> {
    pub terrain_server: &'a dyn AirportDatabase<RunwayIterator = core::iter::Empty<Runway>>,

    pub max_climbrate: Velocity,
    pub max_climbrate_change: Acceleration,
}

impl AircraftState {
    #[allow(unused)]
    /// Normalizes an `AircraftState`. Only normalized `AircraftStates` should be fed to the TAWS.
    pub(crate) fn normalize(&mut self) {
        let one_revolution = Angle::new::<revolution>(1.0);
        let half_revolution = Angle::new::<revolution>(0.5);
        let quarter_revolution = Angle::new::<revolution>(0.25);

        self.heading = Self::modulo(self.heading, one_revolution);

        self.roll = Self::modulo(self.roll + half_revolution, one_revolution) - half_revolution;
        self.pitch = Self::modulo(self.pitch + half_revolution, one_revolution) - half_revolution;

        self.position_lat = Self::modulo(self.position_lat + quarter_revolution, half_revolution)
            - quarter_revolution;
        self.position_lon =
            Self::modulo(self.position_lon + half_revolution, one_revolution) - half_revolution;
    }

    #[allow(unused)]
    pub(crate) fn check(&self) {
        let zero = Angle::new::<revolution>(0.0);
        let one_revolution = Angle::new::<revolution>(1.0);
        let half_revolution = Angle::new::<revolution>(0.5);
        let quarter_revolution = Angle::new::<revolution>(0.25);

        (zero..=one_revolution).contains(&self.heading);

        (-half_revolution..=half_revolution).contains(&self.roll);
        (-half_revolution..=half_revolution).contains(&self.pitch);

        (-quarter_revolution..=quarter_revolution).contains(&self.position_lat);
        (-half_revolution..=half_revolution).contains(&self.position_lon);
    }

    pub fn modulo<T: Copy + Add<Output = T> + Rem<Output = T>>(a: T, b: T) -> T {
        ((a % b) + b) % b
    }

    pub fn position(&self) -> Position {
        Position {
            lat: self.position_lat,
            lon: self.position_lon,
            alt: self.altitude_ground,
        }
    }
}

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
            altitude_sea = ft.with(self.altitude),
            altitude_ground = ft.with(self.altitude_ground),
            climb_rate = fpm.with(self.climb_rate),
            position_lat = dg.with(self.position_lat),
            position_lon = dg.with(self.position_lon),
            heading = dg.with(self.heading),
            speed = kt.with(self.speed_ground),
            pitch_angle = dg.with(self.pitch),
            roll_angle = dg.with(self.roll),
            steep_approach = self.steep_approach,
        )
    }
}

impl<'a> Default for TawsConfig<'a> {
    fn default() -> Self {
        static AIRPORT_DATABASE: AirportDatabaseImpl = AirportDatabaseImpl {};
        Self {
            terrain_server: &AIRPORT_DATABASE,
            max_climbrate: Velocity::new::<foot_per_minute>(700.0),
            max_climbrate_change: Acceleration::new::<foot_per_second_squared>(100.0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use uom::si::length::foot;

    const EPS: f64 = 1e-10;

    #[test]
    fn negative_altitude() {
        let mut state = AircraftState::default();
        state.altitude_ground = Length::new::<foot>(-12.0);
    }

    #[test]
    fn normalize_angle_below_zero() {
        let mut aircraft_state = AircraftState::default();
        aircraft_state.heading = Angle::new::<degree>(-1.0);
        aircraft_state.normalize();
        assert_eq!(aircraft_state.heading, Angle::new::<degree>(359.0));
    }

    #[test]
    fn normalize_angle_far_below_zero() {
        let mut aircraft_state = AircraftState::default();
        aircraft_state.heading = Angle::new::<degree>(-1024.0);
        aircraft_state.normalize();
        assert!((aircraft_state.heading - Angle::new::<degree>(56.0)).get::<degree>() < EPS);
    }

    #[test]
    fn normalize_angle_far_above_zero() {
        let mut aircraft_state = AircraftState::default();
        aircraft_state.heading = Angle::new::<degree>(1024.0);
        aircraft_state.normalize();
        assert!((aircraft_state.heading - Angle::new::<degree>(304.0)).get::<degree>() < EPS);
    }
}
