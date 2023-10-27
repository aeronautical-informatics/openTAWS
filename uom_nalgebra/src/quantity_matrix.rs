use core::{fmt::Debug, marker::PhantomData};

use nalgebra::{ArrayStorage, Const, Dim, Matrix, Matrix1, RawStorage, Scalar, U1};

use uom::{
    num_traits::Num,
    si::{Dimension, Quantity, Units, SI},
    Conversion,
};

use paste::paste;

use crate::matrix_shapes::matrix_shapes;

pub mod indexing;
pub mod norm;
pub mod ops;

macro_rules! quantity_matrix {

	([$(($r:tt, $c:tt, $args:tt));*], $extra:tt) => {
		$(
			quantity_matrix! { $r, $c }
		)*
	};

	($r:literal, 1) => {
		paste! {
			pub type [<QuantityVector $r>]<D, T> = QuantityVector<D, T, Const<$r>, ArrayStorage<T, $r, 1>>;
		}
	};

	(1, $c:literal) => {
		paste! {
			pub type [<QuantityRowVector $c>]<D, T> = QuantityRowVector<D, T, Const<$c>, ArrayStorage<T, 1, $c>>;
		}
	};


	($r:literal, $c:literal) => {
		paste! {
			pub type [<QuantityMatrix $r x $c>]<D, T> = QuantityMatrix<D, T, Const<$r>, Const<$c>, ArrayStorage<T, $r, $c>>;
		}
	};
}

matrix_shapes! { quantity_matrix }

pub type QuantityVector<D, T, R, S> = QuantityMatrix<D, T, R, U1, S>;
pub type QuantityRowVector<D, T, C, S> = QuantityMatrix<D, T, U1, C, S>;

pub struct QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
{
    pub(crate) matrix: Matrix<T, R, C, S>,
    pub(crate) dimension: PhantomData<D>,
}

impl<D, T, R, C, S> Copy for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Copy,
    R: Dim,
    C: Dim,
    S: Copy,
    SI<T>: Units<T>,
{
}

impl<D, T, R, C, S> Clone for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Clone,
    R: Dim,
    C: Dim,
    S: Clone,
    SI<T>: Units<T>,
{
    fn clone(&self) -> Self {
        Self {
            matrix: self.matrix.clone(),
            dimension: self.dimension,
        }
    }
}

impl<D, T, R, C, S> Debug for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Debug,
    R: Dim,
    C: Dim,
    S: Debug,
    SI<T>: Units<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        //ToDo: hmm, ugly; use ::new::<base_unit>(T::one()); if si::$dim::base_unit becomes a thing)
        let unit = Quantity::<D, SI<T>, T> {
            value: T::one(),
            dimension: PhantomData,
            units: PhantomData,
        };

        f.write_fmt(format_args!("{:?} * {:?}", self.matrix, unit))
    }
}

impl<D, T> From<Quantity<D, SI<T>, T>> for QuantityMatrix<D, T, U1, U1, ArrayStorage<T, 1, 1>>
where
    D: Dimension + ?Sized,
    T: Num + Scalar + Conversion<T>,
    SI<T>: Units<T>,
{
    fn from(quantity: Quantity<D, SI<T>, T>) -> Self {
        Self {
            matrix: Matrix1::new(quantity.value),
            dimension: PhantomData,
        }
    }
}

impl<D, T, S> From<QuantityMatrix<D, T, U1, U1, S>> for Quantity<D, SI<T>, T>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Copy,
    S: RawStorage<T, U1>,
    SI<T>: Units<T>,
{
    fn from(quantity: QuantityMatrix<D, T, U1, U1, S>) -> Self {
        Quantity {
            value: quantity.matrix[(0, 0)],
            dimension: PhantomData,
            units: PhantomData,
        }
    }
}

impl<T, R, C, S> From<Matrix<T, R, C, S>> for QuantityMatrix<uom::si::ratio::Dimension, T, R, C, S>
where
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
{
    fn from(matrix: Matrix<T, R, C, S>) -> Self {
        Self {
            matrix,
            dimension: PhantomData,
        }
    }
}
