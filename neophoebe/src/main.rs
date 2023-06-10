mod simulation;
use simulation::*;

fn main() {
    let s = Simulation::new("../haha.ron".into()).run(1000).report();

    println!("{s}");
}
