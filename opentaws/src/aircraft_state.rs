use core::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Rem},
};

use lazy_static::lazy_static;

use crate::prelude::{uom::fmt::DisplayStyle, *};

// ToDo: turn into consts when uom supports const new
lazy_static! {
    static ref ZERO_REVOLUTION: Angle = Angle::new::<angle::revolution>(0.0);
    static ref QUARTER_REVOLUTION: Angle = Angle::new::<angle::revolution>(0.25);
    static ref HALF_REVOLUTION: Angle = Angle::new::<angle::revolution>(0.5);
    static ref ONE_REVOLUTION: Angle = Angle::new::<angle::revolution>(1.0);
    static ref ZERO_LENGTH: Length = Length::new::<length::foot>(0.0);
}

/// Marker for normalized AircraftStates
pub struct Normalized;

/// Represents a normalized AircraftState.
/// A normalized AircraftState satisfies various constraints:
/// * Latitude is in [[-90 .. 90]] deg
/// * Longitude is in [[-180 ... 180]] deg
/// * Altitude above ground >= 0 foot
/// * heading is in [0 .. 360) deg
/// * track is in [0 .. 360) deg
pub type NormalizedAircraftState = AircraftState<Normalized>;

/// Represents the current state of an aircraft
#[derive(Clone, Debug, Default)]
pub struct AircraftState<T = ()> {
    /// Time when this aircraft state was emitted
    timestamp: Time,

    /// Geographic Latitude, specifying the north-south position, WGS84
    position_lat: Angle,

    /// Geographic Longitude, specifying the east-west position, WGS84
    position_lon: Angle,

    /// Height above sea level in foot
    altitude: Length,

    /// Height above current terrain in foot
    altitude_ground: Length,

    ///  Aircraft speed as measured by GPS
    speed_ground: Velocity,

    /// Airspeed as measured by pressure sensors
    speed_air: Velocity,

    /// Rate of descent
    climb_rate: Velocity,

    /// Angle in degrees (clockwise) between true north and the direction where the
    /// aircrafts nose is pointing
    heading: Angle,

    /// Angle in degrees (clockwise) between true north and the direction of movement
    track: Angle,

    /// ToDo: Use Quaternions instead of Euler angles
    /// The angle on the pitch axis.
    /// The pitch axis has its origin at the center of gravity and is directed to the right, parallel to a line drawn from wingtip to wingtip.
    /// A positive angle points the nose up above the horizon and lowers the tail. (clockwise in direction of the axis)
    /// https://en.wikipedia.org/wiki/Aircraft_principal_axes#Transverse_axis_(pitch)
    /// pitch: Angle,

    /// The angle on the roll axis.
    /// The roll axis has its origin at the center of gravity and is directed forward, parallel to the fuselage reference line.
    /// A positive angle raises the left wing and lowers the right wing. (clockwise in direction of the axis)
    /// https://en.wikipedia.org/wiki/Aircraft_principal_axes#Longitudinal_axis_(roll)
    /// roll: Angle,

    /// The angle on the yaw axis.
    /// The yaw axis has its origin at the center of gravity and is directed towards the bottom of the aircraft, perpendicular to the wings and to the fuselage reference line.
    /// A positive angle points the nose to the right. (clockwise in direction of the axis)
    /// https://en.wikipedia.org/wiki/Aircraft_principal_axes#Vertical_axis_(yaw)
    /// yaw: Angle,

    /// The current situation (flight segment) of the aircraft (take-off, cruise, landing, ...)
    situation: Option<FlightSegment>,

    _phantom: PhantomData<T>,
}

impl AircraftState {
    pub fn new() -> Self {
        AircraftState::default()
    }

    // Setterss
    pub fn timestamp_mut(&mut self) -> &mut Time {
        &mut self.timestamp
    }

    pub fn position_latitude_mut(&mut self) -> &mut Angle {
        &mut self.position_lat
    }

    pub fn position_longitude_mut(&mut self) -> &mut Angle {
        &mut self.position_lon
    }

    pub fn altitude_mut(&mut self) -> &mut Length {
        &mut self.altitude
    }

    pub fn altitude_ground_mut(&mut self) -> &mut Length {
        &mut self.altitude_ground
    }

    pub fn speed_ground_mut(&mut self) -> &mut Velocity {
        &mut self.speed_ground
    }

    pub fn speed_air_mut(&mut self) -> &mut Velocity {
        &mut self.speed_air
    }

    pub fn climb_rate_mut(&mut self) -> &mut Velocity {
        &mut self.climb_rate
    }

