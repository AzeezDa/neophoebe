mod simulation;
use simulation::*;

fn main() {
    let s = Simulation::new("../parameters.ron".into())
        .run(1000)
        .report();

    println!("{s}");
}
