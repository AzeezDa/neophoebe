use std::ops::{Index, IndexMut};

pub struct LowerMatrix<T> {
    matrix: Vec<Vec<T>>
}

impl<T: Clone + Copy> LowerMatrix<T> {
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

impl<T> Index<(usize, usize)> for LowerMatrix<T> {
    type Output = T;

    fn index(&self, (p, q): (usize, usize)) -> &Self::Output {
        if p <= q {
            return &self.matrix[q][p];
        }

        &self.matrix[p][q]

    }
}

impl<T> IndexMut<(usize, usize)> for LowerMatrix<T> {
    fn index_mut(&mut self, (p, q): (usize, usize)) -> &mut Self::Output {
        if p <= q {
            return &mut self.matrix[q][p];
        }

        &mut self.matrix[p][q]
    }
}