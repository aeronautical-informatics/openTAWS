/* use lazy_static::lazy_static;
use uom::{
    num_traits::Zero,
    si::f64::{Angle, Length},
};

use crate::{
    geodetic::{Coordinates, Epsg4979, NormalizedCoordinates},
    wgs84::{ECCENTRICITY_SQUARED, SEMI_MAJOR_AXIS},
};

lazy_static! {
    pub static ref ECEF: Ecef = Ecef::new();
}

#[derive(Debug)]
pub struct Ecef {
    origin: NormalizedCoordinates<Epsg4979>,
}

impl Ecef {
    fn new() -> Self {
        lazy_static! {
            static ref ORIGIN: NormalizedCoordinates<Epsg4979> = Coordinates::<Epsg4979>::new(
                Angle::zero(),
                Angle::zero(),
                -1.0 * (*SEMI_MAJOR_AXIS)
            )
            .normalize();
        }

        Self { origin: *ORIGIN }
    }

    pub fn origin(&self) -> NormalizedCoordinates<Epsg4979> {
        self.origin
    }

    pub fn transform(coords: NormalizedCoordinates<Epsg4979>) -> Vector3<'static, Length, Ecef> {
        let sin_latitude = coords.latitude().sin().get::<uom::si::ratio::ratio>();
        let cos_latitude = coords.latitude().cos().get::<uom::si::ratio::ratio>();

        let sin_longitude = coords.longitude().sin().get::<uom::si::ratio::ratio>();
        let cos_longitude = coords.longitude().cos().get::<uom::si::ratio::ratio>();

        let eccentricity_squared = (*ECCENTRICITY_SQUARED).get::<uom::si::ratio::ratio>();
        let semi_major_axis = *SEMI_MAJOR_AXIS;

        let n = semi_major_axis / (1.0 - (eccentricity_squared * sin_latitude.powi(2))).sqrt();

        let x = (n + coords.altitude_ellipsoid()) * cos_latitude * cos_longitude;
        let y = (n + coords.altitude_ellipsoid()) * cos_latitude * sin_longitude;
        let z = (((1.0 - eccentricity_squared) * n) + coords.altitude_ellipsoid()) * sin_latitude;

        Vector3 {
            vector: nalgebra::Vector3::new(x, y, z),
            crs: &ECEF,
        }
    }
}

pub struct Enu {
    origin: NormalizedCoordinates<Epsg4979>,
    origin_ecef: Vector3<'static, Length, Ecef>,
    transform: nalgebra::IsometryMatrix3<f64>,
}

impl Enu {
    pub fn new(origin: NormalizedCoordinates<Epsg4979>) -> Self {
        lazy_static! {
            static ref QUARTER_ROTATION: Angle = Angle::new::<uom::si::angle::revolution>(0.25);
        }

        let origin_ecef = Ecef::transform(origin);

        let r1 = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Vector3::x_axis(),
            (*QUARTER_ROTATION - origin.latitude()).get::<uom::si::angle::radian>(),
        );

        let r3 = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Vector3::z_axis(),
            (*QUARTER_ROTATION + origin.longitude()).get::<uom::si::angle::radian>(),
        );

        let t = nalgebra::Translation3::new(0.0, 0.0, 0.0);

        let transform = nalgebra::IsometryMatrix3::from_parts(t, r1 * r3);

        Self {
            origin,
            origin_ecef,
            transform,
        }
    }

    //pub fn transform(&self, coords: NormalizedCoordinates<Epsg4979>) -> Vector3<Length, Enu> {}
}

pub struct Vector3<'a, T, CRS> {
    vector: nalgebra::Vector3<T>,
    crs: &'a CRS,
}

/*impl<'a, CRS> Vector3<'a, Length, CRS> {
    fn get<N>(&self) -> nalgebra::Vector3<f64>
    where
        N: length::Conversion<f64>,
    {
        let x = self.vector.x.get::<N>();
        let y = self.vector.y.get::<N>();
        let z = self.vector.z.get::<N>();

        nalgebra::Vector3::new(x, y, z)
    }
}

impl<'a, CRS> Vector3<'a, Velocity, CRS> {
    fn get<N>(&self) -> nalgebra::Vector3<f64>
    where
        N: velocity::Conversion<f64>,
    {
        let x = self.vector.x.get::<N>();
        let y = self.vector.y.get::<N>();
        let z = self.vector.z.get::<N>();

        nalgebra::Vector3::new(x, y, z)
    }
}

impl<'a, CRS> Vector3<'a, Acceleration, CRS> {
    fn get<N>(&self) -> nalgebra::Vector3<f64>
    where
        N: acceleration::Conversion<f64>,
    {
        let x = self.vector.x.get::<N>();
        let y = self.vector.y.get::<N>();
        let z = self.vector.z.get::<N>();

        nalgebra::Vector3::new(x, y, z)
    }
}*/

impl Vector3<'static, Length, Ecef> {
    pub fn from_coordinates(coords: NormalizedCoordinates<Epsg4979>) -> Self {
        Ecef::transform(coords)
    }
}

/*impl<'a> Vector3<'a, Length, Enu> {
    pub fn from_coordinates(crs: &'a Enu, coords: NormalizedCoordinates<Epsg4979>) -> Self {
        crs.transform(coords)
    }
}*/
 */
