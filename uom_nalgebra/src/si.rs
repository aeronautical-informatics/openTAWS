use core::marker::PhantomData;

use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, Matrix, RawStorage, Scalar};

use uom::{
    num::Num,
    si::{self, Units, SI},
    Conversion,
};

use paste::paste;

use crate::*;

macro_rules! si_matrix {
	($shapes:tt, [$($dim:ident),*]) => {
		$(
			pub mod $dim {

				use core::marker::PhantomData;

				use nalgebra::{Const, Matrix, RawStorage, Scalar};

				use uom::{
					num::Num,
    				si::{self, Units, SI},
    				Conversion,
				};

				paste! {
					pub use uom::si::$dim::*;

					pub mod f32 { pub use uom::si::f32::[<$dim:camel>]; }
					pub mod f64 { pub use uom::si::f64::[<$dim:camel>]; }
				}

				use paste::paste;

				use crate::*;

				si_matrix! { $shapes, $dim }
			}
		)*
	};

	([$(($r:tt, $c:tt, $args:tt));*], $dim:ident) => {
		$(
			si_matrix! { $dim, $r, $c, $args }
		)*
	};

	($dim:ident, $r:literal, 1, $args:tt) => {
		paste! {
            pub type [<$dim:camel Vector $r>]<T> = [<QuantityVector $r>]<si::$dim::Dimension, T>;

			quantity_matrix_new! { [<$dim:camel Vector $r>], $dim, $r, 1, $args }
			quantity_matrix_xyzwab! { [<$dim:camel Vector $r>], $dim, $r, 1, $args }
		}
	};

	($dim:ident, 1, $c:literal, $args:tt) => {
		paste! {
            pub type [<$dim:camel RowVector $c>]<T> = [<QuantityRowVector $c>]<si::$dim::Dimension, T>;

			quantity_matrix_new! { [<$dim:camel RowVector $c>], $dim, 1, $c, $args }
		}
	};

	($dim:ident, $r:literal, $c:literal, $args:tt) => {
		paste! {
            pub type [<$dim:camel Matrix $r x $c>]<T> = [<QuantityMatrix $r x $c>]<si::$dim::Dimension, T>;

			quantity_matrix_new! { [<$dim:camel Matrix $r x $c>], $dim, $r, $c, $args }
		}
	};
}

macro_rules! quantity_matrix_new {
	($name:ident, $dim:ident, $r:literal, $c:literal, [$($($arg:ident),*);*]) => {
		paste! {
			impl<T> $name<T>
			where
				T: Num + Conversion<T>,
				SI<T>: Units<T>
			{
				#[allow(clippy::too_many_arguments)]
				pub fn new<N>($($($arg: T),*),*) -> Self
				where
					N: si::$dim::Unit + si::$dim::Conversion<T>,
				{

					$($(
                        let $arg = si::$dim::[<$dim:camel>]::<SI<T>, T>::new::<N>($arg);
					)*)*

					Self {
						matrix: Matrix::<T, Const<$r>, Const<$c>, _>::new($($($arg.value),*),*),
						dimension: PhantomData
					}
				}

				pub fn from_matrix<N, S>(matrix: Matrix<T, Const<$r>, Const<$c>, S>) -> Self
				where
					T: Scalar,
					N: si::$dim::Unit + si::$dim::Conversion<T>,
					S: RawStorage<T, Const<$r>, Const<$c>>,
				{
					let matrix = matrix.map(|elem| si::$dim::[<$dim:camel>]::<SI<T>, T>::new::<N>(elem).value);

					Self {
						matrix,
						dimension: PhantomData
					}
				}

				pub fn zeros<N>() -> Self
				where
					T: Scalar,
                    N: si::$dim::Unit + si::$dim::Conversion<T>,
				{
					let matrix = Matrix::<T, Const<$r>, Const<$c>, _>::zeros()
                            .map(|elem| si::$dim::[<$dim:camel>]::<SI<T>, T>::new::<N>(elem).value);

                        Self {
                            matrix,
                            dimension: PhantomData
                        }
				}
			}
		}
	};
}

macro_rules! quantity_matrix_xyzwab {
    ($name:ident, $dim:ident, $r:literal, $c:literal, [$($($arg:ident),*);*]) => {
        paste! {
            impl<T> $name<T>
            where
                T: Num + Conversion<T> + Scalar,
                SI<T>: Units<T>
            {
                $($(
                    pub fn $arg<N>() -> Self
                    where
                        N: si::$dim::Unit + si::$dim::Conversion<T>,
                    {
                        let matrix = Matrix::<T, Const<$r>, Const<$c>, _>::$arg()
                            .map(|elem| si::$dim::[<$dim:camel>]::<SI<T>, T>::new::<N>(elem).value);

                        Self {
                            matrix,
                            dimension: PhantomData
                        }
                    }
                )*)*
            }
        }
    };
}

macro_rules! quantity_matrix_get {

	($shapes:tt, $dims:tt) => {
		quantity_matrix_get! { $dims }
	};

    ([$($dim:ident),*]) => {
		paste! {
			$(
				impl<T, R, C, S> QuantityMatrix<si::$dim::Dimension, T, R, C, S>
				where
					T: Num + Conversion<T> + Scalar,
					R: Dim,
					C: Dim,
					S: RawStorage<T, R, C>,
					SI<T>: Units<T>,
					DefaultAllocator: Allocator<T, R, C>,
				{
					pub fn get<N>(&self) -> Matrix<
						T,
						R,
						C,
						<DefaultAllocator as Allocator<T, R, C>>::Buffer
					>
					where
						N: si::$dim::Unit + si::$dim::Conversion<T>,
					{
						self.matrix.map(|elem| {
							si::$dim::[<$dim:camel>]::<SI<T>, T> {
								value: elem,
								dimension: PhantomData,
								units: PhantomData
							}.get::<N>()
						})
					}
				}
			)*
		}
    };
}

matrix_shapes! { si_matrix, [ratio, length, velocity, acceleration, force, momentum] }

matrix_shapes! { quantity_matrix_get, [
    absement,
    acceleration,
    amount_of_substance,
    angle,
    angular_acceleration,
    angular_jerk,
    angular_velocity,
    area,
    available_energy,
    capacitance,
    catalytic_activity,
    catalytic_activity_concentration,
    curvature,
    electric_charge,
    electric_current,
    electric_potential,
    electrical_conductance,
    electrical_resistance,
    energy,
    force,
    frequency,
    heat_capacity,
    heat_flux_density,
    heat_transfer,
    inductance,
    information,
    information_rate,
    jerk,
    length,
    luminance,
    luminous_intensity,
    magnetic_flux,
    magnetic_flux_density,
    mass,
    mass_concentration,
    mass_density,
    mass_rate,
    molar_concentration,
    molar_energy,
    molar_heat_capacity,
    molar_mass,
    momentum,
    power,
    pressure,
    ratio,
    specific_heat_capacity,
    solid_angle,
    radiant_exposure,
    temperature_interval,
    thermal_conductivity,
    thermodynamic_temperature,
    time,
    torque,
    velocity,
    volume,
    volume_rate
]}
