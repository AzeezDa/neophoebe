use std::collections::{HashSet, HashMap};

use rand::{Rng, prelude::Distribution};

use super::{Parameters, Relations};

fn rand_exp(lambda: f64) -> f64 {
    let r = rand::random::<f64>();
    -(1.-r).ln()/lambda
}

pub struct Population {
    susceptible: HashSet<usize>,
    exposed: HashSet<usize>,
    contagious: HashSet<usize>,
    recovered: HashSet<usize>,
    deceased: HashSet<usize>,
    exposed_timers: HashMap<usize, f64>,
    contagious_timers: HashMap<usize, (f64, f64)>
}

impl Population {
    pub fn new(params: &Parameters) -> Self {
        let mut susceptible  = HashSet::from_iter(0..params.population_size);

        let patient_zero = (&mut rand::thread_rng()).gen_range(0..params.population_size);
        susceptible.remove(&patient_zero);
        let mut contagious = HashSet::new();
        contagious.insert(patient_zero);

        Self {
            susceptible,
            exposed: HashSet::new(),
            contagious,
            recovered: HashSet::new(),
            deceased: HashSet::new(),
            exposed_timers: HashMap::new(),
            contagious_timers: HashMap::new()
        }
    }

    pub fn update(&mut self, params: &Parameters, relations: &Relations) {
        let mut to_remove = vec![];

        for &i in self.susceptible.iter() {
            let mut p = 1.;
            for &j in self.contagious.iter() {
                p *= 1. - relations.get(i, j) * (1. - params.hygenicity).powi(2) * params.disease_spread;
            }
            if rand::distributions::Bernoulli::new(1. - p).unwrap().sample(&mut rand::thread_rng()) {
                to_remove.push(i);
            }
        }
        for &i in to_remove.iter() {
            self.susceptible.remove(&i);
            self.exposed.insert(i);
            self.exposed_timers.insert(i, rand_exp(params.disease_incubation));
        }

        to_remove.clear();

        for (&p, incubation) in self.exposed_timers.iter_mut() {
            *incubation -= 1.;
            if *incubation <= 0. {
                to_remove.push(p);
            }
        }
        for i in to_remove.iter() {
            self.exposed.remove(i);
            self.exposed_timers.remove(i);
            self.contagious.insert(*i);
            self.contagious_timers.insert(*i, (rand_exp(params.disease_recovery), rand_exp(params.disease_mortality)));
        }

        to_remove.clear();

        for (&p, (recovery, mortality)) in self.contagious_timers.iter_mut() {
            *recovery -= 1.;
            *mortality -= 1.;
            if *recovery <= 0. && *recovery <= *mortality {
                self.recovered.insert(p);
                to_remove.push(p);
            } else if *mortality <= 0. && *recovery > *mortality {
                self.deceased.insert(p);
                to_remove.push(p);
            }
        }
        for i in to_remove.iter() {
            self.contagious.remove(i);
            self.contagious_timers.remove(i);
        }
    }

    pub fn get_sizes(&self) -> String {
        format!("{},{},{},{},{}\n", self.susceptible.len(), self.exposed.len(), self.contagious.len(), self.recovered.len(), self.deceased.len())
    }
}