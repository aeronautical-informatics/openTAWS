use core::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};

use nalgebra::{
    allocator::Allocator, ClosedAdd, ClosedMul, ClosedSub, DefaultAllocator, Dim, Matrix,
    RawStorage, Scalar, U1, U3,
};

use uom::{
    num_traits::Num,
    si::{Dimension, Quantity, Units, SI},
    Conversion,
};

use crate::QuantityMatrix;

impl<D, T, R, C, S> Add for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: Add<Output = Matrix<T, R, C, S>>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            matrix: self.matrix + rhs.matrix,
            dimension: PhantomData,
        }
    }
}

impl<D, T, R, C, S> AddAssign for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.matrix += rhs.matrix
    }
}

impl<D, T, R, C, S> Sub for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: Sub<Output = Matrix<T, R, C, S>>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            matrix: self.matrix - rhs.matrix,
            dimension: PhantomData,
        }
    }
}

impl<D, T, R, C, S> SubAssign for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.matrix -= rhs.matrix;
    }
}

impl<D, DRhs, DRes, T, R, C, S> Mul<QuantityMatrix<DRhs, T, R, C, S>> for Quantity<D, SI<T>, T>
where
    D: Dimension + ?Sized,
    DRhs: Dimension + ?Sized,
    DRes: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    T: Mul<Matrix<T, R, C, S>, Output = Matrix<T, R, C, S>>,
    Quantity<D, SI<T>, T>: Mul<Quantity<DRhs, SI<T>, T>, Output = Quantity<DRes, SI<T>, T>>,
{
    type Output = QuantityMatrix<DRes, T, R, C, S>;

    fn mul(self, rhs: QuantityMatrix<DRhs, T, R, C, S>) -> Self::Output {
        QuantityMatrix {
            matrix: self.value * rhs.matrix,
            dimension: PhantomData,
        }
    }
}

impl<D, DRhs, DRes, T, R, C, S> Mul<Quantity<DRhs, SI<T>, T>> for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    DRhs: Dimension + ?Sized,
    DRes: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: Mul<T, Output = Matrix<T, R, C, S>>,
    Quantity<D, SI<T>, T>: Mul<Quantity<DRhs, SI<T>, T>, Output = Quantity<DRes, SI<T>, T>>,
{
    type Output = QuantityMatrix<DRes, T, R, C, S>;

    fn mul(self, rhs: Quantity<DRhs, SI<T>, T>) -> Self::Output {
        QuantityMatrix {
            matrix: self.matrix * rhs.value,
            dimension: PhantomData,
        }
    }
}

impl<D, DRhs, T, R, C, S> MulAssign<Quantity<DRhs, SI<T>, T>> for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    DRhs: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: MulAssign<T>,
    Quantity<D, SI<T>, T>: MulAssign<Quantity<DRhs, SI<T>, T>>,
{
    fn mul_assign(&mut self, rhs: Quantity<DRhs, SI<T>, T>) {
        self.matrix *= rhs.value
    }
}

impl<D, DRhs, DRes, T, R, C, S> Div<Quantity<DRhs, SI<T>, T>> for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    DRhs: Dimension + ?Sized,
    DRes: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: Div<T, Output = Matrix<T, R, C, S>>,
    Quantity<D, SI<T>, T>: Div<Quantity<DRhs, SI<T>, T>, Output = Quantity<DRes, SI<T>, T>>,
{
    type Output = QuantityMatrix<DRes, T, R, C, S>;

    fn div(self, rhs: Quantity<DRhs, SI<T>, T>) -> Self::Output {
        QuantityMatrix {
            matrix: self.matrix / rhs.value,
            dimension: PhantomData,
        }
    }
}

