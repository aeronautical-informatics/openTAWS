use std::convert::Infallible;

use uom::si::{f64::*, length::foot, velocity::foot_per_minute};

use cucumber_rust::{async_trait, given, then, when, WorldInit};
use opentaws::prelude::*;

mod util;
use util::*;

fn main() {
    let runner = MyWorld::init(&["./features"]);
    futures::executor::block_on(runner.run());
}

#[derive(WorldInit)]
pub struct MyWorld {
    taws: Taws,
    moulds: Vec<Box<dyn FnMut(&mut AircraftState)>>,
    test_length: usize,
}

#[given("the plane is flying")]
fn is_flying(_world: &mut MyWorld) {}

#[given(regex = r#"^(.+) is ?(not)? armed$"#)]
fn is_armed(world: &mut MyWorld, alert: AlertWrapper, maybe_not: String) {
    if maybe_not == "not" {
        world.taws.disarm(alert.into());
    } else {
        world.taws.arm(alert.into());
    }
}

#[given(regex = "^(.+) is ?(not)? inhibited$")]
fn is_inhibited(world: &mut MyWorld, alert: AlertWrapper, maybe_not: String) {
    if maybe_not == "not" {
        world.taws.uninhibit(alert.into());
    } else {
        world.taws.inhibit(alert.into());
    }
}

#[given(regex = r"^steep approach is ?(not)? selected$")]
fn steep_approach(world: &mut MyWorld, maybe_not: String) {
    if maybe_not == "not" {
        world.add_mould(|a| a.steep_approach = false);
    } else {
        world.add_mould(|a| a.steep_approach = true);
    }
}

#[then(regex = r"^(.+) shall be armed$")]
fn shall_be_armed(world: &mut MyWorld, alert: AlertWrapper) {
    // TODO check if space needs to be removed
    assert!(world.taws.is_armed(alert.into()));
}

#[when(regex = r"^the rate of descent is at (most|least) (\d+) feet per minute$")]
fn rate_of_descent(world: &mut MyWorld, most_or_least: String, rod: f64) {
    let rod = Velocity::new::<foot_per_minute>(rod);
    let mut bouncer = BouncingClamp();
    // most and least are swapped here, as aircraft_state stores rate of climb, while
    // the sentence give rate of descent
    // TODO validate that this is a safe assumption?
    match most_or_least.as_str() {
        "most" => {
            world.add_mould(move |a| bouncer.at_least(&mut a.climb_rate, -rod));
        }
        "least" => {
            world.add_mould(move |a| bouncer.at_most(&mut a.climb_rate, -rod));
        }
        _ => {
            panic!("unable to parse this sentence");
        }
    }
}

#[when(regex = r"^the height above terrain is ?(not)? between (\d+) and (\d+) feet$")]
fn height_above_terrain(world: &mut MyWorld, maybe_not: String, lower: f64, upper: f64) {
    let height_at_least = Length::new::<foot>(lower);
    let height_at_most = Length::new::<foot>(upper);

    let mut bouncer = BouncingClamp();

    if maybe_not == "not" {
        world.add_mould(move |a| {
            bouncer.not_in_range(&mut a.altitude_ground, height_at_least, height_at_most)
        });
    } else {
        world.add_mould(move |a| {
            bouncer.in_range(&mut a.altitude_ground, height_at_least, height_at_most)
        }); // TODO altitude or altitude_ground
    }
}

#[then(regex = "^a (.*) alert is not emitted at all$")]
fn is_not_emitted(world: &mut MyWorld, alert_and_level: AlertAndLevelWrapper) {
    let (alert, level) = alert_and_level.into();
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
        if alert_state.iter().any(|(a, l)| a == alert && l <= level) {
            panic!(
                "Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}",
                frame, alert_state
            );
        }
    }
}
#[then(regex = r"^a (.*) alert is emitted within (\d+) seconds$")]
fn is_emitted_within(world: &mut MyWorld, alert_and_level: AlertAndLevelWrapper, _seconds: f64) {
    let (alert, level) = alert_and_level.into();
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
        // TODO what about the time constraint?
        // Count all alerts that are from the functionality Mode1 and are of higher or
        // same priority as `level`. If the count is 0, the system did not alert
        // appropiately.
        if alert_state
            .iter()
            .filter(|(a, l)| *a == alert && *l <= level)
            .count()
            == 0
        {
            panic!(
                "Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}",
                frame, alert_state
            );
        }
    }
}

// Brot und Butter implementations
#[async_trait(?Send)]
impl cucumber_rust::World for MyWorld {
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
