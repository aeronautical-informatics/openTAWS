use std::convert::Infallible;

use async_trait::async_trait;

use uom::si::{f64::*, length::foot, velocity::foot_per_minute};

use cucumber::Steps;
use opentaws::prelude::*;

mod util;
use util::*;

fn main() {
    let runner = cucumber::Cucumber::<MyWorld>::new()
        .features(&["features"])
        .steps(steps());

    futures::executor::block_on(runner.run());
}

pub struct MyWorld {
    taws: Taws,
    moulds: Vec<Box<dyn FnMut(&mut AircraftState)>>,
    test_length: usize,
}

pub fn steps() -> Steps<crate::MyWorld> {
    let mut builder: Steps<crate::MyWorld> = Steps::new();
    builder
        .given("the plane is flying", |world, _step| world)
        .given_regex(r#"^(.+) is (.*)armed$"#, |mut world, mut matches, _step| {
            matches[1].retain(|c| !c.is_whitespace());
            let alert_system = parse_alert(&matches[1]);
            if matches[2].starts_with("not") {
                world.taws.disarm(alert_system);
            } else {
                world.taws.arm(alert_system);
            }
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
                    world.add_mould(|a| a.steep_approach = false);
                } else {
                    world.add_mould(|a| a.steep_approach = true);
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
                        world.add_mould( move |a| bouncer.at_least(&mut a.climb_rate, -rod));
                    }
                    "least" => {
                        world.add_mould( move |a| bouncer.at_most(&mut a.climb_rate, -rod));
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
                    world.add_mould(move |a| bouncer.not_in_range(
                        &mut a.altitude_ground,
                        height_at_least,
                        height_at_most
                    ));
                } else {
                    world.add_mould( move |a| bouncer.in_range(
                        &mut a.altitude_ground,
                        height_at_least,
                        height_at_most
                    )); // TODO altitude or altitude_ground
                }
                world
            },
        )
        .then_regex(
            "^a (.*) alert is not emitted at all$",
            |mut world, matches, _step| {
                let (alert,level) = parse_alert_level(&matches[1]);

                let mut aircraft_states: Vec<_> = AircraftStateGenerator::default()
                    .take(world.test_length)
                    .collect();

                // press the test data in our moulds
                for frame in aircraft_states.iter_mut() {
                    for f in world.moulds.iter_mut() {
                        f(frame);
                    }
                }

                for frame in aircraft_states {
                    let alert_state = world.taws.process(&frame);
                    if alert_state.iter().any(|(a, l)| a == alert && l <= level)
                    {
                        panic!("Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}", frame, alert_state);
                    }
                }
                world
            },
        )
        .then_regex(
            r"^a (.*) alert is emitted within (\d+) seconds$",
            |mut world, matches, _step| {
                let (alert,level) = parse_alert_level(&matches[1]);

                let mut aircraft_states: Vec<_> = AircraftStateGenerator::default()
                    .take(world.test_length)
                    .collect();

                // press the test data in our moulds
                for frame in aircraft_states.iter_mut() {
                    for f in world.moulds.iter_mut() {
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
                        .filter(|(a, l)| *a  == alert && *l <= level)
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
#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            taws: Taws::new(Default::default()),
            moulds: Vec::new(),
            test_length: 10000, // TODO is this a good number?
        })
    }
}

impl MyWorld {
    pub fn add_mould<F: 'static + FnMut(&mut AircraftState)>(&mut self, f: F) {
        self.moulds.push(Box::new(f));
    }
}

impl std::fmt::Debug for MyWorld {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl std::panic::UnwindSafe for MyWorld {} // This is a lie, but what they gonna do, panic?
