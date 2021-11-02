use crate::AirportEntry;
use kd_tree::{Node, Tree};

// Provides const AIRPORTS: &[AirportImpl] = ...
include!(concat!(env!("OUT_DIR"), "/airports.rs"));
