use opentaws::prelude::*;
use uom::si::f64::{Angle, Length, Velocity};

use super::{constraint_enforcement, Constraint, ConstraintEnforcer};

#[derive(Clone, Default, PartialEq)]
pub struct AircraftStateConstraints {
    altitude: Option<Constraint<Length>>,
    altitude_ground: Option<Constraint<Length>>,
    climb_rate: Option<Constraint<Velocity>>,
    speed_air: Option<Constraint<Velocity>>,
    heading: Option<Constraint<Angle>>,
    pitch: Option<Constraint<Angle>>,
    roll: Option<Constraint<Angle>>,

    steep_approach: Option<bool>,
    precision_approach: Option<bool>,
    go_around: Option<bool>,
    take_off: Option<bool>,
}

impl AircraftStateConstraints {
    fn apply<Q, TEnforcer>(constraint: &Option<Constraint<Q>>, state: &mut Q)
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
        if let Some(constraint) = constraint {
            constraint.apply_to::<TEnforcer>(state);
            constraint.check(*state);
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

        state.steep_approach = self.steep_approach.unwrap_or(state.steep_approach);
        state.precision_approach = self.precision_approach.unwrap_or(state.precision_approach);
        state.go_around = self.go_around.unwrap_or(state.go_around);
        state.take_off = self.take_off.unwrap_or(state.take_off);
    }

    pub fn add_altitude_constraint(&mut self, c: Constraint<Length>) {
        self.altitude = match &self.altitude {
            Some(altitude) => Some(altitude.merge(&c).unwrap()),
            None => Some(c),
        }
    }

    pub fn add_altitude_ground_constraint(&mut self, c: Constraint<Length>) {
        self.altitude_ground = match &self.altitude_ground {
            Some(altitude_gnd) => Some(altitude_gnd.merge(&c).unwrap()),
            None => Some(c),
        }
    }

    pub fn add_climb_rate_constraint(&mut self, c: Constraint<Velocity>) {
        self.climb_rate = match &self.climb_rate {
            Some(climb_rate) => Some(climb_rate.merge(&c).unwrap()),
            None => Some(c),
        }
    }

    pub fn add_speed_air_constraint(&mut self, c: Constraint<Velocity>) {
        self.speed_air = match &self.speed_air {
            Some(speed_air) => Some(speed_air.merge(&c).unwrap()),
            None => Some(c),
        }
    }

    pub fn add_heading_constraint(&mut self, c: Constraint<Angle>) {
        self.heading = match &self.heading {
            Some(heading) => Some(heading.merge(&c).unwrap()),
            None => Some(c),
        }
    }

    pub fn add_pitch_constraint(&mut self, c: Constraint<Angle>) {
        self.pitch = match &self.pitch {
            Some(pitch) => Some(pitch.merge(&c).unwrap()),
            None => Some(c),
        }
    }

    pub fn add_roll_constraint(&mut self, c: Constraint<Angle>) {
        self.roll = match &self.roll {
            Some(roll) => Some(roll.merge(&c).unwrap()),
            None => Some(c),
        }
    }

    pub fn add_steep_approach_constraint(&mut self, c: bool) {
        self.steep_approach = Some(c);
    }

    pub fn add_precision_approach_constraint(&mut self, c: bool) {
        self.precision_approach = Some(c);
    }

    pub fn add_go_around_constraint(&mut self, c: bool) {
        self.go_around = Some(c);
    }

    pub fn add_take_off_constraint(&mut self, c: bool) {
        self.take_off = Some(c);
    }
}
