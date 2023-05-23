use std::collections::HashMap;

type Id = usize;
type Time = usize;
type Stack = Vec<Id>;

pub enum Edge {
    Tree,
    Back,
    Forward,
    Cross,
    Unknown,
}

pub struct Vertex {
    pub id: Id,
    pub degree: usize,
    pub entry_time: usize,
    pub parent: Option<Id>,
    pub discovered: bool,
    pub processed: bool,
    pub ancestor: Id,
}

impl Vertex {
    pub fn new(id: Id) -> Self {
        Vertex {
            id,
            degree: 0,
            entry_time: 0,
            parent: None,
            discovered: false,
            processed: false,
            ancestor: id,
        }
    }
}

pub struct Graph {
    directed: bool,
    adj: HashMap<Id, Vec<Id>>,
}

impl Graph {
    pub fn new(directed: bool) -> Self {
        Graph {
            directed,
            adj: HashMap::new(),
        }
    }

    pub fn topological_sort(&self) -> Stack {
        let mut time = 0;
        let mut stack = Stack::new();
        let mut ancestor = Vec::new();
        let mut degree = Vec::new();
        let mut entry_time = vec![0; self.adj.len()];
        let mut discovered = vec![false; self.adj.len()];
        let mut processed = vec![false; self.adj.len()];
        let mut parent = vec![None; self.adj.len()];

        for n in self.adj.keys() {
            if !discovered[*n] {
                self.dfs(
                    *n,
                    &mut time,
                    &mut entry_time,
                    &mut degree,
                    &mut stack,
                    &mut discovered,
                    &mut processed,
                    &mut parent,
                    &mut ancestor,
                );
            }
        }

        stack
    }

    pub fn postprocess_vertex(&self, v: Id, stack: &mut Stack) {
        stack.push(v);
    }

    pub fn process_edge(
        &self,
        v: Id,
        u: Id,
        entry_time: &mut Vec<usize>,
        discovered: &Vec<bool>,
        processed: &Vec<bool>,
        parent: &Vec<Option<Id>>,
    ) {
        match edge(v, u, entry_time, discovered, processed, parent) {
            Edge::Back => println!("Back Edge!"),
            _ => return,
        }
    }

    pub fn dfs(
        &self,
        root: Id,
        time: &mut Time,
        entry_time: &mut Vec<usize>,
        degree: &mut Vec<usize>,
        stack: &mut Stack,
        discovered: &mut Vec<bool>,
        processed: &mut Vec<bool>,
        parent: &mut Vec<Option<Id>>,
        ancestor: &mut Vec<Id>,
    ) {
        discovered[root] = true;
        *time += 1;
        entry_time[root] = *time;

        if let Some(neighbors) = self.adj.get(&root) {
            for &u in neighbors {
                if !discovered[u] {
                    parent[u] = Some(root);
                    self.process_edge(root, u, entry_time, discovered, processed, parent);
                    self.dfs(
                        u, time, entry_time, degree, stack, discovered, processed, parent, ancestor,
                    )
                } else if !processed[u] && parent[root] != Some(u) {
                    self.process_edge(root, u, entry_time, discovered, processed, parent);
                } else if self.directed {
                    self.process_edge(root, u, entry_time, discovered, processed, parent);
                }
            }
        }

        self.postprocess_vertex(root, stack);

        *time += 1;
    }
}

fn edge(
    v: Id,
    u: Id,
    entry_time: &Vec<Time>,
    discovered: &Vec<bool>,
    processed: &Vec<bool>,
    parent: &Vec<Option<Id>>,
) -> Edge {
    if parent[u] == Some(v) {
        return Edge::Tree;
    }

    if discovered[u] && !processed[u] {
        return Edge::Back;
    }

    if processed[u] && entry_time[u] > entry_time[v] {
        return Edge::Forward;
    }

    if processed[u] && entry_time[u] < entry_time[v] {
        return Edge::Cross;
    }

    Edge::Unknown
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        println!("\nTOPOLOGICAL SORT:");

        let mut g = Graph::new(true);

        g.adj.insert(0, vec![1, 2]);
        g.adj.insert(1, vec![2, 3]);
        g.adj.insert(2, vec![4, 5]);
        g.adj.insert(3, vec![]);
        g.adj.insert(4, vec![3]);
        g.adj.insert(5, vec![4]);
        g.adj.insert(6, vec![0, 5]);

        let sorted = g.topological_sort();

        // (G, A, B, C, F, E, D)
        // (6, 0, 1, 2, 5, 4, 3)
        assert_eq!(sorted, vec![3, 4, 5, 2, 1, 0, 5]);
    }
}
