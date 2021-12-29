use crate::prediction::HeightPrediction;
use geo::prelude::*;
use geo::Point;
use rayon::prelude::*;
use taws_primitives::Position;
use uom::si::angle::degree;
use uom::si::f64::{Angle, Length, Time};
use uom::si::length::{foot, meter};
use uom::si::ratio::ratio;

pub trait TerrainDatabase {
    /// Estimate elevation at a specific position
    fn elevation_at(&self, long: Angle, lat: Angle) -> Option<Length>;
    fn clearance(
        &self,
        prediction: HeightPrediction,
        start: Position,
        bearing: Angle,
        check_duration: Time,
        resolution: Length,
    ) -> Option<Length>;
}

pub struct SimpleTerrainDatabase<const SIZE_LONG: usize, const SIZE_LAT: usize> {
    /// 2D-Array for elevation data. Unit is foot
    data: [[u64; SIZE_LAT]; SIZE_LONG],
    /// Start position of the database. Unit is Degree
    start: Point<f64>,
    /// Step in long/x and lat/y direction for each value in data. Unit is Degree
    step: Point<f64>,
}

impl<const SIZE_LONG: usize, const SIZE_LAT: usize> SimpleTerrainDatabase<SIZE_LONG, SIZE_LAT> {
    pub const fn new(
        data: [[u64; SIZE_LAT]; SIZE_LONG],
        start: Point<f64>,
        step: Point<f64>,
    ) -> SimpleTerrainDatabase<SIZE_LONG, SIZE_LAT> {
        SimpleTerrainDatabase { data, start, step }
    }
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

        let index_long = ((input.0 - start.0) / step.0).get::<ratio>().round() as i64;
        let index_lat = ((input.1 - start.1) / step.1).get::<ratio>().round() as i64;

        if index_long < 0 || index_lat < 0 {
            return None;
        }

        self.data
            .get(index_long as usize)
            .map(|l| l.get(index_lat as usize))
            .flatten()
            .map(|m| Length::new::<foot>(*m as f64))
    }

    fn clearance(
        &self,
        prediction: HeightPrediction,
        start: Position,
        bearing: Angle,
        check_duration: Time,
        resolution: Length,
    ) -> Option<Length> {
        let start = Point::new(start.lon.get::<degree>(), start.lat.get::<degree>());
        let end = start.haversine_destination(
            bearing.get::<degree>(),
            prediction.distance_by_time(check_duration).get::<meter>(),
        );

        let points = start.haversine_intermediate_fill(&end, resolution.get::<meter>(), true);
        points
            //Iterate over all points in parallel
            .par_iter()
            // Map List other element but filter out "None" Values
            // "None" Values are only possible if our airplane exits the area covered by our terrain database
            .filter_map(|p| {
                // Calculate clearance by getting the elevation at this point
                self.elevation_at(Angle::new::<degree>(p.lng()), Angle::new::<degree>(p.lat()))
                    // If Height at this point exists
                    .map(|height| {
                        // Subtract height from predicted airplane height
                        prediction
                            // Get Height of Airplane by calculating the horizontal distance of our start point to our current point
                            // Then get the predicted height by asking our prediction function for the height at this distance
                            .height_by_distance(Length::new::<meter>(start.haversine_distance(p)))
                            - height
                    })
            })
            // Filter so only finite values exist (infinite [and NaN] values are only possible if the elevation database is broken)
            .filter(|c| c.is_finite())
            .min_by(|x, y| x.partial_cmp(y).unwrap())
    }
}
