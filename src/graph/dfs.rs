use std::collections::{HashMap, HashSet};

type Id = i32;

pub struct Graph {
    pub nodes: HashMap<Id, Vec<Id>>
}

impl Graph {

    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    pub fn dfs(&self, start: Id, path: &mut Vec<Id>, visited: &mut HashSet<Id>) {

        visited.insert(start);

        path.push(start);

        if let Some(neighbors) = self.nodes.get(&start) {
            for &n in neighbors {
                if !visited.contains(&n) {
                    visited.insert(n);
                    self.dfs(n, path, visited)
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

        //  1 -- 2 -- 3
        //  |         |
        //  4 ------- |

        let mut g = Graph::new();

        g.nodes.insert(1, vec![2, 4]);
        g.nodes.insert(2, vec![1, 3]);
        g.nodes.insert(3, vec![2, 4]);
        g.nodes.insert(4, vec![1, 3]);

        let mut path = Vec::new();
        let mut visited = HashSet::new();
        g.dfs(1, &mut path, &mut visited);

        assert_eq!(path, vec![1,2,3,4]);
    }
}
