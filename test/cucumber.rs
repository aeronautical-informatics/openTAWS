use std::convert::Infallible;
use std::str::FromStr;

use async_trait::async_trait;

use uom::si::{f64::*, length::foot, velocity::foot_per_minute};

use cucumber::{t, Steps};
use otaws::{AircraftState, AircraftStateReceiver, AlertLevel, Functionality, TAWSConfig, TAWS};

struct ScenarioContext {}

pub struct MyWorld {
    taws: TAWS,
    template_frame: AircraftState,

    props: ScenarioProperties,
}

#[derive(Default)]
struct ScenarioProperties {
    height_min: Option<Length>,
    height_max: Option<Length>,
    height_outside: Option<bool>,
    rate_of_descent_min: Option<Velocity>,
}

impl std::panic::UnwindSafe for MyWorld {}

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            taws: TAWS::new(Default::default()),
            template_frame: Default::default(),
            props: Default::default(),
        })
    }
}

pub fn steps() -> Steps<crate::MyWorld> {
    let mut builder: Steps<crate::MyWorld> = Steps::new();

    builder
        .given("the plane is flying", |world, _step| world)
        .given_regex("(.+) is armed", |world, mut matches, _step| {
            matches[1].retain(|c| !c.is_whitespace());
            let functionality = matches[1].parse().unwrap();
            //if matches[2].starts_with("not") {
            //    assert!(!world.taws.function_is_armed(&functionality));
            //} else {
            assert!(world.taws.function_is_armed(&functionality));
            //}
            world
        })
        .given_regex("(.+) is (.*)inhibited", |mut world, mut matches, _step| {
            matches[1].retain(|c| !c.is_whitespace());
            let functionality = matches[1].parse().unwrap();
            if matches[2].starts_with("not") {
                world.taws.function_uninhibit(&functionality);
            } else {
                world.taws.function_inhibit(&functionality);
            }
            world
        })
        .given_regex(
            r"steep approach is (.*)selected",
            |mut world, matches, _step| {
                if matches[1].starts_with("not") {
                    world.template_frame.steep_approach = false;
                } else {
                    world.template_frame.steep_approach = true;
                }
                world
            },
        )
        .then_regex(r"(.+) shall be armed", |world, mut matches, _step| {
            matches[1].retain(|c| !c.is_whitespace());
            let functionalitiy = matches[1].parse().unwrap();
            assert!(world.taws.function_is_armed(&functionalitiy));
            world
        })
        .when_regex(
            r"the rate of descent is at least (\d+) feet per minute",
            |mut world, matches, _step| {
                world.props.rate_of_descent_min = Some(Velocity::new::<foot_per_minute>(
                    matches[1].parse().unwrap(),
                ));
                world
            },
        )
        .when_regex(
            r"the height above terrain is (.*)between (\d+) and (\d+) feet",
            |mut world, matches, _step| {
                world.props.height_outside = Some(matches[1].starts_with("not"));
                world.props.height_min = Some(Length::new::<foot>(matches[2].parse().unwrap()));
                world.props.height_max = Some(Length::new::<foot>(matches[3].parse().unwrap()));
                world
            },
        )
        .then_regex(
            "a (.*) alert is not emitted at all",
            |mut world, matches, _step| {
                let alert: AlertLevel = matches[1].parse().unwrap();

                let new_frame = world.template_frame.clone();

                let alert_state = world.taws.push(&new_frame);

                assert!(alert_state.alerts.is_empty());
                assert!(alert_state.nuisance_alerts.is_empty());

                world
            },
        )
        .then_regex(
            r"a (.*) alert is emitted within (\d+) seconds",
            |mut world, matches, _step| {
                let alert: AlertLevel = matches[1].parse().unwrap();

                let new_frame = world.template_frame.clone();

                let alert_state = world.taws.push(&new_frame);

                assert!(alert_state.count(alert) > 0);

                world
            },
        );
    //.given_regex(
    //    r"the rate of rage is at least (.+) feet per minute",
    //    |world, matches, _step| {
    //        panic!("{}", matches[1]);
    //        world
    //    },
    //);

    builder
}
// Questions:
//
// + Decouple receival of Alert States and pushing of new AircraftState? Maybe yes!
// + Add time (maybe tick/frame counter) to AircraftState
// +

fn main() {
    let runner = cucumber::Cucumber::<MyWorld>::new()
        .features(&["features"])
        .steps(steps());

    futures::executor::block_on(runner.run());
}
