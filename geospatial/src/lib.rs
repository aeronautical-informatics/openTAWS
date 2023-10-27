#![no_std]
#![deny(unsafe_code)]
#![allow(dead_code)]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod cartesian;
pub mod geodetic;
pub mod wgs84;