    pub fn heading_mut(&mut self) -> &mut Angle {
        &mut self.heading
    }

    pub fn track_mut(&mut self) -> &mut Angle {
        &mut self.track
    }

    /* pub fn pitch_mut(&mut self) -> &mut Angle {
        &mut self.pitch
    }

    pub fn roll_mut(&mut self) -> &mut Angle {
        &mut self.roll
    }

    pub fn yaw_mut(&mut self) -> &mut Angle {
        &mut self.yaw
    } */

    pub fn situation_mut(&mut self) -> &mut Option<FlightSegment> {
        &mut self.situation
    }
}

impl<T> AircraftState<T> {
    // Getters
    pub fn timestamp(&self) -> Time {
        self.timestamp
    }

    pub fn position_latitude(&self) -> Angle {
        self.position_lat
    }

    pub fn position_longitude(&self) -> Angle {
        self.position_lon
    }

    pub fn altitude(&self) -> Length {
        self.altitude
    }

    pub fn altitude_ground(&self) -> Length {
        self.altitude_ground
    }

    pub fn speed_ground(&self) -> Velocity {
        self.speed_ground
    }

    pub fn speed_air(&self) -> Velocity {
        self.speed_air
    }

    pub fn climb_rate(&self) -> Velocity {
        self.climb_rate
    }

    pub fn heading(&self) -> Angle {
        self.heading
    }

    pub fn track(&self) -> Angle {
        self.track
    }

    /* pub fn pitch(&self) -> Angle {
        self.pitch
    }

    pub fn roll(&self) -> Angle {
        self.roll
    }

    pub fn yaw(&self) -> Angle {
        self.yaw
    } */

    pub fn situation(&self) -> &Option<FlightSegment> {
        &self.situation
    }

    /// Normalizes the [AircraftState] into a [NormalizedAircraftState]
    pub fn normalize(&self) -> NormalizedAircraftState {
        let (lat, lon) = Self::wrap_position(self.position_lat, self.position_lon);

        NormalizedAircraftState {
            timestamp: self.timestamp,

            position_lat: lat,
            position_lon: lon,

            altitude: self.altitude,
            altitude_ground: Self::clamp_altitude_ground(self.altitude_ground),

            speed_ground: self.speed_ground,
            speed_air: self.speed_air,

            climb_rate: self.climb_rate,

            heading: Self::wrap_compass_direction(self.heading),
            track: Self::wrap_compass_direction(self.track),

            // pitch: Angle::new::<degree>(0.0),
            // roll: Angle::new::<degree>(0.0),
            // yaw: Angle::new::<degree>(0.0),
            situation: self.situation.clone(),

            _phantom: PhantomData,
        }
    }

    fn wrap_position(lat: Angle, lon: Angle) -> (Angle, Angle) {
        use ::uom::num_traits::Float;
        let quadrant = ((lat.abs() / *QUARTER_REVOLUTION)
            .get::<ratio::ratio>()
            .floor() as i64)
            % 4;

        let pole = if lat > *ZERO_REVOLUTION {
            *QUARTER_REVOLUTION
        } else {
            -(*QUARTER_REVOLUTION)
        };

        let offset = lat % *QUARTER_REVOLUTION;

        let (lat, lon) = match quadrant {
            0 => (offset, lon),
            1 => (pole - offset, lon + *HALF_REVOLUTION),
            2 => (-offset, lon + *HALF_REVOLUTION),
            3 => (-pole + offset, lon),
            _ => panic!("cannot happen"),
        };

        let lon = Self::modulo(lon + *HALF_REVOLUTION, *ONE_REVOLUTION) - *HALF_REVOLUTION;
        (lat, lon)
    }

    fn clamp_altitude_ground(alt_gnd: Length) -> Length {
        Length::max(*ZERO_LENGTH, alt_gnd)
    }

    fn wrap_compass_direction(dir: Angle) -> Angle {
        Self::modulo(dir, *ONE_REVOLUTION)
    }

    fn modulo<U: Copy + Add<Output = U> + Rem<Output = U>>(a: U, b: U) -> U {
        ((a % b) + b) % b
    }
}

