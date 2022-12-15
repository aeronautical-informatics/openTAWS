//! This is a proof of concept TAWS as described in DO-367. It is not even close to fulfilling
//! DO-367 C, the simplest TAWS class. It exists to learn about using BDD (Cucumber & Gherkin in
//! particular) for implementing avionic.
//!
//! # Using openTAWS
//!
//! Currently it is only possible to use openTAWS from Rust. We've looked briefly into WASM-WASI
//! and C ABI as addiotional targets, but this did not lead anywehre usable _so far_. We are very
//! open to suggestions, so please open an issue if you have some feedback.

#![no_std]
#![deny(unsafe_code)]

pub use alerts::{functionalities, Alert, AlertLevel, AlertState};
pub use types::*;

#[macro_use]
pub mod macros;

mod alerts;
mod envelope;
pub mod prelude;
mod types;

pub trait Taws<const N: usize> {
    fn is_armed(&self, alert_system: Alert) -> bool;
    fn arm(&mut self, alert_system: Alert);
    fn disarm(&mut self, alert_system: Alert);
    fn is_inhibited(&self, alert_system: Alert) -> bool;
    fn inhibit(&mut self, alert_system: Alert);
    fn uninhibit(&mut self, alert_system: Alert);
    fn process(&mut self, state: &AircraftState) -> AlertState<N>;
}
