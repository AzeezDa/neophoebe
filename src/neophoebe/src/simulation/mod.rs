mod relations;
pub use relations::*;

mod parameters;
pub use parameters::*;

mod population;
pub use population::*;

mod lower_matrix;
pub use lower_matrix::*;

mod restriction;
pub use restriction::*;

/// Struct that encapsulates the entire simulation
pub struct Simulation {
    population: Population,
    relations: Relations,
    parameters: Parameters,
    log: String
}

impl Simulation {
    /// Set up the simulator using the given a filepath to the simulator's parameters RON file
    pub fn new(file: String) -> Self {
        let params = Parameters::read(file).unwrap();
        
        Self {
            population: Population::new(&params),
            relations: Relations::new(&params),
            parameters: params,
            log: format!("t,s,e,c,r,d\n")
        }
    }

    /// Run the simulator for given amount of days
    pub fn run(&mut self, days: usize) -> &Self {
        for i in 1..=days {
            self.population.update(&self.parameters, &self.relations);
            self.log.push_str(&format!("{},{}", i, self.population.get_sizes()));
        }
        return self
    }

    // Return a copy of the internally kept log
    pub fn report(&self) -> String {
        self.log.clone()
    }
}