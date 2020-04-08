use terrain_server;

use cucumber::{after, before, cucumber, steps};

pub struct MyWorld {
    // Struct for mutable context in scenarios.
    test_status: String,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            test_status: "World is created".to_string(),
        }
    }
}

mod test_steps {
    use super::*;
    use cucumber::steps;
    use terrain_server::{GeograpraphicPosition, TerrainServerStatus};

    // Any type that implements cucumber::World + Default can be the world
    steps!(MyWorld => {
        
        given "terrain server up and running" |world, step| {
            world.test_status = "Creating an instance of a terrain server".to_string();
            // Set up your context in given steps
            assert_eq!(terrain_server::status(), TerrainServerStatus::OK);
            
        };

        when "user makes an altitude query with the position in geographic coordinates" |world, step| {
            // An sample test point
            let test_position = GeograpraphicPosition{
                latitude:39.0,
                longitude:42.0,
            };

        };

        then "the altitute of the terrain at that point above the mean sea level is given in meters" |world, step| {
            // Check that the outcomes to be observed have occurred
            let altitude = terrain_server::altitude_query();
            assert_eq!(altitude, 0);
        };

    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |scenario| {

});

// A setup function to be called before everything else
fn setup() {}

cucumber! {
    features: "../../features", // Path to our feature files
    world: MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        test_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ],
    after: &[
        an_after_fn // Optional; called after each scenario
    ]
}
