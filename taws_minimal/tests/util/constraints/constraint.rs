use super::constraint_enforcement::{self, ConstraintEnforcer};

#[derive(Clone, PartialEq, Eq, Debug)]
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
    AtLeast(Q),
    AtMost(Q),
    Equal(Q),
    InRange(Q, Q),
    NotInRange(Q, Q),
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
    fn normalize(&self) -> Constraint<Q> {
        match self {
            Constraint::AtLeast(l) => Constraint::AtLeast(*l),
            Constraint::AtMost(r) => Constraint::AtMost(*r),
            Constraint::Equal(x) => Constraint::Equal(*x),
            Constraint::InRange(l, r) => {
                let (l, r) = if l <= r { (l, r) } else { (r, l) };
                Constraint::InRange(*l, *r)
            }
            Constraint::NotInRange(l, r) => {
                let (l, r) = if r <= l { (l, r) } else { (r, l) };
                Constraint::NotInRange(*l, *r)
            }
        }
    }

    pub fn merge(&self, other: &Constraint<Q>) -> Result<Constraint<Q>, String> {
        let c1 = self.normalize();
        let c2 = other.normalize();
        match (c1, c2) {
            (Constraint::AtLeast(l1), Constraint::AtLeast(l2)) => {
                let l = if l1 >= l2 { l1 } else { l2 };
                Ok(Constraint::AtLeast(l))
            }

            (Constraint::AtMost(r1), Constraint::AtMost(r2)) => {
                let r = if r1 <= r2 { r1 } else { r2 };
                Ok(Constraint::AtMost(r))
            }

            (Constraint::AtLeast(l), Constraint::AtMost(r))
            | (Constraint::AtMost(r), Constraint::AtLeast(l)) => match (l, r) {
                (l, r) if l > r => Err("unsatisfiable".to_string()),
                (l, r) => Ok(Constraint::InRange(l, r)),
            },

            (Constraint::Equal(x), Constraint::AtLeast(l))
            | (Constraint::AtLeast(l), Constraint::Equal(x)) => match (x, l) {
                (x, l) if x >= l => Ok(Constraint::Equal(x)),
                _ => Err("unsatisfiable".to_string()),
            },

            (Constraint::Equal(x), Constraint::AtMost(r))
            | (Constraint::AtMost(r), Constraint::Equal(x)) => match (x, r) {
                (x, r) if x <= r => Ok(Constraint::Equal(x)),
                _ => Err("unsatisfiable".to_string()),
            },

            (Constraint::Equal(x), Constraint::Equal(y)) => match (x, y) {
                (x, y) if x == y => Ok(Constraint::Equal(x)),
                _ => Err("unsatisfiable".to_string()),
            },

            (Constraint::AtLeast(l1), Constraint::InRange(l, r))
            | (Constraint::InRange(l, r), Constraint::AtLeast(l1)) => {
                let at_least = Constraint::AtLeast(l1);
                let range_least = Constraint::AtLeast(l);
                let range_most = Constraint::AtMost(r);
                at_least.merge(&range_least)?.merge(&range_most)
            }

            (Constraint::AtMost(r1), Constraint::InRange(l, r))
            | (Constraint::InRange(l, r), Constraint::AtMost(r1)) => {
                let at_most = Constraint::AtMost(r1);
                let range_least = Constraint::AtLeast(l);
                let range_most = Constraint::AtMost(r);
                at_most.merge(&range_most)?.merge(&range_least)
            }

            (Constraint::Equal(x), Constraint::InRange(l, r))
            | (Constraint::InRange(l, r), Constraint::Equal(x)) => {
                let equal = Constraint::Equal(x);
                let range = (Constraint::AtLeast(l), Constraint::AtMost(r));
                let x1 = equal.merge(&range.0)?;
                let x2 = equal.merge(&range.1)?;
                x1.merge(&x2)
            }

            (Constraint::InRange(l1, r1), Constraint::InRange(l2, r2)) => {
                let range1 = (Constraint::AtLeast(l1), Constraint::AtMost(r1));
                let range2 = (Constraint::AtLeast(l2), Constraint::AtMost(r2));
                let at_least = range1.0.merge(&range2.0)?;
                let at_most = range1.1.merge(&range2.1)?;
                at_least.merge(&at_most)
            }

            // Merging NotInRange could lead to non convex constraints or constraints containing `Greater Than` or `Less Than` comparisons.
            // Could be implemented, but maybe not worth it.
            _ => Err("not supported".to_string()),
        }
    }

    pub fn apply_to<TEnforcer>(&self, quantity: &mut Q)
    where
        TEnforcer: ConstraintEnforcer<Q> + Default,
    {
        let mut bouncer = TEnforcer::default();

        match self {
            Constraint::AtLeast(l) => bouncer.at_least(quantity, *l),
            Constraint::AtMost(l) => bouncer.at_most(quantity, *l),
            Constraint::Equal(l) => *quantity = *l,
            Constraint::InRange(l, r) => bouncer.in_range(quantity, *l, *r),
            Constraint::NotInRange(l, r) => bouncer.not_in_range(quantity, *l, *r),
        }
    }

    pub fn check(&self, to_check: Q) {
        match self {
            Constraint::AtLeast(l) => assert!(
                to_check >= *l,
                "Checked Number {:?} failed constraint: at least {:?}",
                to_check,
                l
            ),
            Constraint::AtMost(l) => assert!(
                to_check <= *l,
                "Checked Number {:?} failed constraint: at most {:?}",
                to_check,
                l
            ),
            Constraint::Equal(l) => assert_eq!(
                &to_check, l,
                "Checked Number {:?} failed constraint: equal to {:?}",
                to_check, l
            ),
            Constraint::InRange(l, r) => assert!(
                to_check >= *l && to_check <= *r,
                "Checked Number {:?} failed constraint: in range {:?} - {:?}",
                to_check,
                l,
                r
            ),
            Constraint::NotInRange(l, r) => assert!(
                *l < to_check || to_check < *r,
                "Checked Number {:?} failed constraint: not in range {:?} - {:?}",
                to_check,
                r,
                l
            ),
        }
    }
}
