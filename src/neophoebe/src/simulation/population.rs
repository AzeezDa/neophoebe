use std::collections::{HashMap, HashSet};

use rand::{prelude::Distribution, Rng};

use super::{Parameters, Relations, Restriction};

/// Generate a random exponentially distributed value given the parameter lambda
///
/// Note this uses this form the exponential form: 1/λ * e^{-x/λ} because we are inputting the expected value for the variables that are
/// exponentially distributed, so we save a step and just inverse the parameter.
fn rand_exp(lambda: f64) -> f64 {
    let r = rand::random::<f64>();
    -(1. - r).ln() * lambda
}

/// A struct that contains the compartmentation of the population and important fields e.g. how long a person has been contagious, or positively tested persons.
pub struct Population {
    susceptible: HashSet<usize>,
    exposed: HashSet<usize>,
    contagious: HashSet<usize>,
    recovered: HashSet<usize>,
    deceased: HashSet<usize>,
    exposed_timers: HashMap<usize, f64>,
    contagious_timers: HashMap<usize, (f64, f64)>,
    active_restriction: Restriction,
    positive_tested: HashSet<usize>,
}

impl Population {
    /// Given the `Parameters` struct construct a new population
    pub fn new(params: &Parameters) -> Self {
        // All persons are first susceptible
        let mut susceptible = HashSet::from_iter(0..params.population_size);

        // Infect patient zero
        let patient_zero = (&mut rand::thread_rng()).gen_range(0..params.population_size);
        susceptible.remove(&patient_zero);
        let mut contagious = HashSet::new();
        contagious.insert(patient_zero);
        let mut contagious_timers = HashMap::new();
        contagious_timers.insert(
            patient_zero,
            (
                rand_exp(params.disease_recovery),
                rand_exp(params.disease_mortality),
            ),
        );

        // Since personal restriction is always on we set it up immediately, otherwise we set no restriction
        let restriction = match params.restriction_plan {
            Restriction::PersonalRestriction(_) => params.restriction_plan,
            _ => Restriction::NoRestriction,
        };

        Self {
            susceptible,
            exposed: HashSet::new(),
            contagious,
            recovered: HashSet::new(),
            deceased: HashSet::new(),
            exposed_timers: HashMap::new(),
            contagious_timers,
            active_restriction: restriction,
            positive_tested: HashSet::new(),
        }
    }

    /// Iterate the simulation once given the parameters and the relationship matrix
    pub fn update(&mut self, params: &Parameters, relations: &Relations) {
        let spread_constant = (1. - params.hygiencity).powi(2) * params.disease_spread;
        let mut rng = rand::thread_rng();
        let mut to_remove = vec![];

        // SUSCEPTIBLE -> EXPOSED
        for &i in self.susceptible.iter() {
            let mut p = 1.;
            for &j in self.contagious.iter() {
                let mut relation = relations.get(i, j);

                match self.active_restriction {
                    Restriction::LowerCutOffRestriction(_, p, _) => {
                        if relation < p {
                            relation = 0.;
                        }
                    }
                    Restriction::UpperCutOffRestriction(_, p, _) => {
                        if relation > p {
                            relation = 0.;
                        }
                    }
                    Restriction::PersonalRestriction(p) => {
                        if self.positive_tested.contains(&j) {
                            relation *= p;
                        }
                    }
                    Restriction::CommunityRestriction(_, p, _) => {
                        relation *= p;
                    }
                    _ => {}
                }

                p *= 1. - relation * spread_constant
            }
            if rand::distributions::Bernoulli::new(1. - p)
                .unwrap()
                .sample(&mut rng)
            {
                to_remove.push(i);
            }
        }
        for &i in to_remove.iter() {
            self.susceptible.remove(&i);
            self.exposed.insert(i);
            self.exposed_timers
                .insert(i, rand_exp(params.disease_incubation));
        }

        // EXPOSED -> CONTAGIOUS
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
            self.contagious_timers.insert(
                *i,
                (
                    rand_exp(params.disease_recovery),
                    rand_exp(params.disease_mortality),
                ),
            );
        }

        // CONTAGIOUS -> RECOVERED | DECEASED
        to_remove.clear();

        for (&p, (recovery, mortality)) in self.contagious_timers.iter_mut() {
            *recovery -= 1.;
            *mortality -= 1.;
            if *recovery <= 0. {
                self.recovered.insert(p);
                to_remove.push(p);
            } else if *mortality <= 0. {
                self.deceased.insert(p);
                to_remove.push(p);
            }
        }
        for i in to_remove.iter() {
            self.contagious.remove(i);
            self.contagious_timers.remove(i);
        }

        // APPLY TESTING/SCREENING PLAN
        match params.restriction_plan {
            Restriction::NoRestriction => {}
            Restriction::CommunityRestriction(limit, _, _)
            | Restriction::LowerCutOffRestriction(limit, _, _)
            | Restriction::UpperCutOffRestriction(limit, _, _) => {
                let mut positives = 0;
                let group = rand::seq::index::sample(&mut rng, params.population_size, params.tests_per_day).into_vec();
                for i in group {
                    if self.contagious.contains(&i) || self.exposed.contains(&i) {
                        positives += 1;
                    }
                }
                if positives >= limit {
                    self.active_restriction = params.restriction_plan;
                }
            }
            Restriction::PersonalRestriction(_) => {
                let group = rand::seq::index::sample(&mut rng, params.population_size, params.tests_per_day).into_vec();
                for i in group {
                    if self.contagious.contains(&i) || self.exposed.contains(&i) {
                        self.positive_tested.insert(i);
                    }
                }
            }
        }

        // UPDATE RESTRICTION TIMERS
        match &mut self.active_restriction {
            Restriction::CommunityRestriction(_, _, time)
            | Restriction::LowerCutOffRestriction(_, _, time)
            | Restriction::UpperCutOffRestriction(_, _, time) => {
                *time -= 1;
                if *time <= 0 {
                    self.active_restriction = Restriction::NoRestriction;
                }
            }
            _ => {}
        }
    }

    /// Get the current sizes of each compartmentation
    pub fn get_sizes(&self) -> String {
        format!(
            "{},{},{},{},{}\n",
            self.susceptible.len(),
            self.exposed.len(),
            self.contagious.len(),
            self.recovered.len(),
            self.deceased.len()
        )
    }
}
