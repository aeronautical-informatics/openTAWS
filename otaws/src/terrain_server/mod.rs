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
