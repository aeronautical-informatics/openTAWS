use std::sync::Arc;
use std::{
    convert::Infallible,
    ops::{Add, Rem, Sub},
};

use async_trait::async_trait;
use rand::{Rng, RngCore};

use uom::{
    num::Signed,
    si::{
        f64::*, information::byte, length::foot, ratio::ratio, time::second,
        velocity::foot_per_minute,
    },
};

use arbitrary::{Arbitrary, Unstructured};
use cucumber::Steps;
use opentaws::prelude::*;

fn main() {
    let my_vec: Vec<_> = AircraftStateGenerator::default().take(100).collect();

    println!("{:#?}", my_vec);

    //let mut rng = rand_pcg::Mcg128Xsl64::new(0xcafef00dd15ea5e5);
    //let mut unstructured = Unstructured::new(AsMut::<[u8]>::as_mut(&mut rng));

    //let aircraft_state = AircraftStateWrapper::arbitrary(&mut unstructured)
    //    .expect("`unstructured` has enough underlying data to create all variants of `MyEnum`");
    //println!("Random aircraft state: {:#?}", aircraft_state);

    //let runner = cucumber::Cucumber::<MyWorld>::new()
    //    .features(&["features"])
    //    .steps(steps());

    //futures::executor::block_on(runner.run());
}

