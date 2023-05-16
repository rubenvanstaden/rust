use std::collections::HashMap;

type Id = usize;
type Stack = Vec<Id>;
type Nodes = HashMap<Id, Vertex>;

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
    pub adj: HashMap<Id, Vec<Id>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj: HashMap::new(),
        }
    }

    pub fn topological_sort(&self, nodes: &mut Nodes) -> Stack {

        let mut stack = Stack::new();

        for (id, _neighbors) in &self.adj {
            if !nodes[&id].discovered {
                self.dfs(*id, nodes, &mut stack);
            }
        }

        stack
    }

    pub fn process_edge(&self, v: Id, u: Id, nodes: &mut Nodes) {

        let x = match nodes.get(&v) {
            None => return,
            Some(n) => n,
        };

        let y = match nodes.get(&u) {
            None => return,
            Some(n) => n,
        };

        match edge(x, y) {
            Edge::Back => println!("Back end!"),
            _ => return,
        }
    }

    pub fn dfs(&self, root: Id, nodes: &mut Nodes, stack: &mut Stack) {

        let v = match nodes.get_mut(&root) {
            None => return,
            Some(n) => n,
        };

        v.discovered = true;

        let parent = v.parent;

        if let Some(neighbors) = self.adj.get(&v.id) {
            for &id in neighbors {
                if let Some(u) = nodes.get_mut(&id) {
                    if !u.discovered {
                        u.parent = Some(root);
                        self.process_edge(v.id, u.id, nodes);
                        self.dfs(u.id, nodes, stack);
                    } else if !u.processed && parent != Some(u.id) {
                        
                    }
                }
            }
        }
    }
}

fn edge(v: &Vertex, u: &Vertex) -> Edge {

    if u.parent == Some(v.id) {
        return Edge::Tree
    }

    if u.discovered && !u.processed {
        return Edge::Back;
    }

    Edge::Unknown
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        let mut nodes = Nodes::new();

        nodes.insert(0, Vertex::new(0));
        nodes.insert(1, Vertex::new(1));
        nodes.insert(2, Vertex::new(2));
        nodes.insert(3, Vertex::new(3));
        nodes.insert(4, Vertex::new(4));
        nodes.insert(5, Vertex::new(5));
        nodes.insert(6, Vertex::new(6));

        let mut g = Graph::new();

        g.adj.insert(0, vec![1, 2]);
        g.adj.insert(1, vec![2, 3]);
        g.adj.insert(2, vec![4, 5]);
        g.adj.insert(3, vec![]);
        g.adj.insert(4, vec![3]);
        g.adj.insert(5, vec![4]);
        g.adj.insert(6, vec![0, 5]);

        let sorted = g.topological_sort(&mut nodes);

        println!("{:?}", sorted);
    }
}
