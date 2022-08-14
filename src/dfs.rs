use std::collections::HashSet;

pub struct Dfs {
    visited: HashSet<usize>,
}

impl Dfs {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
        }
    }

    fn dfs_adjacent_matrix_internal(&mut self, matrix: &[&[i32]], start: usize) {
        self.visited.insert(start);

        for j in 0..matrix[start].len() {
            if matrix[start][j] > 0 {
                if !self.visited.contains(&j) {
                    self.dfs_adjacent_matrix_internal(matrix, j);
                }
            }
        }
    }

    pub fn dfs_adjacent_matrix(&mut self, matrix: &[&[i32]], start: usize) -> i32 {
        self.reset();

        let mut components = 0;
        for i in start..matrix.len() {
            if !self.visited.contains(&i) {
                components += 1;

                self.dfs_adjacent_matrix_internal(matrix, i);
            }
        }

        components
    }

    pub fn get_visited(&self) -> &HashSet<usize> {
        &self.visited
    }

    pub fn reset(&mut self) {
        self.visited.clear();
    }
}
