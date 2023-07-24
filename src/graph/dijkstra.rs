use std::collections::HashMap;

type Id = usize;
type Edge = (Id, i32); // (node, weight)

pub struct Graph {
    nodes: HashMap<Id, Vec<Edge>>,
}

impl Graph {

    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn dijkstra(&self, start: usize) -> (i32, Vec<i32>) {

        // Create a visited tree.
        let mut visited = vec![false; self.nodes.len()];

        // Keep track of the minimal distance each node has to the start.
        let mut distance = vec![i32::MAX; self.nodes.len()];

        distance[start] = 0;

        let mut v = start;

        while !visited[v] {

            println!("curr: {}", v);

            // Add node to the visited tree.
            visited[v] = true;

            // Update the distance to each neighbor.
            if let Some(neighbors) = self.nodes.get(&v) {
                for (u, weight) in neighbors {
                    println!("(u: {}, w: {})", u, weight);
                    if distance[*u] > distance[v] + weight {
                        distance[*u] = distance[v] + weight;
                    }
                }
            }

            // Extract the shortest path to the neighbors.
            let mut dist = i32::MAX;
            for nodes in self.nodes.values() {
                for (u, _) in nodes {
                    if !visited[*u] && dist > distance[*u] {

                        // Update inmem var to calculate the MIN.
                        dist = distance[*u];

                        // Update the current node to the shortest path found.
                        v = *u;
                    }
                }
            }
        }

        (0, distance)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        let mut g = Graph::new();

        g.nodes.insert(0, vec![(1, 1), (3, 3)]);
        g.nodes.insert(1, vec![(0, 1), (2, 1)]);
        g.nodes.insert(2, vec![(1, 1), (3, 2)]);
        g.nodes.insert(3, vec![(0, 3), (2, 2)]);

        let (weight, distance) = g.dijkstra(0);

        assert_eq!(weight, 0);
        assert_eq!(distance[0], 0);
        assert_eq!(distance[1], 1);
        assert_eq!(distance[2], 2);
    }
}
