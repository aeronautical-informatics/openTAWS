#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

use opentaws::class_c::*;
use opentaws::prelude::*;

pub type AlertSource = opentaws::class_c::ClassC_Source;
pub type Alert = opentaws::alerts::Alert<ClassC_Source>;
pub type Alerts = opentaws::alerts::Alerts<Alert>;

pub struct MinimalTaws {
    functionalities: ClassC,
}

impl MinimalTaws {
    pub fn new() -> Self {
        Self {
            functionalities: ClassC::new(),
        }
    }
}

impl Taws for MinimalTaws {
    type AlertSource = AlertSource;

    type Alert = Alert;

    type Alerts = Alerts;

    type Functionalities = ClassC;

    fn functionalities(
        &self,
    ) -> &dyn TawsFunctionalities<AlertSource = Self::AlertSource, Alert = Self::Alert> {
        &self.functionalities
    }

    fn functionalities_mut(
        &mut self,
    ) -> &mut dyn TawsFunctionalities<AlertSource = Self::AlertSource, Alert = Self::Alert> {
        &mut self.functionalities
    }
}
