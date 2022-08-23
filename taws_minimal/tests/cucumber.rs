//mod constraints;
mod util;

use cucumber::{given, then, when, WorldInit};

use opentaws::prelude::*;
use uom::si::f64::{Length, Velocity};
use uom::si::{length, velocity};
use util::aircraft_state::AircraftStateGenerator;
use util::constraints::Constraint;
use util::parameters::*;
use util::world::MyWorld;

fn main() {
    smol::block_on(MyWorld::run("features"));
}

// TODO check for parallel testing
// TODO allow for statefull tests
// TODO evaluate merge possibilities re parser functions for similar sentences

#[given(expr = "the plane {maybe} flying")]
fn given_flying(_world: &mut MyWorld, _maybe: MaybeParameter) {}

#[given(expr = "{alert} {maybe} armed")]
fn given_alert_armed(world: &mut MyWorld, alert: AlertParameter, maybe: MaybeParameter) {
    let alert: Alert = alert.into();
    match maybe.into() {
        true => world.taws.arm(alert),
        false => world.taws.disarm(alert),
    }
}

#[given(expr = "{alert} {maybe} inhibited")]
fn given_alert_inhibited(world: &mut MyWorld, alert: AlertParameter, maybe: MaybeParameter) {
    let alert: Alert = alert.into();
    match maybe.into() {
        true => world.taws.inhibit(alert),
        false => world.taws.uninhibit(alert),
    }
}

#[given(expr = "a {alert} {alert_level} alert {maybe} active")]
fn given_alert_level_active(
    _world: &mut MyWorld,
    _alert: AlertParameter,
    _level: AlertLevelParameter,
    _maybe: MaybeParameter,
) {
    todo!()
}

#[given(expr = "steep approach {maybe} selected")]
fn given_steep_approach_selected(world: &mut MyWorld, maybe: MaybeParameter) {
    world.constraints[world.phase].add_steep_approach_constraint(maybe.into());
}

#[given(expr = "non-precision approach {maybe} selected")]
fn given_precision_approach_selected(world: &mut MyWorld, maybe: MaybeParameter) {
    let maybe: bool = maybe.into();
    world.constraints[world.phase].add_precision_approach_constraint(!maybe);
}

#[given(expr = "take-off {maybe} selected")]
fn given_take_off(world: &mut MyWorld, maybe: MaybeParameter) {
    world.constraints[world.phase].add_take_off_constraint(maybe.into());
}

#[given(expr = "go around {maybe} selected")]
fn given_go_around(world: &mut MyWorld, maybe: MaybeParameter) {
    world.constraints[world.phase].add_go_around_constraint(maybe.into());
}

#[given(expr = "the height above terrain is {constraint} foot")]
#[when(expr = "the height above terrain is {constraint} foot")]
fn given_height_above_terrain(world: &mut MyWorld, height_above_terrain: ConstraintParameter) {
    let height_above_terrain: Constraint<f64> = height_above_terrain.into();

    let unit = Length::new::<length::foot>(1.0);
    let height_above_terrain = match height_above_terrain {
        Constraint::AtLeast(p, a) => Constraint::AtLeast(p, a * unit),
        Constraint::AtMost(p, a) => Constraint::AtMost(p, a * unit),
        Constraint::Equal(p, a) => Constraint::Equal(p, a * unit),
        Constraint::InRange(p, a, b) => Constraint::InRange(p, a * unit, b * unit),
        Constraint::NotInRange(p, a, b) => Constraint::NotInRange(p, a * unit, b * unit),
    };

    //let height_above_terrain = height_above_terrain * Length::new::<length::foot>(1.0);

    world.constraints[world.phase].add_altitude_ground_constraint(height_above_terrain);
}

#[when(expr = "the rate of descent is {constraint} feet per minute")]
fn when_rate_of_descent(world: &mut MyWorld, rate_of_descent: ConstraintParameter) {
    let rate_of_descent: Constraint<f64> = rate_of_descent.into();

    let unit = Velocity::new::<velocity::foot_per_minute>(-1.0);
    let climb_rate = match rate_of_descent {
        Constraint::AtLeast(p, a) => Constraint::AtMost(p, a * unit),
        Constraint::AtMost(p, a) => Constraint::AtLeast(p, a * unit),
        Constraint::Equal(p, a) => Constraint::Equal(p, a * unit),
        Constraint::InRange(p, a, b) => Constraint::InRange(p, b * unit, a * unit),
        Constraint::NotInRange(p, a, b) => Constraint::NotInRange(p, b * unit, a * unit),
    };

    world.constraints[world.phase].add_climb_rate_constraint(climb_rate);
}

