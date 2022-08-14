use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Edge(u32, u32);

impl Edge {
    pub fn new(u: u32, v: u32) -> Self {
        Self(u, v)
    }

    fn is_end(&self, v: u32) -> bool {
        self.0 == v || self.1 == v
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0, self.1) == (other.0, other.1) || (self.0, self.1) == (other.1, other.0)
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0 < self.1 {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}

type Tree = HashSet<Edge>;
pub type Graph = HashMap<Edge, u32>;

pub struct MstKruskal {}

impl MstKruskal {
    fn is_vertex_in_tree(t: &Tree, v: u32) -> bool {
        t.iter().any(|e| e.is_end(v))
    }

    fn combine_trees(mut trees: Vec<Tree>, Edge(u, v): &Edge) -> Vec<Tree> {
        let mut res = vec![];
        let mut combined = Tree::from([Edge(*u, *v)]);
        loop {
            if let Some(tree) = trees.pop() {
                if Self::is_vertex_in_tree(&tree, *v) || Self::is_vertex_in_tree(&tree, *u) {
                    combined.extend(tree.into_iter());
                } else {
                    res.push(tree);
                }
            } else {
                res.push(combined);
                return res;
            }
        }
    }

    // The algorithm is described as follows:
    //
    // Start with an empty set of edges.
    // Consider all the edges that do not produce a cycle when being added to the set. Pick the one with smallest weight and add it to the set.
    // Repeat #2 until no further edges can be added without producing a cycle.
    // Obviously, the resulting set of edges produces a tree that contains all the vertexes from G. Let us sketch the proof why indeed it produces one of minimum weight.
    pub fn minimum_spanning_tree(g: &Graph) -> Option<Tree> {
        if g.is_empty() {
            return None;
        }

        let mut trees: Vec<Tree> = vec![];
        loop {
            if let Some((e, ..)) = g
                .iter()
                .filter(|(Edge(v, u), ..)| {
                    trees
                        .iter()
                        .all(|t| !Self::is_vertex_in_tree(t, *v) || !Self::is_vertex_in_tree(t, *u))
                })
                .min_by_key(|(_, weight)| *weight)
            {
                trees = Self::combine_trees(trees, e);
            } else {
                return trees.pop();
            }
        }
    }
}
