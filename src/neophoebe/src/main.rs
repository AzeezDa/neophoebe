mod simulation;
use simulation::*;

use std::env::args;

fn main() {
    let a: Vec<String> = args().collect();
    let parameters_file = a.get(1).expect("No Parameters' File Provided");
    let mut days: usize = 500;
    if let Some(d) = a.get(2) {
        days = d.parse().expect("Error day format");
    }

    let s = Simulation::new(parameters_file.clone())
        .run(days)
        .report();

    println!("{s}");
}
