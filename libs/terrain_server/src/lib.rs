//! This create offers all the TerrainServer functionality.
//!
//! It may be used as a decouple, free standing library. However, expect breakage as it is only
//! developed as interal contrib lib for otwas.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Re-export point
pub use geo::Point;

/// Some documentation
///
/// This is indeed a very nice Enum
#[derive(Debug, PartialEq)]
pub enum TerrainServerStatus {
    /// The server feels great!
    OK,

    /// The server feels bad, DB ghosted it
    NoDatabase,
}

/// Queries the current altitude of the vehicle. Yields meters above sealevel.
pub fn altitude_query() -> i64 {
    0
}

/// Struct which holds the whole state of a TerrainServer instance
///
/// To create a new one, enter this code:
///
/// ```
/// let ts = TerrainServer::default();
/// ```
#[derive(Default)]
pub struct TerrainServer;

impl TerrainServer {
    /// Returns the current Server State
    pub fn status(&self) -> TerrainServerStatus {
        self::TerrainServerStatus::OK
    }

    pub fn altitude (&self, point: geo::Point)->i64 {
        self::altitude_query()
    }
}
