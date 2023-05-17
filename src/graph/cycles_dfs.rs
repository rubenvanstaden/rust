use std::collections::{HashMap, HashSet};

type Id = i32;

pub struct Graph {
    pub nodes: HashMap<Id, Vec<Id>>,
}

impl Graph {
    
    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    pub fn dfs(&self, start: Id, visited: &mut HashSet<Id>) {

        visited.insert(start);
        
        if let Some(neighbors) = self.nodes.get(&start) {

            for &n in neighbors {

                if !visited.contains(&n) {
                    visited.insert(n);
                    self.dfs(n, visited);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cycles() {

        let mut g = Graph::new();

        g.nodes.insert(1, vec![2, 3]);
        g.nodes.insert(2, vec![1, 4]);
        g.nodes.insert(3, vec![1, 4]);
        g.nodes.insert(4, vec![2, 3]);

        let mut visited = HashSet::new();
        g.dfs(1, &mut visited);
    }
}
