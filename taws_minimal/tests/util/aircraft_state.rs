use arbitrary::{Arbitrary, Unstructured};
use opentaws::prelude::*;
use rand::RngCore;

#[derive(Debug, Clone)]
struct AircraftStateWrapper(AircraftState);

impl<'a> Arbitrary<'a> for AircraftStateWrapper {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(AircraftStateWrapper(AircraftState {
            timestamp: Time::new::<second>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            altitude: Length::new::<foot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            altitude_ground: Length::new::<foot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            climb_rate: Velocity::new::<foot_per_minute>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            position_lat: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            position_lon: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            speed_ground: Velocity::new::<knot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            speed_air: Velocity::new::<knot>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            heading: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            pitch: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            roll: Angle::new::<degree>(<i32 as Arbitrary>::arbitrary(u)? as f64),
            steep_approach: u.arbitrary()?,
            precision_approach: u.arbitrary()?,
            go_around: u.arbitrary()?,
            take_off: u.arbitrary()?,
        }))
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

        Some(AircraftStateWrapper::arbitrary(&mut u).unwrap().0) // the unwrap is safe, we guarantee that enough bytes are available
    }
}