#[when(expr = "the height above terrain is {constraint} feet")]
fn when_height_above_terrain(world: &mut MyWorld, height_above_ground: ConstraintParameter) {
    let height_above_ground: Constraint<f64> = height_above_ground.into();

    let unit = Length::new::<length::foot>(1.0);
    let height_above_ground = match height_above_ground {
        Constraint::AtLeast(p, a) => Constraint::AtLeast(p, a * unit),
        Constraint::AtMost(p, a) => Constraint::AtMost(p, a * unit),
        Constraint::Equal(p, a) => Constraint::Equal(p, a * unit),
        Constraint::InRange(p, a, b) => Constraint::InRange(p, a * unit, b * unit),
        Constraint::NotInRange(p, a, b) => Constraint::NotInRange(p, a * unit, b * unit),
    };

    world.constraints[world.phase].add_altitude_ground_constraint(height_above_ground);
}

#[given(expr = "the plane is {constraint} NM of an airport")]
#[when(expr = "the distance to the nearest airport is {constraint} NM")]
fn when_distance_to_airport(_world: &mut MyWorld, _distance: ConstraintParameter) {
    todo!();
}

#[when(expr = "a failure is detected by the continuos monitoring")]
#[when(expr = "the initiated self-test detects a failure")]
fn when_failure_detected(world: &mut MyWorld) {
    todo!();
}

#[given(expr = "the self-test is initiated")]
#[when(expr = "the self-test is initiated")]
fn when_self_test_initiated(_world: &mut MyWorld) {
    todo!();
}

#[when(expr = "the rate of input data reduces or stagnates")]
fn when_data_rate_reduces(_world: &mut MyWorld) {
    todo!();
}

#[then(expr = "{alert} {maybe} be armed")]
fn then_alert_armed(world: &mut MyWorld, alert: AlertParameter, maybe: MaybeParameter) {
    assert_eq!(world.taws.is_armed(alert.into()), maybe.into())
}

#[then(expr = "a {alert} {alert_level} alert {maybe} emitted( at all)")]
fn then_alert_emitted(
    world: &mut MyWorld,
    alert: AlertParameter,
    level: AlertLevelParameter,
    should_emit: MaybeParameter,
) {
    let alert: Alert = alert.into();
    let level: AlertLevel = level.into();
    let should_emit: bool = should_emit.into();

    let n_constraints = world.constraints.len();
    let aircraft_states = AircraftStateGenerator::default().take(world.test_length * n_constraints);

    for (c, mut frame) in aircraft_states
        .enumerate()
        .map(|(c, f)| (c % n_constraints, f))
    {
        world.constraints[c].apply_to(&mut frame);

        let alert_state = world.taws.process(&frame);
        let emitted = alert_state.iter().any(|(a, l)| a == alert && l <= level);
        //println!("{:#?}", alert_state);
        assert_eq!(emitted, should_emit);
        //alert_state.iter().filter(|(a, l)| a == alert).
        // Make sure we are in the last phase of this scenario with "c + 1 == world.constraints.len()"
        /*if c + 1 == n_constraints && alert_state.iter().any(|(a, l)| a == alert && l <= level) {
            panic!(
                "Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}",
                frame, alert_state
            );
        }*/
    }
}

#[then(expr = "a {alert} {alert_level} alert {maybe} emitted {constraint} seconds")]
fn then_alert_emitted_within(
    world: &mut MyWorld,
    alert: AlertParameter,
    level: AlertLevelParameter,
    should_emit: MaybeParameter,
    _time: ConstraintParameter,
) {
    let alert: Alert = alert.into();
    let level: AlertLevel = level.into();
    let should_emit: bool = should_emit.into();

    let n_constraints = world.constraints.len();
    let aircraft_states = AircraftStateGenerator::default().take(world.test_length * n_constraints);

    for (c, mut frame) in aircraft_states
        .enumerate()
        .map(|(c, f)| (c % n_constraints, f))
    {
        world.constraints[c].apply_to(&mut frame);

        let alert_state = world.taws.process(&frame);
        let emitted = alert_state.iter().any(|(a, l)| a == alert && l <= level);
        //println!("{:#?}", alert_state);
        assert_eq!(emitted, should_emit);
        //alert_state.iter().filter(|(a, l)| a == alert).
        // Make sure we are in the last phase of this scenario with "c + 1 == world.constraints.len()"
        /*if c + 1 == n_constraints && alert_state.iter().any(|(a, l)| a == alert && l <= level) {
            panic!(
                "Aicraft state that violated the scenario: {:#?}\nalerts emitted: {:#?}",
                frame, alert_state
            );
        }*/
    }
}
