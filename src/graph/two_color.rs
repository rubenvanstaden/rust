use std::collections::{HashMap, VecDeque};

type Id = usize;

#[derive(Clone, PartialEq)]
pub enum Color {
    Unknown,
    White,
    Black,
}

pub struct Graph {
    pub nodes: HashMap<Id, Vec<Id>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn bipartite(&self) -> bool {

        let mut visited = vec![false; self.nodes.len()];
        let mut processed = vec![false; self.nodes.len()];
        let mut color = vec![Color::Unknown; self.nodes.len()];

        let mut bipartite = true;

        for &v in self.nodes.keys() {
            if !visited[v] {
                color[v] = Color::White;
                self.bfs(v, &mut bipartite, &mut visited, &mut processed, &mut color);
            }
        }

        bipartite
    }

    pub fn bfs(
        &self,
        start: Id,
        bipartite: &mut bool,
        visited: &mut Vec<bool>,
        processed: &mut Vec<bool>,
        color: &mut Vec<Color>,
    ) {

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {

            self.process_vertex_early(v);

            if let Some(neighbors) = self.nodes.get(&v) {

                for &u in neighbors {

                    self.process_edge(v, u, color, bipartite);

                    if !visited[u] {
                        visited[u] = true;
                        queue.push_back(u);
                    }
                }
            }

            self.process_vertex_late(v);
            processed[v] = true;

            println!("");
        }
    }

    pub fn process_vertex_early(&self, v: Id) {
        println!("Processing vertex early ({})", v);
    }

    pub fn process_vertex_late(&self, v: Id) {
        println!("Processing vertex late ({})", v);
    }

    pub fn process_edge(&self, v: Id, u: Id, color: &mut Vec<Color>, bipartite: &mut bool) {
        println!("Processing edge ({}, {})", v, u);

        match color[u] {
            Color::Unknown => {
                match color[v] {
                    Color::Unknown => println!("ERROR"),
                    Color::White => color[u] = Color::Black,
                    Color::Black => color[u] = Color::White,
                }
            },
            _ => {
                if color[u] == color[v] {
                    *bipartite = false;
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
        g.nodes.insert(0, vec![1]);
        g.nodes.insert(1, vec![0, 2]);
        g.nodes.insert(2, vec![1, 3]);
        g.nodes.insert(3, vec![2]);
        assert!(g.bipartite());

        let mut g = Graph::new();
        g.nodes.insert(0, vec![1, 2]);
        g.nodes.insert(1, vec![0, 2]);
        g.nodes.insert(2, vec![0, 1]);
        assert!(!g.bipartite());

    }
}
