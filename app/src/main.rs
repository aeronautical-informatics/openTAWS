use terrain_server::*;

fn main() {
    let attitude = altitude_query();
    println!("Hello, world! {}", attitude);
}
