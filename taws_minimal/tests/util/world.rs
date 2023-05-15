use cucumber::World;
use opentaws::prelude::*;
use taws_minimal::MinimalTaws;

use super::{aircraft_state::AircraftStateGenerator, constraints::AircraftStateConstraints};

#[derive(World)]
pub struct MyWorld {
    pub taws: MinimalTaws,
    pub phases: Vec<AircraftStateConstraints>,
    pub test_length: usize,
    pub phase: usize,

    pub taws_constraints: AircraftStateConstraints,
    pub state_gen: AircraftStateGenerator,
}

impl Default for MyWorld {
    fn default() -> Self {
        let mut taws_constraints = AircraftStateConstraints::default();

        let max_altitude_gnd = Length::new::<length::foot>(330_000.0);
        taws_constraints.add_altitude_ground_constraint(super::constraints::Constraint::InRange(
            Length::new::<length::foot>(0.0),
            max_altitude_gnd,
        ));

        let min_max_climb_rate = Velocity::new::<velocity::foot_per_minute>(680_000.0);
        taws_constraints.add_climb_rate_constraint(super::constraints::Constraint::InRange(
            -min_max_climb_rate,
            min_max_climb_rate,
        ));

        Self {
            taws: MinimalTaws::new(),
            phases: vec![taws_constraints.clone()],
            test_length: 100,
            phase: 0,
            taws_constraints: taws_constraints.clone(),
            state_gen: AircraftStateGenerator::default(),
        }
    }
}

impl std::fmt::Debug for MyWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyWorld").finish()
    }
}

impl MyWorld {
    pub fn next_phase(&mut self) {
        self.phases.push(self.taws_constraints.clone());
        self.phase += 1;
    }

    pub fn next_aircraft_state(&mut self) -> AircraftState {
        self.state_gen.next().unwrap()
    }

    pub fn next_aircraft_states(&mut self, n: usize) -> Vec<AircraftState> {
        (0..n).map(|_| self.next_aircraft_state()).collect()
    }
}
