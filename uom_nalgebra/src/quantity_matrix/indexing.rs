use core::marker::PhantomData;

use nalgebra::{indexing::MatrixIndex, Dim, Matrix, RawStorage, Scalar};

use uom::{
    num_traits::Num,
    si::{Dimension, Quantity, Units, SI},
    Conversion,
};

use crate::QuantityMatrix;

pub trait IndexResult<D>
where
    D: Dimension + ?Sized,
{
    type Output;

    fn get(self) -> Self::Output;
}

impl<D, T> IndexResult<D> for &T
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Scalar + Copy,
    SI<T>: Units<T>,
{
    type Output = Quantity<D, SI<T>, T>;

    fn get(self) -> Self::Output {
        Quantity::<D, SI<T>, T> {
            value: *self,
            dimension: PhantomData,
            units: PhantomData,
        }
    }
}

impl<D, T, R, C, S> IndexResult<D> for Matrix<T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Scalar,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
    SI<T>: Units<T>,
{
    type Output = QuantityMatrix<D, T, R, C, S>;

    fn get(self) -> Self::Output {
        QuantityMatrix {
            matrix: self,
            dimension: PhantomData,
        }
    }
}

impl<D, T, R, C, S> QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T> + Scalar,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
    SI<T>: Units<T>,
{
    pub fn index<'a, I, Res>(&'a self, index: I) -> Res::Output
    where
        I: MatrixIndex<'a, T, R, C, S, Output = Res>,
        Res: IndexResult<D>,
    {
        self.matrix.index(index).get()
    }
}
