use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    vertices: Vec<usize>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new(vertices: Vec<usize>, edges: HashMap<usize, Vec<usize>>) -> Self {
        Graph { vertices, edges }
    }

    fn is_two_colorable(&self) -> bool {
        let mut color_map: HashMap<usize, bool> = HashMap::new();
        for vertex in self.vertices.iter() {
            color_map.insert(*vertex, false);
        }
        for (vertex, neighbors) in self.edges.iter() {
            for neighbor in neighbors.iter() {
                if color_map[&vertex] == color_map[&neighbor] {
                    return false;
                }
            }
            color_map.insert(*vertex, !color_map[&vertex]);
        }
        true
    }
}

fn main() {
    let vertices = vec![0, 1, 2, 3];
    let edges: HashMap<usize, Vec<usize>> = [(0, vec![1, 3]), (1, vec![0, 2]), (2, vec![1, 3]), (3, vec![0, 2])]
        .iter().cloned().collect();
    let graph = Graph::new(vertices, edges);

    println!("{:?}", graph.is_two_colorable()); // true
}

