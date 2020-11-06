use async_trait::async_trait;
use std::convert::Infallible;

use otaws::{types::TAWSConfig, TAWS};

pub struct MyWorld {
    taws: TAWS,
}

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            taws: TAWS::new(TAWSConfig::default()),
        })
    }
}

mod example_steps {
    use cucumber::{t, Steps};

    pub fn steps() -> Steps<crate::MyWorld> {
        let mut builder: Steps<crate::MyWorld> = Steps::new();

        builder
            .given("the plane is flying", |world, _step| world)
            .then("Mode 1 shall be armed", |world, _step| {
                assert!(world.taws.is_armed());
                world
            })
            .given("Mode 1 is armed", |world, _step| world);

        builder
    }
}

fn main() {
    let runner = cucumber::Cucumber::<MyWorld>::new()
        .features(&["features"])
        .steps(example_steps::steps());

    futures::executor::block_on(runner.run());
}