impl From<NormalizedAircraftState> for AircraftState {
    fn from(state: NormalizedAircraftState) -> Self {
        AircraftState {
            timestamp: state.timestamp,
            position_lat: state.position_lat,
            position_lon: state.position_lon,
            altitude: state.altitude,
            altitude_ground: state.altitude_ground,
            speed_ground: state.speed_ground,
            speed_air: state.speed_air,
            climb_rate: state.climb_rate,
            heading: state.heading,
            track: state.track,
            // pitch: state.pitch,
            // roll: state.roll,
            // yaw: state.yaw,
            situation: state.situation.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Display for AircraftState<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = Time::format_args(time::second, DisplayStyle::Abbreviation);
        let ft = Length::format_args(length::foot, DisplayStyle::Abbreviation);
        let deg = Angle::format_args(angle::degree, DisplayStyle::Abbreviation);
        let fpm = Velocity::format_args(velocity::foot_per_minute, DisplayStyle::Abbreviation);
        let kt = Velocity::format_args(velocity::knot, DisplayStyle::Abbreviation);

        write!(
            f,
            "
AircrafState: {{
  timestamp: {timestamp:.4},
  altitude_sea: {altitude_sea:.2},
  altitude_ground: {altitude_ground:.2},
  climb_rate: {climb_rate:.2},
  position_lat: {position_lat:.7},
  position_lon: {position_lon:.7},
  heading: {heading:.2},
  speed: {speed:.2},
  situation: {situation}
}}",
            timestamp = s.with(self.timestamp),
            altitude_sea = ft.with(self.altitude),
            altitude_ground = ft.with(self.altitude_ground),
            climb_rate = fpm.with(self.climb_rate),
            position_lat = deg.with(self.position_lat),
            position_lon = deg.with(self.position_lon),
            heading = deg.with(self.heading),
            speed = kt.with(self.speed_ground),
            situation = format_args!("{:?}", self.situation)
        )
    }
}

/// Represents a flight segment
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FlightSegment {
    /// The aircraft is in cruise flight situation
    Cruise,

    /// The aircraft is in a take-off situation
    TakeOff,

    // ToDo: Climb and Approach necessary?
    // Climb ???,
    // Approach ???,
    /// The aircraft is in a landing situation
    Landing {
        /// Determines whether a circling approach is flown
        circling_approach: bool, // Must be part of Approach

        /// Determines whether a precision approach (ILS) is flown
        precision_approach: bool, // Must be part of Approach and Landing

        /// Determines wheather a steep approach is flown
        steep_approach: bool, // Must be part of Approach and Landing
    },

    /// The aircraft is in a go around situation
    GoAround,
}

impl Display for FlightSegment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
			FlightSegment::TakeOff => write!(f, "Take-Off {{ }}"),
			FlightSegment::Cruise => write!(f, "Cruise {{ }}"),
			FlightSegment::Landing { circling_approach, precision_approach, steep_approach } =>
                write!(f, "Landing {{ circling: {circling_approach}, precision: {precision_approach}, steep: {steep_approach} }}"),
			FlightSegment::GoAround => write!(f, "Go Around {{ }}")
		}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EPS: f64 = 1e-10;

    macro_rules! wrap_position_tests {
        ($($name:ident: $pos:expr => $expected:expr),*) => {$(
            #[test]
            fn $name() {
                let mut state = AircraftState::new();
                *state.position_latitude_mut() = Angle::new::<angle::degree>($pos.0);
                *state.position_longitude_mut() = Angle::new::<angle::degree>($pos.1);

                let norm_state = state.normalize();

				assert!(angle_approx_eq(norm_state.position_latitude(), Angle::new::<angle::degree>($expected.0)));
				assert!(angle_approx_eq(norm_state.position_longitude(), Angle::new::<angle::degree>($expected.1)));
            })*
        };
    }

    wrap_position_tests! {
        test_wrap_zero_pos: (0.0, 0.0) => (0.0, 0.0),

        test_wrap_lat_1: (10.0, 0.0) => (10.0, 0.0),
        test_wrap_lat_2: (90.0, 0.0) => (90.0, -180.0),
        test_wrap_lat_3: (100.0, 0.0) => (80.0, -180.0),
        test_wrap_lat_4: (180.0, 0.0) => (0.0, -180.0),
        test_wrap_lat_5: (190.0, 0.0) => (-10.0, -180.0),
        test_wrap_lat_6: (270.0, 0.0) => (-90.0, 0.0),
        test_wrap_lat_7: (280.0, 0.0) => (-80.0, 0.0),
        test_wrap_lat_8: (360.0, 0.0) => (0.0, 0.0),
        test_wrap_lat_9: (370.0, 0.0) => (10.0, 0.0),
        test_wrap_lat_10: (450.0, 0.0) => (90.0, -180.0),

        test_wrap_lon_1: (0.0, 10.0) => (0.0, 10.0),
        test_wrap_lon_2: (0.0, 180.0) => (0.0, -180.0),
        test_wrap_lon_3: (0.0, 190.0) => (0.0, -170.0),
        test_wrap_lon_4: (0.0, 350.0) => (0.0, -10.0),
        test_wrap_lon_5: (0.0, 360.0) => (0.0, 0.0),
        test_wrap_lon_6: (0.0, 370.0) => (0.0, 10.0),
        test_wrap_lon_7: (0.0, 540.0) => (0.0, -180.0),

        test_wrap_lat_lon_1: (45.0, 45.0) => (45.0, 45.0),
        test_wrap_lat_lon_2: (90.0, 90.0) => (90.0, -90.0),
        test_wrap_lat_lon_3: (135.0, 135.0) => (45.0, -45.0),
        test_wrap_lat_lon_4: (180.0, 180.0) => (0.0, 0.0),
        test_wrap_lat_lon_5: (225.0, 225.0) => (-45.0, 45.0),
        test_wrap_lat_lon_6: (270.0, 270.0) => (-90.0, -90.0),
        test_wrap_lat_lon_7: (360.0, 360.0) => (0.0, 0.0),
        test_wrap_lat_lon_8: (405.0, 405.0) => (45.0, 45.0),
        test_wrap_lat_lon_9: (450.0, 450.0) => (90.0, -90.0)
    }

    #[test]
    fn test_clamp_pos_alt_gnd() {
        let mut state = AircraftState::new();
        *state.altitude_ground_mut() = Length::new::<length::foot>(1000.0);

        let norm_state = state.normalize();

        assert!(length_approx_eq(
            norm_state.altitude_ground(),
            Length::new::<length::foot>(1000.0)
        ))
    }

    #[test]
    fn test_clamp_neg_alt_gnd() {
        let mut state = AircraftState::new();
        *state.altitude_ground_mut() = Length::new::<length::foot>(-1000.0);

        let norm_state = state.normalize();

        assert!(length_approx_eq(
            norm_state.altitude_ground(),
            Length::new::<length::foot>(0.0)
        ))
    }

    macro_rules! wrap_heading_tests {
        ($($name:ident: $hdg:expr => $expected:expr),*) => {$(
            #[test]
            fn $name() {
                let mut state = AircraftState::new();
                *state.heading_mut() = Angle::new::<angle::degree>($hdg);

                let norm_state = state.normalize();

				assert!(angle_approx_eq(norm_state.heading(), Angle::new::<angle::degree>($expected)));
            })*
        };
    }

    wrap_heading_tests! {
        test_wrap_heading_1: 0.0 => 0.0,
        test_wrap_heading_2: 10.0 => 10.0,
        test_wrap_heading_3: 90.0 => 90.0,
        test_wrap_heading_4: 180.0 => 180.0,
        test_wrap_heading_5: 270.0 => 270.0,
        test_wrap_heading_6: 360.0 => 0.0,

        test_wrap_heading_7: -10.0 => 350.0,
        test_wrap_heading_8: -90.0 => 270.0,
        test_wrap_heading_9: -180.0 => 180.0,
        test_wrap_heading_10: -270.0 => 90.0,
        test_wrap_heading_11: -360.0 => 0.0
    }

    macro_rules! wrap_track_tests {
        ($($name:ident: $trk:expr => $expected:expr),*) => {$(
            #[test]
            fn $name() {
                let mut state = AircraftState::new();
                *state.track_mut() = Angle::new::<angle::degree>($trk);

                let norm_state = state.normalize();

				assert!(angle_approx_eq(norm_state.track(), Angle::new::<angle::degree>($expected)));
            })*
        };
    }

    wrap_track_tests! {
        test_wrap_track_1: 0.0 => 0.0,
        test_wrap_track_2: 10.0 => 10.0,
        test_wrap_track_3: 90.0 => 90.0,
        test_wrap_track_4: 180.0 => 180.0,
        test_wrap_track_5: 270.0 => 270.0,
        test_wrap_track_6: 360.0 => 0.0,

        test_wrap_track_7: -10.0 => 350.0,
        test_wrap_track_8: -90.0 => 270.0,
        test_wrap_track_9: -180.0 => 180.0,
        test_wrap_track_10: -270.0 => 90.0,
        test_wrap_track_11: -360.0 => 0.0
    }

    fn angle_approx_eq(value: Angle, target: Angle) -> bool {
        (target - value).abs() < Angle::new::<angle::degree>(EPS)
    }

    fn length_approx_eq(value: Length, target: Length) -> bool {
        (target - value).abs() < Length::new::<length::foot>(EPS)
    }
}
