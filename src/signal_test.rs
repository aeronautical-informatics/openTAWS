//! This file shall provide filtering

use crate::prelude::*;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use uom::si::ratio::{ratio, Ratio};

// TODO: What is wrong, if two values do not fit together?
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnplausibleSignal {
    AltitudeGround,
    AltitudeSea,
    SpeedAir,
    SpeedGround,
    NotEnoughData,
    //InternalError,
    /// Time stamp is either equally aged or older than the last
    Anachronistic,
}

#[derive(Debug)]
pub struct SignalTest {
    states: AllocRingBuffer<AircraftState>,
}

impl SignalTest {
    pub fn new(config: &TAWSConfig) -> Self {
        Self {
            states: AllocRingBuffer::with_capacity(16),
        }
    }

    pub fn check(
        &mut self,
        aircraft_state: AircraftState,
    ) -> Result<AircraftState, UnplausibleSignal> {
        // push new aircraft states
        self.states.push(aircraft_state);

        // check if enough aircraft states where gathered
        if self.states.len() < self.states.capacity() {
            return Err(UnplausibleSignal::NotEnoughData);
        }

        // check if the new aircraft state goes forward in time
        let dt = self.states.get(-2).unwrap().timestamp - self.states.get(-1).unwrap().timestamp;
        if Time::new::<second>(0.0) >= dt {
            return Err(UnplausibleSignal::Anachronistic);
        }

        let last_state = self.states.get(-2).unwrap();
        let current_state = self.states.get(-1).unwrap();

        // check if altitude ground is negative
        if current_state.altitude_ground < Length::new::<foot>(0.0) {
            return Err(UnplausibleSignal::AltitudeGround);
        }

        // check if height change is plausible
        let max_speed = last_state.speed_ground.max(current_state.speed_ground);
        let d_altitude = current_state.altitude - last_state.altitude;
        //if d_altitude > Ratio::new::<ratio>(1.5) * dt * max_speed  {
        //    return Err(UnplausibleSignal::AltitudeSea);
        //}

        Ok(self.states.get(-1).unwrap().clone())
    }
}
