use geo::Point;
use geo::prelude::*;
use uom::si::angle::degree;
use uom::si::f64::{Angle, Length, Time};
use uom::si::length::{foot, meter};
use uom::si::ratio::ratio;
use taws_primitives::Position;
use crate::prediction::HeightPrediction;
use rayon::prelude::*;

pub trait TerrainDatabase {
    /// Estimate elevation at a specific position
    fn elevation_at(&self, long: Angle, lat: Angle) -> Option<Length>;
    fn clearance(&self, prediction: HeightPrediction, start: Position, bearing: Angle, check_duration: Time, resolution: Length) -> Option<Length>;
}

pub struct SimpleTerrainDatabase<const SIZE_LONG: usize, const SIZE_LAT: usize> {
    /// 2D-Array for elevation data. Unit is foot
    data: [[u64; SIZE_LAT]; SIZE_LONG],
    /// Start position of the database. Unit is Degree
    start: Point<f64>,
    /// Step in long/x and lat/y direction for each value in data. Unit is Degree
    step: Point<f64>,
}

impl<const SIZE_LONG: usize, const SIZE_LAT: usize> TerrainDatabase
    for SimpleTerrainDatabase<SIZE_LONG, SIZE_LAT>
{
    fn elevation_at(&self, long: Angle, lat: Angle) -> Option<Length> {
        let input = (long, lat);
        let start = (
            Angle::new::<degree>(self.start.lng()),
            Angle::new::<degree>(self.start.lat()),
        );
        let step = (
            Angle::new::<degree>(self.step.lng()),
            Angle::new::<degree>(self.step.lat()),
        );

        let index_long: [i64; 2] = {
            let index = ((input.0 - start.0) / step.0).get::<ratio>();
            [index.floor() as i64, index.ceil() as i64]
        };
        let index_lat: [i64; 2] = {
            let index = ((input.1 - start.1) / step.1).get::<ratio>();
            [index.floor() as i64, index.ceil() as i64]
        };

        let mut max = None;
        for i_long in index_long {
            for i_lat in index_lat {
                if i_long >= 0 && i_lat >= 0 {
                    max = max.max(
                        self.data
                            .get(i_long as usize)
                            .map(|l| l.get(i_lat as usize))
                            .flatten(),
                    )
                }
            }
        }

        max.map(|m| Length::new::<foot>(*m as f64))
    }

    fn clearance(&self, prediction: HeightPrediction, start: Position, bearing: Angle, check_duration: Time, resolution: Length) -> Option<Length> {
        let start = Point::new(start.lon.get::<degree>(), start.lat.get::<degree>());
        let end = start.haversine_destination(bearing.get::<degree>(), prediction.distance_by_time(check_duration).get::<meter>());

        let points = start.haversine_intermediate_fill(&end, resolution.get::<meter>(), true);
        points.par_iter()
            .filter_map(|p| self.elevation_at(Angle::new::<degree>(p.lng()), Angle::new::<degree>(p.lat())))
            .filter(|c| c.is_finite())
            .min_by(|x, y| x.partial_cmp(y).unwrap())
    }
}
