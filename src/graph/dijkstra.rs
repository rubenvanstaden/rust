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

    pub fn dijkstra(&self, start: Id) -> (Vec<i32>, i32) {
        let mut total_weight = 0;
        let mut dist = i32::MAX;

        let mut intree = vec![false; self.nodes.len()];
        let mut distance = vec![i32::MAX; self.nodes.len()];

        distance[start] = 0;

        // The node v is the current node.
        let mut v = start;

        while !intree[v] {

            intree[v] = true;

            if v != start {
                total_weight = total_weight + dist;
            }

            // 1. Update the distance of each neighbouring node.

            if let Some(neighbors) = self.nodes.get(&v) {
                for &(u, weight) in neighbors {
                    if distance[u] > distance[v] + weight {
                        distance[u] = distance[v] + weight;
                    }
                }
            }

            // 2. Now, select the neighbor with the shortest path from "start".

            dist = i32::MAX;
            for (&u, _edges) in &self.nodes {
                if !intree[u] && dist > distance[u] {
                    dist = distance[u];
                    v = u;
                }
            }
        }

        (distance, total_weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {

        let mut g = Graph {
            nodes: HashMap::new(),
        };

        g.nodes.insert(0, vec![(1, 1), (3, 3)]);
        g.nodes.insert(1, vec![(0, 1), (2, 1)]);
        g.nodes.insert(2, vec![(1, 1), (3, 2)]);
        g.nodes.insert(3, vec![(0, 3), (2, 2)]);

        let (distances, weight) = g.dijkstra(0);

        assert_eq!(weight, 6);
        assert_eq!(distances[0], 0);
        assert_eq!(distances[1], 1);
        assert_eq!(distances[2], 2);
        assert_eq!(distances[3], 3);
    }
}
