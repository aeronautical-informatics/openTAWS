# opentaws

This is a proof of concept TAWS as described in DO-367. It is not even close to fulfilling
DO-367 C, the simplest TAWS class. It exists to learn about using BDD (Cucumber & Gherkin in
particular) for implementing avionic.

## Using openTAWS

Currently it is only possible to use openTAWS from Rust. We've looked briefly into WASM-WASI
and C ABI as addiotional targets, but this did not lead anywehre usable _so far_. We are very
open to suggestions, so please open an issue if you have some feedback.

## Contributing

Here is our current wishlist:

+ [ ] Provide a way for non Rust software to use openTAWS
+ [ ] Implement more from the standard
+ [ ] Add a shim-example for attaching openTAWS to FlightGear

License: MIT OR Apache-2.0