impl<D, DRhs, DRes, T, R, RRhs, RRes, C, CRhs, CRes, S, SRhs, SRes>
    Mul<QuantityMatrix<DRhs, T, RRhs, CRhs, SRhs>> for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    DRhs: Dimension + ?Sized,
    DRes: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    RRhs: Dim,
    RRes: Dim,
    C: Dim,
    CRhs: Dim,
    CRes: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: Mul<Matrix<T, RRhs, CRhs, SRhs>, Output = Matrix<T, RRes, CRes, SRes>>,
    Quantity<D, SI<T>, T>: Mul<Quantity<DRhs, SI<T>, T>, Output = Quantity<DRes, SI<T>, T>>,
{
    type Output = QuantityMatrix<DRes, T, RRes, CRes, SRes>;

    fn mul(self, rhs: QuantityMatrix<DRhs, T, RRhs, CRhs, SRhs>) -> Self::Output {
        QuantityMatrix {
            matrix: self.matrix * rhs.matrix,
            dimension: PhantomData,
        }
    }
}

impl<D, DRhs, T, R, RRhs, C, CRhs, S, SRhs> MulAssign<QuantityMatrix<DRhs, T, RRhs, CRhs, SRhs>>
    for QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    DRhs: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    RRhs: Dim,
    C: Dim,
    CRhs: Dim,
    SI<T>: Units<T>,
    Matrix<T, R, C, S>: MulAssign<Matrix<T, RRhs, CRhs, SRhs>>,
    Quantity<D, SI<T>, T>: MulAssign<Quantity<DRhs, SI<T>, T>>,
{
    fn mul_assign(&mut self, rhs: QuantityMatrix<DRhs, T, RRhs, CRhs, SRhs>) {
        self.matrix *= rhs.matrix;
    }
}

impl<D, T, R, C, S> QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    SI<T>: Units<T>,
{
    pub fn transpose(
        &self,
    ) -> QuantityMatrix<D, T, C, R, <DefaultAllocator as Allocator<T, C, R>>::Buffer>
    where
        T: Scalar,
        S: RawStorage<T, R, C>,
        DefaultAllocator: Allocator<T, R, C>,
        DefaultAllocator: Allocator<T, C, R>,
    {
        QuantityMatrix {
            matrix: self.matrix.transpose(),
            dimension: PhantomData,
        }
    }
}

impl<D, T, R, C, S> QuantityMatrix<D, T, R, C, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
    SI<T>: Units<T>,
{
    pub fn dot<DRhs, DRes, SRhs>(
        &self,
        other: &QuantityMatrix<DRhs, T, R, C, SRhs>,
    ) -> Quantity<DRes, SI<T>, T>
    where
        T: Scalar + ClosedAdd + ClosedMul,
        DRhs: Dimension + ?Sized,
        DRes: Dimension + ?Sized,
        SRhs: RawStorage<T, R, C>,
        Quantity<D, SI<T>, T>: Mul<Quantity<DRhs, SI<T>, T>, Output = Quantity<DRes, SI<T>, T>>,
    {
        Quantity {
            value: self.matrix.dot(&other.matrix),
            dimension: PhantomData,
            units: PhantomData,
        }
    }
}

impl<D, T, S> QuantityMatrix<D, T, U3, U1, S>
where
    D: Dimension + ?Sized,
    T: Num + Conversion<T>,
    S: RawStorage<T, U3, U1>,
    SI<T>: Units<T>,
{
    pub fn cross<DRhs, DRes, SRhs>(
        &self,
        other: &QuantityMatrix<DRhs, T, U3, U1, SRhs>,
    ) -> QuantityMatrix<DRes, T, U3, U1, <DefaultAllocator as Allocator<T, U3, U1>>::Buffer>
    where
        T: Scalar + ClosedAdd + ClosedSub + ClosedMul,
        DRhs: Dimension + ?Sized,
        DRes: Dimension + ?Sized,
        SRhs: RawStorage<T, U3, U1>,
        Quantity<D, SI<T>, T>: Mul<Quantity<DRhs, SI<T>, T>, Output = Quantity<DRes, SI<T>, T>>,
        DefaultAllocator: Allocator<T, U3, U1>,
    {
        QuantityMatrix::<DRes, T, U3, U1, <DefaultAllocator as Allocator<T, U3, U1>>::Buffer> {
            matrix: self.matrix.cross(&other.matrix),
            dimension: PhantomData,
        }
    }
}
