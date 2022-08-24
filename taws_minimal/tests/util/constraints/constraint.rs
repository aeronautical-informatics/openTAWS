use std::any::Any;

use uom::num_traits::{Signed, Zero};

use super::constraint_enforcement::{self, ConstraintEnforcer};

#[derive(Clone, PartialEq, Debug)]
pub enum Constraint<Q>
where
    Q: Copy
        + PartialOrd
        + PartialEq
        + std::ops::Add<Output = Q>
        + std::ops::Rem<Output = Q>
        + std::ops::Sub<Output = Q>
        + constraint_enforcement::Abs
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
        + PartialEq
        + std::ops::Add<Output = Q>
        + std::ops::Rem<Output = Q>
        + std::ops::Sub<Output = Q>
        + constraint_enforcement::Abs
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
        let mut bouncer = constraint_enforcement::BouncingClamp();

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
