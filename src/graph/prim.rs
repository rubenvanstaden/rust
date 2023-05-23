use std::collections::HashMap;

type Id = usize;
type Edge = (Id, i32); // (node, weight)

pub struct Graph {
    pub nodes: HashMap<Id, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn prim(&self, start: Id) -> i32 {
        let mut total_weight = 0;
        let mut dist = i32::MAX;

        let mut parent: Vec<Option<Id>> = vec![None; self.nodes.len()];
        let mut intree = vec![false; self.nodes.len()];
        let mut distance = vec![i32::MAX; self.nodes.len()];

        distance[start] = 0;
        let mut v = start;

        while !intree[v] {
            intree[v] = true;
            if v != start {
                if let Some(p) = parent[v] {
                    println!("edge ({}, {}) in tree", p, v);
                }
                total_weight += dist;
            }

            if let Some(neighbors) = self.nodes.get(&v) {
                for &edge in neighbors {
                    let u = edge.0;
                    let weight = edge.1;

                    // Update the distance is the distance to the neighbor
                    // is less than the stored distance. And the neighbor
                    // has not been visited before.
                    if distance[u] > weight && !intree[u] {
                        distance[u] = weight;
                        parent[u] = Some(v);
                    }
                }
            }

            // Find the next vertex that has a minimum weight connection.
            dist = i32::MAX;
            for (&node, _edges) in &self.nodes {
                if !intree[node] && dist > distance[node] {
                    dist = distance[node];
                    v = node;
                }
            }
        }

        total_weight
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prim_1() {
        let mut g = Graph::new();

        g.nodes.insert(0, vec![(1, 2), (2, 3)]);
        g.nodes.insert(1, vec![(0, 2), (2, 1)]);
        g.nodes.insert(2, vec![(0, 3), (1, 1)]);

        assert_eq!(g.prim(0), 3);
    }

    #[test]
    fn test_prim_2() {
        let mut g = Graph::new();

        g.nodes.insert(0, vec![(1, 2), (2, 3), (3, 7)]);
        g.nodes.insert(1, vec![(0, 2), (2, 1), (4, 5)]);
        g.nodes.insert(2, vec![(0, 3), (1, 1), (3, 4), (5, 6)]);
        g.nodes.insert(3, vec![(0, 7), (2, 4), (5, 2)]);
        g.nodes.insert(4, vec![(1, 5), (5, 3), (6, 9)]);
        g.nodes.insert(5, vec![(2, 6), (3, 2), (4, 3), (6, 1)]);
        g.nodes.insert(6, vec![(4, 9), (5, 1)]);

        assert_eq!(g.prim(0), 13);
    }
}
