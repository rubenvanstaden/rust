use std::collections::{HashMap, HashSet};

type Id = i32;

pub struct Graph {
    nodes: HashMap<Id, Vec<Id>>,
}

impl Graph {

    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    fn process_edge(&self, x: Id, y: Id, parent: &mut Vec<Option<Id>>) {
        if parent[y as usize] != Some(x) {
            println!("Cycle from {} to {}", y, x);
        }
    }

    pub fn dfs(&self, v: Id, visited: &mut HashSet<Id>, parent: &mut Vec<Option<Id>>) {

        visited.insert(v);

        if let Some(neighbors) = self.nodes.get(&v) {
            for &u in neighbors {
                if !visited.contains(&u) {
                    parent[u as usize] = Some(v);
                    self.process_edge(v, u, parent);
                    self.dfs(u, visited, parent);
                } else if parent[v as usize] != Some(u) {
                    self.process_edge(v, u, parent);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        let mut g = Graph::new();

        g.nodes.insert(0, vec![1, 2]);
        g.nodes.insert(1, vec![0, 2]);
        g.nodes.insert(2, vec![0, 1, 3, 4]);
        g.nodes.insert(3, vec![2, 4]);
        g.nodes.insert(4, vec![3, 2]);

        let mut visited = HashSet::new();
        let mut parent = vec![None; g.nodes.len()];
        g.dfs(0, &mut visited, &mut parent);

        //assert_eq!(g.);
    }
}
