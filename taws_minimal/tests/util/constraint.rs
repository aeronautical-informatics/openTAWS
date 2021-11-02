#[allow(dead_code)]
use crate::util::PressMould;
use crate::BouncingClamp;
use aviation_database::{AirportDatabase, Runway};
use opentaws::AircraftState;
use uom::si::f64::*;

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
            + std::ops::Add
            + std::ops::Add<Output = Q>
            + std::ops::Rem
            + std::ops::Rem<Output = Q>
            + std::ops::Sub
            + std::ops::Sub<Output = Q>
            + crate::util::Abs
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
            + std::ops::Add
            + std::ops::Add<Output = Q>
            + std::ops::Rem
            + std::ops::Rem<Output = Q>
            + std::ops::Sub
            + std::ops::Sub<Output = Q>
            + crate::util::Abs
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

    fn apply<Q>(constraints: &[Constraint<Q>], state: &mut Q)
    where
        Q: Copy
            + PartialOrd
            + PartialEq<Q>
            + std::ops::Add
            + std::ops::Add<Output = Q>
            + std::ops::Rem
            + std::ops::Rem<Output = Q>
            + std::ops::Sub
            + std::ops::Sub<Output = Q>
            + crate::util::Abs
            + std::fmt::Debug,
    {
        let merged = Self::merge(constraints);
        for c in &merged {
            c.apply_to(state)
        }
        for c in constraints {
            c.check(*state)
        }
    }

    pub fn apply_to(&self, state: &mut AircraftState) {
        Self::apply(&self.altitude, &mut state.altitude);
        Self::apply(&self.altitude_ground, &mut state.altitude_ground);
        Self::apply(&self.climb_rate, &mut state.climb_rate);
        Self::apply(&self.speed_air, &mut state.speed_air);
        Self::apply(&self.heading, &mut state.heading);
        Self::apply(&self.pitch, &mut state.pitch);
        Self::apply(&self.roll, &mut state.roll);

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

#[derive(Clone, PartialEq, Debug)]
pub enum Constraint<Q>
where
    Q: Copy
        + PartialOrd
        + PartialEq<Q>
        + std::ops::Add
        + std::ops::Add<Output = Q>
        + std::ops::Rem
        + std::ops::Rem<Output = Q>
        + std::ops::Sub
        + std::ops::Sub<Output = Q>
        + crate::util::Abs
        + std::fmt::Debug,
{
    AtLeast(usize, Q),
    AtMost(usize, Q),
    Equal(usize, Q),
    InRange(usize, Q, Q),
    NotInRange(usize, Q, Q),
}

impl<Q> Constraint<Q>
where
    Q: Copy
        + PartialOrd
        + PartialEq<Q>
        + std::ops::Add
        + std::ops::Add<Output = Q>
        + std::ops::Rem
        + std::ops::Rem<Output = Q>
        + std::ops::Sub
        + std::ops::Sub<Output = Q>
        + crate::util::Abs
        + std::fmt::Debug,
{
    pub fn phase(&self) -> usize {
        match self {
            Constraint::AtLeast(i, _) => *i,
            Constraint::AtMost(i, _) => *i,
            Constraint::Equal(i, _) => *i,
            Constraint::InRange(i, _, _) => *i,
            Constraint::NotInRange(i, _, _) => *i,
        }
    }

    pub fn apply_to(&self, quantity: &mut Q) {
        let mut bouncer = BouncingClamp();

        match self {
            Constraint::AtLeast(_, l) => bouncer.at_least(quantity, *l),
            Constraint::AtMost(_, l) => bouncer.at_most(quantity, *l),
            Constraint::Equal(_, l) => *quantity = *l,
            Constraint::InRange(_, l, r) => bouncer.in_range(quantity, *l, *r),
            Constraint::NotInRange(_, l, r) => bouncer.not_in_range(quantity, *l, *r),
        }
    }

    pub fn check(&self, to_check: Q) {
        match self {
            Constraint::AtLeast(_, l) => assert!(
                to_check >= *l,
                "Checked Number {:?} failed constraint: at least {:?}",
                to_check,
                l
            ),
            Constraint::AtMost(_, l) => assert!(
                to_check <= *l,
                "Checked Number {:?} failed constraint: at most {:?}",
                to_check,
                l
            ),
            Constraint::Equal(_, l) => assert_eq!(
                &to_check, l,
                "Checked Number {:?} failed constraint: equal to {:?}",
                to_check, l
            ),
            Constraint::InRange(_, l, r) => assert!(
                to_check >= *l && to_check <= *r,
                "Checked Number {:?} failed constraint: in range {:?} - {:?}",
                to_check,
                l,
                r
            ),
            Constraint::NotInRange(_, l, r) => assert!(
                *l < to_check || to_check < *r,
                "Checked Number {:?} failed constraint: not in range {:?} - {:?}",
                to_check,
                r,
                l
            ),
        }
    }
}

#[derive(Debug, Default)]
pub struct ConstraintAirportDatabase;

impl AirportDatabase for ConstraintAirportDatabase {
    type RunwayIterator = core::iter::Empty<Runway>;
}
