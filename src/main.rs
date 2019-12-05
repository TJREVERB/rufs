#[path = "modules/_mod.rs"]
mod modules;

use modules::eps::EPS;

fn main() {
    println!("Hello, world!");
    let e = EPS::new();
    println!("{}", e.address);
}
