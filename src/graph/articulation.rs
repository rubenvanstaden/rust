use std::collections::HashMap;

type Id = usize;

pub struct Graph {
    nodes: HashMap<Id, Vec<Id>>,
}

pub enum Edge {
    Tree,
    Back,
    Unknown,
}

pub fn edge_type(
    v: Id,
    u: Id,
    parent: &Vec<Option<Id>>,
    discovered: &Vec<bool>,
    processed: &Vec<bool>,
) -> Edge {
    if parent[u] == Some(v) {
        return Edge::Tree;
    }

    if discovered[u] && !processed[u] {
        return Edge::Back;
    }

    Edge::Unknown
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn preprocess_vertex(&self, v: Id, ancestor: &mut Vec<Id>) {
        ancestor[v] = v;
    }

    pub fn postprocess_vertex(
        &self,
        v: Id,
        parent: &Vec<Option<Id>>,
        entry_time: &mut Vec<usize>,
        ancestor: &mut Vec<Id>,
        degree: &mut Vec<usize>,
    ) {
        match parent[v] {
            None => {
                //println!("Root cut node: {}", v);
            }
            Some(n) => {
                if ancestor[v] == n {
                    //println!("Parent cut node: {}", n);
                }

                if ancestor[v] == v {
                    //println!("Bridge cut node: {}", n);

                    // If v is not a leaf node.
                    if degree[v] > 0 {
                        //println!("Bridge cut node: {}", v);
                    }
                }

                let time_v = entry_time[ancestor[v]];
                let time_parent = entry_time[ancestor[n]];
                if time_v < time_parent {
                    ancestor[n] = ancestor[v];
                }
            }
        }
    }

    pub fn process_edge(
        &self,
        v: Id,
        u: Id,
        degree: &mut Vec<usize>,
        entry_time: &mut Vec<usize>,
        parent: &Vec<Option<Id>>,
        discovered: &Vec<bool>,
        processed: &Vec<bool>,
        ancestor: &mut Vec<Id>,
    ) {
        //println!("({}, {})", v, u);

        match edge_type(v, u, parent, discovered, processed) {
            Edge::Tree => {
                degree[v] += 1;
            }
            Edge::Back => {
                if parent[v] != Some(u) {
                    if entry_time[u] < entry_time[ancestor[v]] {
                        ancestor[v] = u;
                    }
                }
            }
            Edge::Unknown => {
                //println!("[*] FAILED!");
            }
        }
    }

    pub fn articulation_nodes(&self) -> Vec<Id> {
        let mut time = 0;
        let mut degree = vec![0; self.nodes.len()];
        let mut entry_time = vec![0; self.nodes.len()];
        let mut discovered = vec![false; self.nodes.len()];
        let mut processed = vec![false; self.nodes.len()];
        let mut ancestor = vec![0; self.nodes.len()];
        let mut parent = vec![None; self.nodes.len()];
        let mut cutnodes = Vec::new();

        self.dfs(
            0,
            &mut time,
            &mut degree,
            &mut entry_time,
            &mut discovered,
            &mut processed,
            &mut cutnodes,
            &mut ancestor,
            &mut parent,
        );

        cutnodes
    }

    pub fn dfs(
        &self,
        root: Id,
        time: &mut usize,
        degree: &mut Vec<usize>,
        entry_time: &mut Vec<usize>,
        discovered: &mut Vec<bool>,
        processed: &mut Vec<bool>,
        cutnodes: &mut Vec<Id>,
        ancestor: &mut Vec<Id>,
        parent: &mut Vec<Option<Id>>,
    ) {
        *time += 1;

        entry_time[root] = *time;

        discovered[root] = true;

        self.preprocess_vertex(root, ancestor);

        if let Some(neighbors) = self.nodes.get(&root) {
            for &u in neighbors {
                if !discovered[u] {
                    parent[u] = Some(root);
                    self.process_edge(
                        root, u, degree, entry_time, parent, discovered, processed, ancestor,
                    );
                    self.dfs(
                        u, time, degree, entry_time, discovered, processed, cutnodes, ancestor,
                        parent,
                    );
                } else if !processed[u] && parent[root] != Some(u) {
                    self.process_edge(
                        root, u, degree, entry_time, parent, discovered, processed, ancestor,
                    );
                }
            }
        }

        self.postprocess_vertex(root, parent, entry_time, ancestor, degree);
        *time += 1;

        processed[root] = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut g = Graph::new();

        g.nodes.insert(0, vec![7, 1, 6]);
        g.nodes.insert(1, vec![0, 2, 6, 4]);
        g.nodes.insert(2, vec![1, 3, 4]);
        g.nodes.insert(3, vec![2, 4]);
        g.nodes.insert(4, vec![1, 2, 3, 5]);
        g.nodes.insert(5, vec![4]);
        g.nodes.insert(6, vec![0, 1]);
        g.nodes.insert(7, vec![0]);

        let _cutnodes = g.articulation_nodes();

        //println!("{:?}", cutnodes);
    }
}
