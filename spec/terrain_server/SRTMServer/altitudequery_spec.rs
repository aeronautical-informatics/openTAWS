extern crate rspec;

use std::io;
use std::sync::Arc;
pub use geo::Point;

use terrain_server;

pub fn main () {

    let logger = Arc::new(rspec::Logger::new(io::stdout()));
    let configuration = rspec::ConfigurationBuilder::default().build().unwrap();
    let runner = rspec::Runner::new(configuration, vec![logger]);

    struct Environment {
        position: Point<f64>,
        altitude: i64,
        sut: TerrainServer,
    }

    let environment = Environment{
        position: (0., 0.).into(),
        altidude = 0,
        sut: TerrainServer.new(),
    }

    runner::run(
        &rspec::describe("Altitude query", environment, |ctx| {
            ctx.specify("a position in geographical coordinates", |ctx| {
                ctx.it("should return the altitude of the terrain at that position", |env| {
                    assert_eq!(env.sut.altitude(env.position), env.altitude);
                });
            });
        }));

}
