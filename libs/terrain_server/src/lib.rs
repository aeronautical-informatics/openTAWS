#[derive(Debug, PartialEq)]
pub struct GeograpraphicPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, PartialEq)]
pub enum TerrainServerStatus {
    OK,
    NoDatabase,
}

pub fn altitude_query() -> i64 {
    0
}

pub fn status() -> TerrainServerStatus {
    self::TerrainServerStatus::OK
}
