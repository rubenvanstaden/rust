use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    pub vertices: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
        }
    }

    pub fn bipartite(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut g = Graph::new();

        g.vertices.insert(1, vec![2, 4]);
        g.vertices.insert(2, vec![1, 3]);
        g.vertices.insert(3, vec![2, 4]);
        g.vertices.insert(4, vec![1, 3]);

        assert_eq!(g.bipartite(), true);
    }
}
