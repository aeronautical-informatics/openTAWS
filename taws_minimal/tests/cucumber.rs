//mod constraints;
mod util;

use cucumber::{given, then, when, World};

use opentaws::prelude::*;
use taws_minimal::AlertSource;

use util::constraints::Constraint;
use util::parameters::*;
use util::world::MyWorld;

use crate::util::constraints::BouncingClamp;

fn main() {
    futures::executor::block_on(MyWorld::run("features"));
}

// TODO check for parallel testing
// TODO allow for statefull tests
// TODO evaluate merge possibilities re parser functions for similar sentences

#[given(expr = "in the next phase")]
fn given_new_phase(world: &mut MyWorld) {
    world.next_phase();
}

#[given(expr = "the aircraft {maybe} on a(n) {flight-segment} segment")]
fn given_segment(world: &mut MyWorld, maybe: MaybeParameter, segment: FlightSegmentParameter) {
    let maybe: bool = maybe.into();
    let mut segment: FlightSegment = segment.into();
    // use cruise as default flight segment => "not on a take-off segment" means "on a cruise segment".
    if !maybe {
        assert!(
            segment != FlightSegment::Cruise,
            "Constaint: \"the aircraft is not on a cruise segment\" is not supported."
        );
        segment = FlightSegment::Cruise;
    }

    world.phases[world.phase].add_situation_constraint(segment);
}
#[given(expr = "{alert} {maybe} inhibited")]
fn given_alert_inhibited(world: &mut MyWorld, alert: AlertSourceParameter, maybe: MaybeParameter) {
    let alert: AlertSource = alert.into();
    match maybe.into() {
        true => world.taws.inhibit(alert),
        false => world.taws.uninhibit(alert),
    }
}

#[given(expr = "steep approach {maybe} selected")]
fn given_steep_approach_selected(world: &mut MyWorld, maybe: MaybeParameter) {
    world.phases[world.phase].add_steep_approach_constraint(maybe.into());
}

#[given(expr = "non-precision approach {maybe} selected")]
fn given_precision_approach_selected(world: &mut MyWorld, maybe: MaybeParameter) {
    let maybe: bool = maybe.into();
    world.phases[world.phase].add_precision_approach_constraint(!maybe);
}

#[given(expr = "circling approach {maybe} selected")]
fn given_circling_approach_selected(world: &mut MyWorld, maybe: MaybeParameter) {
    let maybe: bool = maybe.into();
    world.phases[world.phase].add_circling_approach_constraint(!maybe);
}

#[given(expr = "the height above terrain is {constraint} feet")]
#[when(expr = "the height above terrain is {constraint} feet")]
fn given_height_above_terrain(world: &mut MyWorld, height_above_terrain: ConstraintParameter) {
    let height_above_terrain: Constraint<f64> = height_above_terrain.into();

    let unit = Length::new::<length::foot>(1.0);
    let height_above_terrain = match height_above_terrain {
        Constraint::AtLeast(a) => Constraint::AtLeast(a * unit),
        Constraint::AtMost(a) => Constraint::AtMost(a * unit),
        Constraint::Equal(a) => Constraint::Equal(a * unit),
        Constraint::InRange(a, b) => Constraint::InRange(a * unit, b * unit),
        Constraint::NotInRange(a, b) => Constraint::NotInRange(a * unit, b * unit),
    };

    world.phases[world.phase].add_altitude_ground_constraint(height_above_terrain);
}

#[when(expr = "the rate of descent is {constraint} feet per minute")]
fn when_rate_of_descent(world: &mut MyWorld, rate_of_descent: ConstraintParameter) {
    let rate_of_descent: Constraint<f64> = rate_of_descent.into();

    let unit = Velocity::new::<velocity::foot_per_minute>(-1.0);
    let climb_rate = match rate_of_descent {
        Constraint::AtLeast(a) => Constraint::AtMost(a * unit),
        Constraint::AtMost(a) => Constraint::AtLeast(a * unit),
        Constraint::Equal(a) => Constraint::Equal(a * unit),
        Constraint::InRange(a, b) => Constraint::InRange(b * unit, a * unit),
        Constraint::NotInRange(a, b) => Constraint::NotInRange(b * unit, a * unit),
    };

    world.phases[world.phase].add_climb_rate_constraint(climb_rate);
}

#[then(expr = "{alert} {maybe}( be) armed")]
fn then_alert_armed(world: &mut MyWorld, alert: AlertSourceParameter, maybe: MaybeParameter) {
    let mut state = world.next_aircraft_state();
    for phase in world.phases.iter() {
        phase.apply_to::<BouncingClamp>(&mut state);
        let _alerts = world.taws.process(&state.normalize());
        _alerts.unwrap();
    }

    let is_armed = world.taws.is_armed(alert.into());
    assert_eq!(is_armed, maybe.into())
}

#[then(expr = "a {alert} {alert_level} {maybe}( be) emitted {constraint} seconds")]
fn then_alert_emitted_within(
    world: &mut MyWorld,
    alert: AlertSourceParameter,
    level: AlertLevelParameter,
    should_emit: MaybeParameter,
    _time: ConstraintParameter,
) {
    then_alert_emitted(world, alert, level, should_emit)
}

#[then(expr = "a {alert} {alert_level} {maybe}( be) emitted( at all)")]
fn then_alert_emitted(
    world: &mut MyWorld,
    alert_src: AlertSourceParameter,
    min_level: AlertLevelParameter,
    should_emit: MaybeParameter,
) {
    let alert_src: AlertSource = alert_src.into();
    let min_level: AlertLevel = min_level.into();
    let should_emit: bool = should_emit.into();

    let mut aircraft_states: Vec<AircraftState> = world.next_aircraft_states(world.test_length);

    for state in aircraft_states.iter_mut() {
        for (i, phase) in world.phases.iter().enumerate() {
            phase.apply_to::<BouncingClamp>(state);
            let alerts = world.taws.process(&state.normalize()).unwrap();

            if i < world.phases.len() - 1 {
                continue;
            }

            let emitted = alerts.is_alert_active(alert_src, min_level);

            assert_eq!(emitted, should_emit);
        }
    }
}
