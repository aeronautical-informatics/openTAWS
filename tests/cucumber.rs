use std::{
    convert::Infallible,
    ops::{Add, Rem, Sub},
};

use async_trait::async_trait;
use rand::RngCore;

use uom::{
    num::Signed,
    si::{f64::*, length::foot, time::second, velocity::foot_per_minute},
};

use arbitrary::{Arbitrary, Unstructured};
use cucumber::Steps;
use opentaws::prelude::*;

fn main() {
    let runner = cucumber::Cucumber::<MyWorld>::new()
        .features(&["features"])
        .steps(steps());

    futures::executor::block_on(runner.run());
}

pub struct MyWorld {
    taws: Taws,
    mould_pipeline: Vec<Box<dyn FnMut(&mut AircraftState)>>,
    test_length: usize,
}

#[derive(Debug, Clone)]
struct AircraftStateWrapper(AircraftState);

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            taws: Taws::new(Default::default()),
            mould_pipeline: Vec::new(),
            test_length: 1000, // TODO is this a good number?
        })
    }
}

// convenience macro
macro_rules! pipeline_extend {
    ($world:expr, $closure:expr) => {
        $world.mould_pipeline.push(Box::new($closure));
    };
}

pub fn steps() -> Steps<crate::MyWorld> {
    let mut builder: Steps<crate::MyWorld> = Steps::new();
    builder
        .given("the plane is flying", |world, _step| world)
        .given_regex("^(.+) is armed$", |world, mut matches, _step| {
            matches[1].retain(|c| !c.is_whitespace());
            let alert_system = parse_alert(&matches[1]);
            //if matches[2].starts_with("not") {
            //    assert!(!world.taws.function_is_armed(&parse_alert));
            //} else {
            assert!(world.taws.is_armed(alert_system));
            //}
            world
        })
        .given_regex(
            "^(.+) is (.*)inhibited$",
            |mut world, mut matches, _step| {
                matches[1].retain(|c| !c.is_whitespace());
                let alert_system = parse_alert(&matches[1]);

                if matches[2].starts_with("not") {
                    world.taws.uninhibit(alert_system);
                } else {
                    world.taws.inhibit(alert_system);
                }
                world
            },
        )
        .given_regex(
            r"^steep approach is (.*)selected$",
            |mut world, matches, _step| {
                if matches[1].starts_with("not") {
                    pipeline_extend!(world, |a| a.steep_approach = false);
                } else {
                    pipeline_extend!(world, |a| a.steep_approach = true);
                }
                world
            },
        )
        .then_regex(r"^(.+) shall be armed$", |world, mut matches, _step| {
            matches[1].retain(|c| !c.is_whitespace());
            let alert_system = parse_alert(&matches[1]);
            assert!(world.taws.is_armed(alert_system));
            world
        })
        .when_regex(
            r"^the rate of descent is at (\w+) (\d+) feet per minute$",
            |mut world, matches, _step| {
                let rod = Velocity::new::<foot_per_minute>(matches[2].parse().unwrap());
                let mut bouncer = BouncingClamp();
                // most and least are swapped here, as aircraft_state stores rate of climb, while
                // the sentence give rate of descent 
                // TODO validate that this is a safe assumption?
                match matches[1].as_str() {
                    "most" => {
                        pipeline_extend!(world, move |a| bouncer.at_least(&mut a.climb_rate, -rod));
                    }
                    "least" => {
                        pipeline_extend!(world, move |a| bouncer.at_most(&mut a.climb_rate, -rod));
                    }
                    _ => {
                        panic!("unable to parse this sentence");
                    }
                }
                world
            },
        )
        .when_regex(
            r"^the height above terrain is (.*)between (\d+) and (\d+) feet$",
            |mut world, matches, _step| {
                let height_at_least = Length::new::<foot>(matches[2].parse().unwrap());
                let height_at_most = Length::new::<foot>(matches[3].parse().unwrap());

                let mut bouncer = BouncingClamp();

                if matches[1].starts_with("not") {
                    pipeline_extend!(world, move |a| bouncer.not_in_range(
                        &mut a.altitude_ground,
                        height_at_least,
                        height_at_most
                    ));
                } else {
                    pipeline_extend!(world, move |a| bouncer.in_range(
                        &mut a.altitude_ground,
                        height_at_least,
                        height_at_most
                    )); // TODO altitude or altitude_ground
                }
                world
            },
        )
        .then_regex(
            "^a Mode 1 (.*) alert is not emitted at all$",
            |mut world, matches, _step| {
                let level = parse_level(&matches[1]);

                let mut aircraft_states: Vec<_> = AircraftStateGenerator::default()
                    .take(world.test_length)
                    .collect();

                // press the test data in our moulds
                for frame in aircraft_states.iter_mut() {
                    for f in world.mould_pipeline.iter_mut() {
                        f(frame);
                    }
                }

                for frame in aircraft_states {
                    let alert_state = world.taws.process(&frame);
                    if alert_state.iter().any(|(a, l)| Alert::Mode1 == a && l <= level)
                    {
                        panic!("Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}", frame, alert_state);
                    }
                }
                world
            },
        )
        .then_regex(
            r"^a Mode 1 (.*) alert is emitted within (\d+) seconds$",
            |mut world, matches, _step| {
                let level = parse_level(&matches[1]);

                let mut aircraft_states: Vec<_> = AircraftStateGenerator::default()
                    .take(world.test_length)
                    .collect();

                // press the test data in our moulds
                for frame in aircraft_states.iter_mut() {
                    for f in world.mould_pipeline.iter_mut() {
                        f(frame);
                    }
                }

                for frame in aircraft_states {
                    let alert_state = world
                        .taws
                        .process(&frame);
                    // TODO what about the time constraint?
                    // Count all alerts that are from the functionality Mode1 and are of higher or
                    // same priority as `level`. If the count is 0, the system did not alert
                    // appropiately.
                    if alert_state
                        .iter()
                        .filter(|(a, l)| *a  == Alert::Mode1 && *l <= level)
                        .count()
                        == 0
                    {
                        panic!("Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}", frame, alert_state);
                    }
                }
                world
            },
        );

    builder
}

