mod ffac;
//mod flta;
mod mode1;
mod mode3;
mod pda;

use core::{fmt::Display, slice::Iter};
use hash32::{Hash, Hasher};

use crate::alerts::Alert;
use crate::prelude::*;

pub use {ffac::*, /*flta::*,*/ mode1::*, mode3::*, pda::*};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ClassC_Source {
    Ffac,
    //Flta,
    Mode1,
    Mode3,
    Pda,
}

impl Display for ClassC_Source {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ClassC_Source::Ffac => f.write_fmt(format_args!("Ffac")),
            //SourceClassC::Flta => f.write_fmt(format_args!("Flta")),
            ClassC_Source::Mode1 => f.write_fmt(format_args!("Mode1")),
            ClassC_Source::Mode3 => f.write_fmt(format_args!("Mode3")),
            ClassC_Source::Pda => f.write_fmt(format_args!("Pda")),
        }
    }
}

impl TawsAlertSource for ClassC_Source {
    const NUM_ALERT_SOURCES: usize = 5;
    const ALERT_SOURCES: &'static [Self] = &[
        ClassC_Source::Ffac,
		//ClassC_Source::Flta,
        ClassC_Source::Mode1,
        ClassC_Source::Mode3,
        ClassC_Source::Pda,
    ];
}

impl IntoIterator for ClassC_Source {
    type Item = &'static ClassC_Source;

    type IntoIter = Iter<'static, ClassC_Source>;

    fn into_iter(self) -> Self::IntoIter {
        [Self::Ffac, Self::Mode1, Self::Mode3, Self::Pda].iter()
    }
}

impl Hash for ClassC_Source {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(&(*self as usize).to_le_bytes())
    }
}

pub struct ClassC {
    ffac: Ffac,
    //flta: flta::Flta,
    mode1: Mode1,
    mode3: Mode3,
    pda: Pda,
}

impl ClassC {
    pub fn new() -> Self {
        ClassC {
            ffac: Ffac::default(),
            //flta: flta:::Flta::default(),
            mode1: Mode1::default(),
            mode3: Mode3::default(),
            pda: Pda::default(),
        }
    }
}

impl TawsFunctionalities for ClassC {
    type AlertSource = ClassC_Source;
    type Alert = Alert<Self::AlertSource>;

    fn functionality(
        &self,
        alert_src: Self::AlertSource,
    ) -> &dyn TawsFunctionality<AlertSource = Self::AlertSource, Alert = Self::Alert> {
        match alert_src {
            ClassC_Source::Ffac => &self.ffac,
			//ClassC_Source::Flta => &self.flta,
            ClassC_Source::Mode1 => &self.mode1,
            ClassC_Source::Mode3 => &self.mode3,
            ClassC_Source::Pda => &self.pda,
        }
    }

    fn functionality_mut(
        &mut self,
        alert_src: Self::AlertSource,
    ) -> &mut dyn TawsFunctionality<AlertSource = Self::AlertSource, Alert = Self::Alert> {
        match alert_src {
            ClassC_Source::Ffac => &mut self.ffac,
			//ClassC_Source::Flta => &mut self.flta,
            ClassC_Source::Mode1 => &mut self.mode1,
            ClassC_Source::Mode3 => &mut self.mode3,
            ClassC_Source::Pda => &mut self.pda,
        }
    }
}

#[derive(Debug)]
pub enum ClassCError {
    //AlertSourceNotImplemented,
    InvalidAircraftState,
}

impl Display for ClassCError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            /*ClassCError::AlertSourceNotImplemented => {
                f.write_fmt(format_args!("Alert source is not implemented."))
            }*/
            ClassCError::InvalidAircraftState => {
                f.write_fmt(format_args!("Invalid aircraft state.",))
            }
        }
    }
}

impl TawsError for ClassCError {}

impl From<ClassCError> for &dyn TawsError {
    fn from(err: ClassCError) -> Self {
        match err {

            ClassCError::InvalidAircraftState => &ClassCError::InvalidAircraftState,
        }
    }
}
