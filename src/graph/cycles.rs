use std::collections::{HashMap, HashSet};

type Id = usize;

pub struct Graph {
    pub nodes: HashMap<Id, Vec<Id>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn preprocess_vertex(&self, _node: Id) {}

    fn postprocess_vertex(&self, _node: Id) {}

    fn process_edge(
        &self,
        x: Id,
        y: Id,
        parent: &mut Vec<Option<Id>>,
        backedges: &mut Vec<(Id, Id)>,
    ) {
        if parent[y] != Some(x) {
            backedges.push((x, y));
        }
    }

    pub fn cycles(&self) -> Vec<(Id, Id)> {
        let mut discovered = HashSet::new();
        let mut processed = HashSet::new();
        let mut parent = vec![None; self.nodes.len()];
        let mut backedges = Vec::new();

        self.dfs(
            0,
            &mut discovered,
            &mut processed,
            &mut parent,
            &mut backedges,
        );

        backedges
    }

    pub fn dfs(
        &self,
        v: Id,
        discovered: &mut HashSet<Id>,
        processed: &mut HashSet<Id>,
        parent: &mut Vec<Option<Id>>,
        backedges: &mut Vec<(Id, Id)>,
    ) {
        discovered.insert(v);

        self.preprocess_vertex(v);

        if let Some(neighbors) = self.nodes.get(&v) {
            for &u in neighbors {
                if !discovered.contains(&u) {
                    parent[u] = Some(v);
                    self.process_edge(v, u, parent, backedges);
                    self.dfs(u, discovered, processed, parent, backedges);
                } else if !processed.contains(&u) && parent[v] != Some(u) {
                    self.process_edge(v, u, parent, backedges);
                }
            }
        }

        self.postprocess_vertex(v);

        // Vertex is processed if all of it's neighbors has been processed.
        processed.insert(v);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cycles() {
        let mut g = Graph::new();

        g.nodes.insert(0, vec![7, 1, 6]);
        g.nodes.insert(1, vec![0, 2, 6, 4]);
        g.nodes.insert(2, vec![1, 3, 4]);
        g.nodes.insert(3, vec![2, 4]);
        g.nodes.insert(4, vec![1, 2, 3, 5]);
        g.nodes.insert(5, vec![4]);
        g.nodes.insert(6, vec![0, 1]);
        g.nodes.insert(7, vec![0]);

        let backedges = g.cycles();

        assert_eq!(backedges, vec![(4, 1), (4, 2), (6, 0)]);
    }
}
