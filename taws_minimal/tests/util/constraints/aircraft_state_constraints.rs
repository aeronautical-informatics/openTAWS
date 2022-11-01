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
    track: Option<Constraint<Angle>>,

    situation: Option<FlightSegment>,
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
        Self::apply::<Length, TEnforcer>(&self.altitude, state.altitude_mut());
        Self::apply::<Length, TEnforcer>(&self.altitude_ground, state.altitude_ground_mut());
        Self::apply::<Velocity, TEnforcer>(&self.climb_rate, state.climb_rate_mut());
        Self::apply::<Velocity, TEnforcer>(&self.speed_air, state.speed_air_mut());
        Self::apply::<Angle, TEnforcer>(&self.heading, state.heading_mut());
        Self::apply::<Angle, TEnforcer>(&self.track, state.track_mut());

        *state.situation_mut() = self.situation.or(*state.situation());
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

    pub fn add_situation_constraint(&mut self, situation: FlightSegment) {
        self.situation = Some(situation);
    }

	pub fn add_steep_approach_constraint(&mut self, is_steep: bool) {
		if let Some(FlightSegment::Landing { ref mut steep_approach, .. }) = self.situation {
			*steep_approach = is_steep;
		}
	}

	pub fn add_precision_approach_constraint(&mut self, is_precision: bool) {
		if let Some(FlightSegment::Landing { ref mut precision_approach, .. }) = self.situation {
			*precision_approach = is_precision;
		}
	}

	pub fn add_circling_approach_constraint(&mut self, is_circling: bool) {
		if let Some(FlightSegment::Landing { ref mut circling_approach, .. }) = self.situation {
			*circling_approach = is_circling;
		}
	}
}
