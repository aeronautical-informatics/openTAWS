use opentaws::prelude::*;
use uom::si::f64::{Angle, Length, Velocity};

use super::{constraint_enforcement, Constraint, ConstraintEnforcer};

#[derive(Clone, Default, PartialEq)]
pub struct AircraftStateConstraints {
    altitude: Vec<Constraint<Length>>,
    altitude_ground: Vec<Constraint<Length>>,
    climb_rate: Vec<Constraint<Velocity>>,
    speed_air: Vec<Constraint<Velocity>>,
    heading: Vec<Constraint<Angle>>,
    pitch: Vec<Constraint<Angle>>,
    roll: Vec<Constraint<Angle>>,
    //TODO use vector with bool and phase
    steep_approach: bool,
    precision_approach: bool,
    go_around: bool,
    take_of: bool,
}

impl AircraftStateConstraints {
    //Retains constraints from the current phase
    fn filter_phase<Q>(constraints: &mut Vec<Constraint<Q>>, current_phase: usize)
    where
        Q: Copy
            + PartialOrd
            + PartialEq<Q>
            + std::ops::Add<Output = Q>
            + std::ops::Rem<Output = Q>
            + std::ops::Sub<Output = Q>
            + constraint_enforcement::Abs
            + std::fmt::Debug,
    {
        let mut i = 0;
        while i < constraints.len() {
            if constraints[i].phase() != current_phase {
                constraints.remove(i);
            } else {
                i += 1;
            }
        }
    }

    //Merges constraints
    //Especially important for merging < and > constraints into one InRange constraint
    fn merge<Q>(constraints: &[Constraint<Q>]) -> Vec<Constraint<Q>>
    where
        Q: Copy
            + PartialOrd
            + PartialEq<Q>
            + std::ops::Add<Output = Q>
            + std::ops::Rem<Output = Q>
            + std::ops::Sub<Output = Q>
            + constraint_enforcement::Abs
            + std::fmt::Debug,
    {
        let mut cs = Vec::new();
        let mut at_least = None;
        let mut at_most = None;

        for c in constraints {
            match c {
                Constraint::AtLeast(_, q) => {
                    if at_least.map_or(true, |a| q > a) {
                        at_least = Some(q);
                    }
                }
                Constraint::AtMost(_, q) => {
                    if at_most.map_or(true, |a| q < a) {
                        at_most = Some(q);
                    }
                }
                Constraint::InRange(_, l, r) => {
                    if at_least.map_or(true, |a| l > a) {
                        at_least = Some(l);
                    }
                    if at_most.map_or(true, |a| r < a) {
                        at_most = Some(r);
                    }
                }
                _ => cs.push(c.clone()),
            }
        }

        match (at_least, at_most) {
            (Some(at_least), Some(at_most)) => {
                assert!(
                    at_least <= at_most,
                    "Left bound ({:?}) is greater than right bound ({:?})",
                    at_least,
                    at_most
                );
                cs.push(Constraint::InRange(
                    constraints.first().unwrap().phase(),
                    *at_least,
                    *at_most,
                ));
            }
            (Some(at_least), None) => cs.push(Constraint::AtLeast(
                constraints.first().unwrap().phase(),
                *at_least,
            )),
            (None, Some(at_most)) => cs.push(Constraint::AtMost(
                constraints.first().unwrap().phase(),
                *at_most,
            )),
            _ => {}
        }

        cs
    }

    fn apply<Q, TEnforcer>(constraints: &[Constraint<Q>], state: &mut Q)
    where
        Q: Copy
            + PartialOrd
            + PartialEq<Q>
            + std::ops::Add<Output = Q>
            + std::ops::Rem<Output = Q>
            + std::ops::Sub<Output = Q>
            + constraint_enforcement::Abs
            + std::fmt::Debug,
        TEnforcer: ConstraintEnforcer<Q> + Default,
    {
        let merged = Self::merge(constraints);
        for c in &merged {
            c.apply_to::<TEnforcer>(state)
        }
        for c in constraints {
            c.check(*state)
        }
    }

    pub fn apply_to<TEnforcer>(&self, state: &mut AircraftState)
    where
        TEnforcer: ConstraintEnforcer<Length>
            + ConstraintEnforcer<Velocity>
            + ConstraintEnforcer<Angle>
            + Default,
    {
        Self::apply::<Length, TEnforcer>(&self.altitude, &mut state.altitude);
        Self::apply::<Length, TEnforcer>(&self.altitude_ground, &mut state.altitude_ground);
        Self::apply::<Velocity, TEnforcer>(&self.climb_rate, &mut state.climb_rate);
        Self::apply::<Velocity, TEnforcer>(&self.speed_air, &mut state.speed_air);
        Self::apply::<Angle, TEnforcer>(&self.heading, &mut state.heading);
        Self::apply::<Angle, TEnforcer>(&self.pitch, &mut state.pitch);
        Self::apply::<Angle, TEnforcer>(&self.roll, &mut state.roll);

        //TODO Change when bools get vec
        state.steep_approach = self.steep_approach;
        state.precision_approach = self.precision_approach;
        state.go_around = self.go_around;
        state.take_off = self.take_of;
    }

    pub fn add_altitude_constraint(&mut self, c: Constraint<Length>) {
        let phase = c.phase();
        self.altitude.push(c);
        Self::filter_phase(self.altitude.as_mut(), phase)
    }

    pub fn add_altitude_ground_constraint(&mut self, c: Constraint<Length>) {
        let phase = c.phase();
        self.altitude_ground.push(c);
        Self::filter_phase(self.altitude_ground.as_mut(), phase)
    }

    pub fn add_climb_rate_constraint(&mut self, c: Constraint<Velocity>) {
        let phase = c.phase();
        self.climb_rate.push(c);
        Self::filter_phase(self.climb_rate.as_mut(), phase)
    }

    pub fn add_speed_air_constraint(&mut self, c: Constraint<Velocity>) {
        let phase = c.phase();
        self.speed_air.push(c);
        Self::filter_phase(self.speed_air.as_mut(), phase)
    }

    pub fn add_heading_constraint(&mut self, c: Constraint<Angle>) {
        let phase = c.phase();
        self.heading.push(c);
        Self::filter_phase(self.heading.as_mut(), phase)
    }

    pub fn add_pitch_constraint(&mut self, c: Constraint<Angle>) {
        let phase = c.phase();
        self.pitch.push(c);
        Self::filter_phase(self.pitch.as_mut(), phase)
    }

    pub fn add_roll_constraint(&mut self, c: Constraint<Angle>) {
        let phase = c.phase();
        self.roll.push(c);
        Self::filter_phase(self.roll.as_mut(), phase)
    }

    pub fn add_steep_approach_constraint(&mut self, c: bool) {
        self.steep_approach = c;
    }

    pub fn add_precision_approach_constraint(&mut self, c: bool) {
        self.precision_approach = c;
    }

    pub fn add_go_around_constraint(&mut self, c: bool) {
        self.go_around = c;
    }

    pub fn add_take_off_constraint(&mut self, c: bool) {
        self.take_of = c;
    }
}
