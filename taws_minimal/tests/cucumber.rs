use std::cmp::PartialEq;
use std::convert::Infallible;

use uom::si::{f64::*, length::foot, velocity::foot_per_minute};

use async_trait::async_trait;
use cucumber::{given, then, when, WorldInit};
use opentaws::prelude::*;
use taws_minimal::MinimalTaws;

mod util;
use util::*;

use util::constraint::*;

fn main() {
    smol::block_on(MyWorld::run("features"));
}

// TODO check for parallel testing
// TODO allow for statefull tests
// TODO evaluate merge possibilities re parser functions for similar sentences

#[derive(WorldInit)]
pub struct MyWorld {
    taws: MinimalTaws<'static>,
    constraints: Vec<AircraftStateConstraints>,
    test_length: usize,
    last_step_type: StepType,
    phase: usize,
}

//TODO use cucumber rs StepType enum
#[derive(PartialEq)]
pub enum StepType {
    Given,
    When,
    Then,
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
    world.update_phase(StepType::Given);
    if maybe_not == "not" {
        world.constraints[world.phase].add_steep_approach_constraint(false)
    } else {
        world.constraints[world.phase].add_steep_approach_constraint(true)
    }
}

#[given(regex = r"^take-off is ?(not)? selected$")]
fn take_of(world: &mut MyWorld, maybe_not: String) {
    world.update_phase(StepType::Given);
    if maybe_not == "not" {
        world.constraints[world.phase].add_take_off_constraint(false)
    } else {
        world.constraints[world.phase].add_take_off_constraint(true)
    }
}

#[given(regex = r"^go around is ?(not)? selected$")]
fn go_around(world: &mut MyWorld, maybe_not: String) {
    world.update_phase(StepType::Given);
    if maybe_not == "not" {
        world.constraints[world.phase].add_go_around_constraint(false)
    } else {
        world.constraints[world.phase].add_go_around_constraint(true)
    }
}

#[given(regex = r"^non-precision approach is ?(not)? selected$")]
fn precision_approach(world: &mut MyWorld, maybe_not: String) {
    world.update_phase(StepType::Given);

    // double negation: non-precision is not selected
    if maybe_not == "not" {
        world.constraints[world.phase].add_precision_approach_constraint(true);
    } else {
        world.constraints[world.phase].add_precision_approach_constraint(false);
    }
}

#[then(regex = r"^(.+) shall ?(not)? be armed$")]
fn shall_be_armed(world: &mut MyWorld, alert: AlertWrapper, maybe_not: String) {
    assert_eq!(world.taws.is_armed(alert.into()), !maybe_not.eq("not"));
}

#[when(regex = r"^the rate of descent is at (most|least) (\d+) feet per minute$")]
fn rate_of_descent(world: &mut MyWorld, most_or_least: String, rod: f64) {
    world.update_phase(StepType::When);

    let rod = Velocity::new::<foot_per_minute>(rod);
    // most and least are swapped here, as aircraft_state stores rate of climb, while
    // the sentence give rate of descent
    // TODO validate that this is a safe assumption?
    match most_or_least.as_str() {
        "most" => {
            world.constraints[world.phase]
                .add_climb_rate_constraint(Constraint::AtLeast(world.phase, -rod));
            //world.add_mould(move |a| bouncer.at_least(&mut a.climb_rate, -rod));
        }
        "least" => {
            world.constraints[world.phase]
                .add_climb_rate_constraint(Constraint::AtMost(world.phase, -rod));
            //world.add_mould(move |a| bouncer.at_most(&mut a.climb_rate, -rod));
        }
        _ => {
            panic!("unable to parse this sentence");
        }
    }
}

#[when(regex = r"^the height above terrain is ?(not)? between (\d+) and (\d+) feet$")]
fn height_above_terrain(world: &mut MyWorld, maybe_not: String, lower: f64, upper: f64) {
    world.update_phase(StepType::When);

    let height_at_least = Length::new::<foot>(lower);
    let height_at_most = Length::new::<foot>(upper);

    if maybe_not == "not" {
        world.constraints[world.phase].add_altitude_ground_constraint(Constraint::NotInRange(
            world.phase,
            height_at_least,
            height_at_most,
        ));
    } else {
        world.constraints[world.phase].add_altitude_ground_constraint(Constraint::InRange(
            world.phase,
            height_at_least,
            height_at_most,
        ));
        // TODO altitude or altitude_ground
    }
}

