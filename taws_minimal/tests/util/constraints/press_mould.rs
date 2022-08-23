use std::ops::{Add, Rem, Sub};

use uom::num_traits::Signed;

// for the lack of a better word
pub trait PressMould<T> {
    fn at_least(&mut self, value: &mut T, at_least: T);
    fn at_most(&mut self, value: &mut T, at_most: T);
    fn in_range(&mut self, value: &mut T, at_least: T, at_most: T);
    fn not_in_range(&mut self, value: &mut T, range_from: T, range_to: T);
}

// Stupid
pub struct BouncingClamp();

impl<T> PressMould<T> for BouncingClamp
where
    T: Copy
        + Clone
        + std::fmt::Debug
        + PartialOrd
        + Add<Output = T>
        + Rem<Output = T>
        + Sub<Output = T>
        + Abs,
{
    fn at_least(&mut self, value: &mut T, at_least: T) {
        if *value < at_least {
            *value = at_least + (at_least - *value)
        }
        assert!(*value >= at_least);
    }

    fn at_most(&mut self, value: &mut T, at_most: T) {
        if *value > at_most {
            *value = at_most - (*value - at_most)
        }
        assert!(*value <= at_most);
    }

    fn in_range(&mut self, value: &mut T, at_least: T, at_most: T) {
        assert!(at_least <= at_most);

        if at_least == at_most {
            *value = at_least;
            return;
        }

        let modulo = |a: T, b: T| ((a % b) + b) % b;

        let span = at_most - at_least;
        let bounced = (modulo(*value + span, span + span) - span).abs();
        *value = bounced + at_least;

        assert!(at_least <= *value && *value <= at_most);
    }

    fn not_in_range(&mut self, value: &mut T, at_most: T, at_least: T) {
        assert!(at_most <= at_least);
        if *value > at_most && *value < at_least {
            *value = *value + (at_least - at_most);
        }
        assert!(*value < at_least || at_most < *value);
    }
}

pub trait Abs: Sized {
    fn abs(self) -> Self;
}

impl Abs for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl<D: ?Sized, U: ?Sized, V: ?Sized> Abs for uom::si::Quantity<D, U, V>
where
    D: uom::si::Dimension,
    U: uom::si::Units<V>,
    V: uom::num_traits::Num + uom::Conversion<V> + Signed,
{
    fn abs(self) -> Self {
        self.abs()
    }
}
