use std::convert::Infallible;

use async_trait::async_trait;
use rand::random;

use uom::si::{f64::*, length::foot, ratio::ratio, time::second, velocity::foot_per_minute};

use cucumber::Steps;
use opentaws::prelude::*;

struct ScenarioContext {}

#[derive(Debug)]
pub struct MyWorld {
    taws: TAWS,
    template_frame: AircraftState,
    props: ScenarioProperties,
}

#[derive(Clone, Debug, Default)]
struct ScenarioProperties {
    height_min: Option<Length>,
    height_max: Option<Length>,
    height_inside: Option<bool>,
    rate_of_descent_min: Option<Velocity>,
}
impl std::panic::UnwindSafe for MyWorld {}

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            taws: TAWS::new(Default::default()),
            template_frame: Default::default(),
            props: Default::default(),
        })
    }
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
                    world.template_frame.steep_approach = false;
                } else {
                    world.template_frame.steep_approach = true;
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
                world.props.rate_of_descent_min = Some(Velocity::new::<foot_per_minute>(
                    matches[1].parse().unwrap(),
                ));
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

fn main() {
    let runner = cucumber::Cucumber::<MyWorld>::new()
        .features(&["features"])
        .steps(steps());

    futures::executor::block_on(runner.run());
}

// Parser magic

/// Try to convert a `&str` to an `Alert` variant
///
/// Panics on error
fn parse_alert<T: AsRef<str>>(from: &T) -> Alert {
    let mut input_word = from.as_ref().to_lowercase();
    input_word.retain(|c| !c.is_whitespace());
    match input_word.as_str() {
        "ffac" => Alert::FFAC,
        "flta" => Alert::FLTA,
        "mode1" => Alert::Mode1,
        "mode2" => Alert::Mode2,
        "mode3" => Alert::Mode3,
        "mode4" => Alert::Mode4,
        "mode5" => Alert::Mode5,
        "pda" => Alert::PDA,
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
