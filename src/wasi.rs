use serde_json::to_string;
use wasm_bindgen::prelude::*;

use crate::alerts::AlertState;

/// altitude in feet
#[wasm_bindgen]
pub fn get_report(longitude: f32, latitiude: f32, altitude: f32) -> String {
    //alert.to_string()
    todo!();
}
