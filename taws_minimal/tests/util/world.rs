use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::WorldInit;
use opentaws::prelude::*;
use taws_minimal::MinimalTaws;

use super::constraints::{AircraftStateConstraints, ConstraintAirportDatabase};

static AIRPORT_DATABASE: ConstraintAirportDatabase = ConstraintAirportDatabase {};

lazy_static::lazy_static! {
    static ref TAWS_CONFIG: TawsConfig<'static> = TawsConfig{
            terrain_server: &AIRPORT_DATABASE,
            max_climbrate: Velocity::new::<foot_per_minute>(700.0),
            max_climbrate_change: Acceleration::new::<foot_per_second_squared>(100.0),
    };
}

#[derive(WorldInit)]
pub struct MyWorld {
    pub taws: MinimalTaws<'static>,
    pub phases: Vec<AircraftStateConstraints>,
    pub test_length: usize,
    pub phase: usize,
}

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            taws: MinimalTaws::new(&TAWS_CONFIG),
            phases: vec![AircraftStateConstraints::default()],
            test_length: 10,
            phase: 0,
        })
    }
}

impl std::fmt::Debug for MyWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyWorld").finish()
    }
}

impl MyWorld {
    pub fn next_phase(&mut self) {
        self.phases.push(AircraftStateConstraints::default());
        self.phase += 1;
    }
}

impl std::panic::UnwindSafe for MyWorld {} // This is a lie, but what they gonna do, panic?
