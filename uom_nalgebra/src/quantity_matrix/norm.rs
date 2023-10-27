use core::{marker::PhantomData, ops::Mul};

use nalgebra::{Dim, Matrix, Normed, RealField, Scalar, Storage};

use uom::{
    num_traits::Num,
    si::{Dimension, Quantity, Units, SI},
    Conversion,
};

use crate::QuantityMatrix;

impl<D, T, R, C, S> QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Scalar,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
{
    pub fn norm(&self) -> Quantity<D, SI<T>, T>
    where
        T: RealField,
        S: Storage<T, R, C>,
    {
        Quantity {
            value: self.matrix.norm(),
            dimension: PhantomData,
            units: PhantomData,
        }
    }

    pub fn norm_squared<DSq>(&self) -> Quantity<DSq, SI<T>, T>
    where
        DSq: Dimension + ?Sized,
        T: RealField,
        S: Storage<T, R, C>,
        Quantity<D, SI<T>, T>: Mul<Quantity<D, SI<T>, T>, Output = Quantity<DSq, SI<T>, T>>,
    {
        Quantity {
            value: self.matrix.norm_squared(),
            dimension: PhantomData,
            units: PhantomData,
        }
    }

    pub fn normalize(self) -> nalgebra::Unit<Matrix<T, R, C, S>>
    where
        Matrix<T, R, C, S>: Normed,
    {
        nalgebra::Unit::<Matrix<T, R, C, S>>::new_normalize(self.matrix)
    }
}
