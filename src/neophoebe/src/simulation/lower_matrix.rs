use std::ops::{Index, IndexMut};

/// A struct to handle a symmetric NxN matrix
pub struct SymmetricMatrix<T> {
    matrix: Vec<Vec<T>>
}

impl<T: Clone + Copy> SymmetricMatrix<T> {
    /// Given a size and a default value, return a NxN `SymmetricMatrix` with all entries set to that default value
    pub fn new(size: usize, default_value: T) -> Self {
        let mut matrix = Vec::with_capacity(size);

        for i in 1..=size {
            matrix.push(vec![default_value; i]);
        }

        Self {
            matrix
        }
    }
}

// SET UP INDEXING FOR SYMMETRIC MATRIX

impl<T> Index<(usize, usize)> for SymmetricMatrix<T> {
    type Output = T;

    /// [p1,p2]=[p2,p1]
    fn index(&self, (p, q): (usize, usize)) -> &Self::Output {
        if p <= q {
            return &self.matrix[q][p];
        }

        &self.matrix[p][q]

    }
}

impl<T> IndexMut<(usize, usize)> for SymmetricMatrix<T> {
    /// [p1,p2]=[p2,p1]
    fn index_mut(&mut self, (p, q): (usize, usize)) -> &mut Self::Output {
        if p <= q {
            return &mut self.matrix[q][p];
        }

        &mut self.matrix[p][q]
    }
}