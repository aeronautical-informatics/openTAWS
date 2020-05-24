extern crate rspec;

use geo::Point;
use std::io;
use std::sync::Arc;
use terrain_server::TerrainServer;

use terrain_server;

#[test]
fn altitude_spec() {
    let logger = Arc::new(rspec::Logger::new(io::stdout()));
    let configuration = rspec::ConfigurationBuilder::default().build().unwrap();
    let runner = rspec::Runner::new(configuration, vec![logger]);

    #[derive(Clone, Debug)]
    struct Environment {
        position: Point<f64>,
        altitude: f64,
        sut: TerrainServer,
    }

    let environment = Environment {
        position: (0., 0.).into(),
        altitude: 0.0,
        sut: TerrainServer::default(),
    };

    rspec::run(&rspec::describe("Altitude query", environment, |ctx| {
        ctx.specify("a position in geographical coordinates", |ctx| {
            ctx.it(
                "should return the altitude of the terrain at that position",
                |env| {
                    assert_eq!(env.sut.altitude(env.position), env.altitude);
                },
            );
        });
    }));
}
