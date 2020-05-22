extern crate rspec;

use std::io;
use std::sync::Arc;

use terrain_server;

pub fn main () {

    let logger = Arc::new(rspec::Logger::new(io::stdout()));
    let configuration = rspec::ConfigurationBuilder::default().build().unwrap();
    let runner = rspec::Runner::new(configuration, vec![logger]);

    let environment = Environment {
        sut: TerrainServer()
    }

    runner::run(
        &rspec::describe("Altitude query", 0, |ctx| {
            ctx.specify("a position in geographical coordinates", |ctx| {
                ctx.it("should return the altitude of the terrain at that position", |num| {
                    assert_eq!(*num, 15);
                });
            });
        }));

}
