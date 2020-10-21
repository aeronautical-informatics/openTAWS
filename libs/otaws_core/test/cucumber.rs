use async_trait::async_trait;
use std::convert::Infallible;

use otaws_core::types;


pub struct Nothing();

#[async_trait(?Send)]
impl cucumber::World for Nothing {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self ())
    }
}


mod example_steps {
    use cucumber::{Steps, t};

    pub fn steps() -> Steps<crate::Nothing> {
        let mut builder: Steps<crate::Nothing> = Steps::new();
        builder
    }
}

fn main() {
    let runner = cucumber::Cucumber::<Nothing>::new()
        .features(&["../../features/taws"])
        .steps(example_steps::steps())
        ;

    futures::executor::block_on(runner.run());
}