#[when(
    regex = r"^the distance to the nearest airport is ?(not)? between ([0-9]+\.?[0-9]*) and ([0-9]+\.?[0-9]*) NM"
)]
fn distance_to_airport(world: &mut MyWorld, maybe_not: String, lower: f64, upper: f64) {
    world.update_phase(StepType::When);
    todo!();
    let height_at_least = Length::new::<foot>(lower);
    let height_at_most = Length::new::<foot>(upper);

    if maybe_not == "not" {
    } else {
    }
}

#[given(regex = r"^the height above terrain is at (most|least) (\d+) foot$")]
fn height_above_terrain_2(world: &mut MyWorld, greater_or_less: String, height: f64) {
    todo!("Merge with previous function");
}

#[given(regex = r"^the nearest runway elevation is at (most|least) (\d+) foot$")]
fn height_above_runway(world: &mut MyWorld, greater_or_less: String, height: f64) {
    todo!("Merge with previous function");
}

#[given(regex = r"^the plane is ?(not)? within ([0-9]+\.?[0-9]*) NM of an airport")]
fn distance_to_airport_2(world: &mut MyWorld, maybe_not: String, distance: f64) {
    todo!("Merge with previous function");
}

#[then(regex = "^a (.*) alert is not emitted at all$")]
fn is_not_emitted(world: &mut MyWorld, alert_and_level: AlertAndLevelWrapper) {
    world.update_phase(StepType::Then);
    let (alert, level) = alert_and_level.into();

    let aircraft_states =
        AircraftStateGenerator::default().take(world.test_length * world.constraints.len());

    let n_constraints = world.constraints.len();
    for (c, mut frame) in aircraft_states
        .enumerate()
        .map(|(c, f)| (c % n_constraints, f))
    {
        world.constraints[c].apply_to(&mut frame);

        let alert_state = world.taws.process(&mut frame);
        // Make sure we are in the last phase of this scenario with "c + 1 == world.constraints.len()"
        if c + 1 == n_constraints && alert_state.iter().any(|(a, l)| a == alert && l <= level) {
            panic!(
                "Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}",
                frame, alert_state
            );
        }
    }
}

#[then(regex = r"^a (.*) alert is emitted within ([0-9]+\.?[0-9]*) seconds$")]
fn is_emitted_within(world: &mut MyWorld, alert_and_level: AlertAndLevelWrapper, _seconds: f64) {
    world.update_phase(StepType::Then);
    let (alert, level) = alert_and_level.into();

    let aircraft_states =
        AircraftStateGenerator::default().take(world.test_length * world.constraints.len());

    let n_constraints = world.constraints.len();
    for (c, mut frame) in aircraft_states
        .enumerate()
        .map(|(c, f)| (c % n_constraints, f))
    {
        world.constraints[c].apply_to(&mut frame);

        let alert_state = world.taws.process(&mut frame);
        // TODO what about the time constraint?
        // Count all alerts that are from the functionality Mode1 and are of higher or
        // same priority as `level`. If the count is 0, the system did not alert
        // appropriately.
        if c + 1 == n_constraints
            && alert_state
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

static AIRPORT_DATABASE: ConstraintAirportDatabase = ConstraintAirportDatabase {};

lazy_static::lazy_static! {
    static ref TAWS_CONFIG: TawsConfig<'static> = TawsConfig{
            terrain_server: &AIRPORT_DATABASE,
            max_climbrate: Velocity::new::<foot_per_minute>(700.0),
            max_climbrate_change: Acceleration::new::<foot_per_second_squared>(100.0),
    };
}

// Brot und Butter implementations
#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            taws: MinimalTaws::new(&TAWS_CONFIG),
            constraints: vec![AircraftStateConstraints::default()],
            test_length: 10, // TODO Increase should we be able to reduce the nearest_airport function from the aviation database
            last_step_type: StepType::Given,
            phase: 0,
        })
    }
}

impl MyWorld {
    // Updates the current phase based on the current StepType
    pub fn update_phase(&mut self, current_type: StepType) {
        if self.last_step_type != current_type {
            //Only increase phase if current phase is not "Then"
            // Then-Step should not enforce AirplaneStateConstraints
            if current_type != StepType::Then {
                self.constraints.push(self.constraints[self.phase].clone());
                self.phase += 1;
            }

            if self.last_step_type == StepType::Then && current_type == StepType::When {
                panic!("Multiple When/Then Pairs are not allowed")
            }

            self.last_step_type = current_type;
        }
    }
}

impl std::fmt::Debug for MyWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyWorld").finish()
        //todo!();
    }
}

impl std::panic::UnwindSafe for MyWorld {} // This is a lie, but what they gonna do, panic?
