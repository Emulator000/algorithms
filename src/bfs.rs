use std::collections::{HashSet, VecDeque};

pub struct Bfs {
    visited: HashSet<usize>,
}

impl Bfs {
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
        }
    }

    fn bfs_adjacent_matrix_internal(&mut self, matrix: &[&[i32]], start: usize) {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut vis;
        while !queue.is_empty() {
            if let Some(front) = queue.front() {
                vis = *front;
            } else {
                break;
            }

            queue.pop_front();

            for j in 0..matrix[vis].len() {
                if matrix[vis][j] > 0 {
                    if !self.visited.contains(&j) {
                        queue.push_back(j);

                        self.visited.insert(j);
                    }
                }
            }
        }
    }

    pub fn bfs_adjacent_matrix(&mut self, matrix: &[&[i32]], start: usize) -> i32 {
        self.reset();

        let mut components = 0;
        for i in start..matrix.len() {
            if !self.visited.contains(&i) {
                components += 1;

                self.bfs_adjacent_matrix_internal(matrix, i);
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
