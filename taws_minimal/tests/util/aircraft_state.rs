use std::marker::PhantomData;

use arbitrary::{Arbitrary, Unstructured};
use opentaws::prelude::*;
use rand::RngCore;

use super::constraints::{AircraftStateConstraints, BouncingClamp};

#[derive(Debug, Clone)]
struct AircraftStateWrapper(AircraftState);

impl<'a> Arbitrary<'a> for AircraftStateWrapper {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut state = AircraftState::default();

        *state.timestamp_mut() = Time::new::<time_second>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.position_latitude_mut() =
            Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.position_longitude_mut() =
            Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.altitude_mut() = Length::new::<foot>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.altitude_ground_mut() =
            Length::new::<foot>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.speed_ground_mut() = Velocity::new::<knot>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.speed_air_mut() = Velocity::new::<knot>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.climb_rate_mut() =
            Velocity::new::<foot_per_minute>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.heading_mut() = Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.track_mut() = Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64);
        *state.situation_mut() = Some(FlightSegment::Cruise);

        Ok(AircraftStateWrapper(state))
    }

    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (std::mem::size_of::<AircraftStateWrapper>(), None)
    }
}

// AircraftState generator
type Prng = rand_pcg::Mcg128Xsl64;
pub struct AircraftStateGenerator(pub Prng);


impl Default for AircraftStateGenerator {
	fn default() -> Self {
		Self(Prng::new(0xcafef00dd15ea5e5))
	}
}

impl Iterator for AircraftStateGenerator {
    type Item = AircraftState;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes_needed = AircraftStateWrapper::size_hint(0).0;
        let mut buf = Vec::with_capacity(bytes_needed);
        while buf.len() < bytes_needed {
            buf.extend_from_slice(&self.0.next_u64().to_le_bytes());
        }
        let mut u = Unstructured::new(&buf);

		let mut state = AircraftStateWrapper::arbitrary(&mut u).unwrap().0; // the unwrap is safe, we guarantee that enough bytes are available
        Some(state)
    }
}
