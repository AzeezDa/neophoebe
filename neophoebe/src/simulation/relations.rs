use itertools::Itertools;

use super::{LowerMatrix, Parameters};

pub struct Relations {
    matrix: LowerMatrix<f64>,
}

impl Relations {
    pub fn new(params: &Parameters) -> Self {
        let mut matrix = LowerMatrix::new(params.population_size, 0.);

        // Household
        let mut i = 0;
        while i < params.population_size {
            let j = (i + params.household_size).min(params.population_size);
            for (p, q) in (i..j).tuple_combinations() {
                matrix[(p, q)] = params.household_relation;
            }
            i += params.household_size;
        }

        // Extra relations
        let mut rng = rand::thread_rng();
        for &(num_applications, relation_size, relation_strength) in params.extra_relations.iter() {
            for _ in 0..num_applications {
                let group =
                    rand::seq::index::sample(&mut rng, params.population_size, relation_size)
                        .into_vec();
                for (&p, &q) in group.iter().tuple_combinations() {
                    let v = matrix[(p, q)];
                    matrix[(p, q)] = (v + relation_strength).clamp(0., 1.);
                }
            }
        }

        Self { matrix }
    }

    #[inline]
    pub fn get(&self, p1: usize, p2: usize) -> f64 {
        self.matrix[(p1, p2)]
    }
}