// Brot und Butter implementations
impl std::fmt::Debug for MyWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl std::panic::UnwindSafe for MyWorld {} // This is a lie, but way they gonna do, panic?

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
        }))
    }

    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (std::mem::size_of::<AircraftStateWrapper>(), None)
    }
}

// Parser magic
fn parse_alert<T: AsRef<str>>(from: &T) -> Alert {
    let mut input_word = from.as_ref().to_lowercase();
    input_word.retain(|c| !c.is_whitespace());
    match input_word.as_str() {
        "ffac" => Alert::Ffac,
        "flta" => Alert::Flta,
        "mode1" => Alert::Mode1,
        "mode2" => Alert::Mode2,
        "mode3" => Alert::Mode3,
        "mode4" => Alert::Mode4,
        "mode5" => Alert::Mode5,
        "pda" => Alert::Pda,
        _ => {
            panic!(
                "unable to convert {} into a variant of `Alert`",
                from.as_ref()
            );
        }
    }
}

fn parse_level<T: AsRef<str>>(from: &T) -> AlertLevel {
    let mut input_word = from.as_ref().to_lowercase();
    input_word.retain(|c| !c.is_whitespace());
    match input_word.as_str() {
        "warning" => AlertLevel::Warning,
        "caution" => AlertLevel::Caution,
        "annunciation" => AlertLevel::Annunciation,
        _ => {
            panic!(
                "unable to convert {} into a variant of `Alert`",
                from.as_ref()
            );
        }
    }
}

// AircraftState generator
type Prng = rand_pcg::Mcg128Xsl64;
struct AircraftStateGenerator(pub Prng);

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
        let mut u = Unstructured::new(&mut buf);

        Some(AircraftStateWrapper::arbitrary(&mut u).unwrap().0) // the unwrap is safe, we guarantee that enough bytes are available
    }
}

// for the lack of a better word
trait PressMould<T> {
    fn at_least(&mut self, value: &mut T, at_least: T);
    fn at_most(&mut self, value: &mut T, at_most: T);
    fn in_range(&mut self, value: &mut T, at_least: T, at_most: T);
    fn not_in_range(&mut self, value: &mut T, range_from: T, range_to: T);
}

// Stupid
struct BouncingClamp();

impl<T> PressMould<T> for BouncingClamp
where
    T: Copy
        + Clone
        + std::fmt::Debug
        + PartialOrd
        + Add<Output = T>
        + Rem<Output = T>
        + Sub<Output = T>
        + Abs,
{
    fn at_least(&mut self, value: &mut T, at_least: T) {
        if *value < at_least {
            *value = at_least + (at_least - *value)
        }
        assert!(*value >= at_least);
    }

    fn at_most(&mut self, value: &mut T, at_most: T) {
        if *value > at_most {
            *value = at_most - (*value - at_most)
        }
        assert!(*value <= at_most);
    }

    fn in_range(&mut self, value: &mut T, at_least: T, at_most: T) {
        assert!(at_least <= at_most);

        if at_least == at_most {
            *value = at_least;
            return;
        }

        let modulo = |a: T, b: T| ((a % b) + b) % b;

        let value_before = *value;
        let span = at_most - at_least;
        let bounced = (modulo(*value + span, span + span) - span).abs();
        *value = bounced + at_least;

        if !(at_least <= *value && *value <= at_most) {
            panic!(
                "v0 {:?}, v1 {:?}, al {:?}, am {:?}",
                value_before, *value, at_least, at_most
            );
        }
        assert!(at_least <= *value && *value <= at_most);
    }

    fn not_in_range(&mut self, value: &mut T, at_most: T, at_least: T) {
        assert!(at_most <= at_least);
        if *value > at_most && *value < at_least {
            *value = *value + (at_least - at_most);
        }
        assert!(*value < at_least || at_most < *value);
    }
}

trait Abs: Sized {
    fn abs(self) -> Self;
}

impl Abs for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl<D: ?Sized, U: ?Sized, V: ?Sized> Abs for uom::si::Quantity<D, U, V>
where
    D: uom::si::Dimension,
    U: uom::si::Units<V>,
    V: uom::num_traits::Num + uom::Conversion<V> + Signed,
{
    fn abs(self) -> Self {
        self.abs()
    }
}
