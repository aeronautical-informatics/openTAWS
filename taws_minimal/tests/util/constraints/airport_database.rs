use aviation_database::{AirportDatabase, Runway};

#[derive(Debug, Default)]
pub struct ConstraintAirportDatabase;

impl AirportDatabase for ConstraintAirportDatabase {
    type RunwayIterator = core::iter::Empty<Runway>;
}
