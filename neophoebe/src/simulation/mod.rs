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

pub struct Simulation {
    population: Population,
    relations: Relations,
    parameters: Parameters,
    log: String
}

impl Simulation {
    pub fn new(file: String) -> Self {
        let params = Parameters::read(file).unwrap();
        
        Self {
            population: Population::new(&params),
            relations: Relations::new(&params),
            parameters: params,
            log: format!("t,s,e,c,r,d\n")
        }
    }

    pub fn run(&mut self, days: usize) -> &Self {
        for i in 1..=days {
            self.population.update(&self.parameters, &self.relations);
            self.log.push_str(&format!("{},{}", i, self.population.get_sizes()));
        }
        return self
    }

    pub fn report(&self) -> String {
        self.log.clone()
    }
}