pub struct MyWorld {
    taws: Taws,
    mould_pipeline: Vec<Box<dyn FnMut(&mut AircraftState)>>,
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
                    pipeline_extend!(world, |a| {
                        a.steep_approach = false;
                    });
                } else {
                    pipeline_extend!(world, |a| {
                        a.steep_approach = true;
                    });
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
            r"^the rate of descent is at least (\d+) feet per minute$",
            |mut world, matches, _step| {
                let roc = Velocity::new::<foot_per_minute>(matches[1].parse().unwrap());
                let bouncer = BouncingClamp();
                pipeline_extend!(world, move |a| {
                    a.climb_rate = -bouncer.at_least(-a.rate_of_climb, roc);
                });
                world
            },
        )
        .when_regex(
            r"^the height above terrain is (.*)between (\d+) and (\d+) feet$",
            |mut world, matches, _step| {
                world.props.height_inside = Some(!matches[1].starts_with("not"));
                //world.props.height_min = Some(Length::new::<foot>(matches[2].parse().unwrap()));
                //world.props.height_max = Some(Length::new::<foot>(matches[3].parse().unwrap()));
                world
            },
        )
        .then_regex(
            "^a Mode 1 (.*) alert is not emitted at all$",
            |mut world, matches, _step| {
                let alert = parse_level(&matches[1]);

                let mut frame = world.template_frame.clone();

                let min = world
                    .props
                    .height_min
                    .unwrap_or(Length::new::<foot>(random()));
                let max = world
                    .props
                    .height_max
                    .unwrap_or(Length::new::<foot>(random()));
                let inside = world.props.height_inside.unwrap_or(random());

                /* TODO rewrite this, it's ugly
                if inside {
                    frame.altitude_ground = min;
                    assert_eq!(world.taws.process(&frame).iter().filter(|((a,_))| a == alert).count(), 0);

                    frame.altitude_ground = max;
                    assert_eq!(world.taws.process(&frame).alerts_count(alert), 0);

                    frame.altitude_ground = (max + min) / Ratio::new::<ratio>(2.0);
                    assert_eq!(world.taws.process(&frame).alerts_count(alert), 0);
                } else {
                    frame.altitude_ground = min - Length::new::<foot>(1.0);
                    assert_eq!(world.taws.process(&frame).alerts_count(alert), 0);

                    frame.altitude_ground = max + Length::new::<foot>(1.0);
                    assert_eq!(world.taws.process(&frame).alerts_count(alert), 0);
                }

                assert_eq!(world.taws.process(&frame).alert_level(Alert::Mode1), None);

                   use quickcheck::QuickCheck;
                   let mut qc = QuickCheck::new();

                   fn tests(mut world: MyWorld )->bool {
                   let alert_state = world.taws.push(&world.template_frame);

                   alert_state.alerts.is_empty() &&
                   alert_state.nuisance_alerts.is_empty()
                   };

                   qc.quickcheck(tests as fn(_)->_);


                   let new_frame = world.template_frame.clone();

                   let alert_state = world.taws.push(&new_frame);

                   assert!(alert_state.alerts.is_empty());
                   assert!(alert_state.nuisance_alerts.is_empty());
                   */
                world
            },
        )
        .then_regex(
            r"^a Mode 1 (.*) alert is emitted within (\d+) seconds$",
            |mut world, matches, _step| {
                let alert = parse_level(&matches[1]);
                let _max_latency = Time::new::<second>(matches[2].parse().unwrap());

                let mut frame = world.template_frame.clone();
                frame.timestamp += Time::new::<second>(0.1);
                let min = world.props.height_min.unwrap();
                let max = world.props.height_max.unwrap();
                let inside = world.props.height_inside.unwrap();
                /* TODO rewrite this, it's ugly
                if inside {
                    frame.altitude_ground = min;
                    assert!(world.taws.process(&frame).alerts_count(alert) > 0);

                    frame.altitude_ground = max;
                    assert!(world.taws.process(&frame).alerts_count(alert) > 0);

                    frame.altitude_ground = (max + min) / Ratio::new::<ratio>(2.0);
                    assert!(world.taws.process(&frame).alerts_count(alert) > 0);
                } else {
                    frame.altitude_ground = min - Length::new::<foot>(1.0);
                    assert!(world.taws.process(&frame).alerts_count(alert) > 0);

                    frame.altitude_ground = max + Length::new::<foot>(1.0);
                    assert!(world.taws.process(&frame).alerts_count(alert) > 0);
                }
                */
                world
            },
        );
    //.given_regex(
    //    r"the rate of rage is at least (.+) feet per minute",
    //    |world, matches, _step| {
    //        panic!("{}", matches[1]);
    //        world
    //    },
    //);

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
            timestamp: Time::new::<second>(u.arbitrary()?),
            altitude: Length::new::<foot>(u.arbitrary()?),
            altitude_ground: Length::new::<foot>(u.arbitrary()?),
            climb_rate: Velocity::new::<foot_per_minute>(u.arbitrary()?),
            position_lat: Angle::new::<degree>(u.arbitrary()?),
            position_lon: Angle::new::<degree>(u.arbitrary()?),
            speed_ground: Velocity::new::<knot>(u.arbitrary()?),
            speed_air: Velocity::new::<knot>(u.arbitrary()?),
            heading: Angle::new::<degree>(u.arbitrary()?),
            pitch: Angle::new::<degree>(u.arbitrary()?),
            roll: Angle::new::<degree>(u.arbitrary()?),
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
            buf.extend(&self.0.next_u64().to_le_bytes());
        }
        let mut u = Unstructured::new(&mut buf);

        Some(AircraftStateWrapper::arbitrary(&mut u).unwrap().0) // the unwrap is safe, we guarantee that enough bytes are available
    }
}

// for the lack of a better word
trait PressMould<T> {
    fn at_least(&mut self, value: T, at_least: T) -> T;
    fn at_most(&mut self, value: T, at_most: T) -> T;
    fn in_range(&mut self, value: T, at_least: T, at_most: T) -> T;
}

// Stupid
struct BouncingClamp();

impl<T> PressMould<T> for BouncingClamp
where
    T: PartialOrd + Add<Output = T> + Rem<Output = T> + Sub<Output = T> + Signed,
{
    fn at_least(&mut self, value: T, at_least: T) -> T {
        if value >= at_least {
            value
        } else {
            at_least + (at_least - value)
        }
    }

    fn at_most(&mut self, value: T, at_most: T) -> T {
        if value <= at_most {
            value
        } else {
            at_most - (value - at_most)
        }
    }

    /// clamp 'n bounce
    fn in_range(&mut self, value: T, at_least: T, at_most: T) -> T {
        let span = at_most - at_least;
        let bounced = ((value + span) % (span + span) - span).abs();
        bounced + at_least
    }
}
