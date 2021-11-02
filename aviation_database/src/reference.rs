use crate::{AirportDatabase, Runway};

#[derive(Debug, Default)]
pub struct AirportDatabaseImpl;

impl AirportDatabase for AirportDatabaseImpl {
    type RunwayIterator = core::iter::Empty<Runway>;
}
