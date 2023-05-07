use std::collections::{HashMap, HashSet, VecDeque};

type Id = i32;

struct Vertex {
    id: Id,
    neighbors: Vec<Id>,
}

impl Vertex {
    fn new(id: Id) -> Self {
        Vertex {
            id,
            neighbors: Vec::new(),
        }
    }

    fn add_neighbor(&mut self, v: Id) {
        self.neighbors.push(v);
    }

    fn preprocess(&self) {
        println!("preprocess: {}", self.id);
    }

    fn postprocess(&self) {
        println!("postprocess: {}", self.id);
    }
}

struct Graph {
    visited: HashSet<Id>,
    processed: HashSet<Id>,
    vertices: HashMap<Id, Vertex>,
}

impl Graph {

    fn new() -> Self {
        Graph {
            visited: HashSet::new(),
            processed: HashSet::new(),
            vertices: HashMap::new(),
        }
    }

    fn print(&self) {
        println!("Num Vertices: {}", self.vertices.len());
        for (id, node) in &self.vertices {
            println!("{}: {}", id, node.neighbors.len());
        }
    }

    fn add_edge(&mut self, x: Id, y: Id) {
        self.vertices
            .entry(x)
            .or_insert(Vertex::new(x))
            .add_neighbor(y);
        self.vertices
            .entry(y)
            .or_insert(Vertex::new(y))
            .add_neighbor(x);
    }

    fn process_edge(&self, x: Id, y: Id, path: &mut Vec<Id>) {
        path.push(y);
        println!("process edge: ({}, {})", x, y);
    }

    fn components(&mut self) -> Vec<HashSet<Id>> {
        let mut comps: Vec<HashSet<Id>> = vec![];
        for v in 0..self.vertices.len() {
            let id = v as Id;
            if !self.visited.contains(&id) {
                println!("{}", id);
                let mut component = HashSet::new();
                self.bfs(id, &mut component);
                comps.push(component);
            }
        }
        comps
    }

    fn bfs(&mut self, start: Id, component: &mut HashSet<Id>) -> Option<Vec<Id>> {
        let mut q = VecDeque::new();
        q.push_back(start);

        self.visited.insert(start);
        component.insert(start);

        let mut path = vec![start];

        while let Some(curr) = q.pop_front() {

            let node = match self.vertices.get(&curr) {
                None => continue,
                Some(v) => v,
            };

            node.preprocess();

            self.processed.insert(curr);

            for &n in &node.neighbors {

                if !self.visited.contains(&n) {
                    q.push_back(n);
                    self.visited.insert(n);
                    component.insert(n);
                    self.process_edge(curr, n, &mut path);
                }
            }

            node.postprocess();
        }

        Some(path)
    }
}

fn main() {

    let mut g = Graph::new();

    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(4, 5);

    g.print();

    println!("");
    println!("Components: {:?}", g.components());

    let mut c = HashSet::new();
    let path = g.bfs(0, &mut c);

    println!("Path: {:?}", path);
}
