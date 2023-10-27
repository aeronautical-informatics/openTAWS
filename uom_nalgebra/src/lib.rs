#![no_std]
#![deny(unsafe_code)]
#![allow(dead_code)]
//#![feature(trace_macros)]

#[cfg(test)]
#[macro_use]
extern crate std;

mod matrix_shapes;
mod quantity_matrix;
pub mod si;

pub use quantity_matrix::*;

pub(crate) use matrix_shapes::*;

#[cfg(test)]
mod tests {

    use uom::si::f64::{Area, Length};

    use crate::si::*;

    #[test]
    fn foo() {
        let v = length::LengthVector3::new::<length::meter>(1.0, 1.0, 1.0);

        println!("{:?}", v);
    }
}
