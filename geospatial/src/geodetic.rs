use core::{
    marker::PhantomData,
    ops::{Add, Rem},
};

use uom::{
    num_traits::Zero,
    si::{
        angle::revolution,
        f64::{Angle, Length},
        ratio::ratio,
    },
};

pub type NormalizedCoordinates<T> = Coordinates<T, Normalized>;

#[derive(Copy, Clone, Default, Debug)]
pub struct Epsg4326;

#[derive(Copy, Clone, Default, Debug)]
pub struct Epsg4979;

#[derive(Copy, Clone, Default, Debug)]
pub struct Epsg9518;

#[derive(Copy, Clone, Default, Debug)]
pub struct Normalized;

#[derive(Copy, Clone, Default, Debug)]
pub struct Coordinates<T, NORM = ()> {
    latitude: Angle,
    longitude: Angle,
    altitude: Length,
    _phantom: PhantomData<(T, NORM)>,
}

impl<T, NORM> Coordinates<T, NORM> {
    pub fn latitude(&self) -> Angle {
        self.latitude
    }

    pub fn longitude(&self) -> Angle {
        self.longitude
    }
}

impl<NORM> Coordinates<Epsg4979, NORM> {
    pub fn altitude_ellipsoid(&self) -> Length {
        self.altitude
    }
}

impl<NORM> Coordinates<Epsg9518, NORM> {
    pub fn altitude_geoid(&self) -> Length {
        self.altitude
    }
}

impl<T> Coordinates<T, ()> {
    pub fn latitude_mut(&mut self) -> &mut Angle {
        &mut self.latitude
    }

    pub fn longitude_mut(&mut self) -> &mut Angle {
        &mut self.longitude
    }
}

impl Coordinates<Epsg4979, ()> {
    pub fn altitude_ellipsoid_mut(&mut self) -> &mut Length {
        &mut self.altitude
    }
}

impl Coordinates<Epsg9518, ()> {
    pub fn altitude_geoid_mut(&mut self) -> &mut Length {
        &mut self.altitude
    }
}

impl Coordinates<Epsg4326> {
    pub fn new(latitude: Angle, longitude: Angle) -> Self {
        Self {
            latitude,
            longitude,
            altitude: Length::zero(),
            _phantom: PhantomData,
        }
    }
}

impl Coordinates<Epsg4979> {
    pub fn new(latitude: Angle, longitude: Angle, altitude_ellipsoid: Length) -> Self {
        Self {
            latitude,
            longitude,
            altitude: altitude_ellipsoid,
            _phantom: PhantomData,
        }
    }
}

impl Coordinates<Epsg9518> {
    pub fn new(latitude: Angle, longitude: Angle, altitude_geoid: Length) -> Self {
        Self {
            latitude,
            longitude,
            altitude: altitude_geoid,
            _phantom: PhantomData,
        }
    }
}

impl<T> Coordinates<T, ()> {
    pub fn normalize(&self) -> Coordinates<T, Normalized> {
        let (latitude, longitude) = Self::wrap_lat_lon(self.latitude, self.longitude);

        Coordinates {
            latitude,
            longitude,
            altitude: self.altitude,
            _phantom: PhantomData,
        }
    }

    fn wrap_lat_lon(lat: Angle, lon: Angle) -> (Angle, Angle) {
        // ToDo: turn into consts when uom supports const new
        lazy_static::lazy_static! {
            static ref ZERO_REVOLUTION: Angle = Angle::new::<revolution>(0.0);
            static ref QUARTER_REVOLUTION: Angle = Angle::new::<revolution>(0.25);
            static ref HALF_REVOLUTION: Angle = Angle::new::<revolution>(0.5);
            static ref ONE_REVOLUTION: Angle = Angle::new::<revolution>(1.0);
        }

        let quadrant = ((lat.abs() / *QUARTER_REVOLUTION).get::<ratio>().floor() as i64) % 4;

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

    fn modulo<U: Copy + Add<Output = U> + Rem<Output = U>>(a: U, b: U) -> U {
        ((a % b) + b) % b
    }
}

#[cfg(test)]
mod test {
    use uom::si::angle::degree;

    use super::*;

    const EPS: f64 = 1e-10;

    macro_rules! wrap_position_tests {
        ($($name:ident: $pos:expr => $expected:expr),*) => {$(
            #[test]
            fn $name() {
                let mut c = Coordinates::<Epsg4326, ()>::default();
                *c.latitude_mut() = Angle::new::<degree>($pos.0);
                *c.longitude_mut() = Angle::new::<degree>($pos.1);

                let c = c.normalize();

				assert!(angle_approx_eq(c.latitude(), Angle::new::<degree>($expected.0)));
				assert!(angle_approx_eq(c.longitude(), Angle::new::<degree>($expected.1)));
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

    fn angle_approx_eq(value: Angle, target: Angle) -> bool {
        (target - value).abs() < Angle::new::<degree>(EPS)
    }
}
