use std::cmp::Ordering;

type Node = Option<Box<Tree>>;

pub struct Tree {
    val: i32,
    left: Node,
    right: Node,
}

impl Tree {
    pub fn new(val: i32) -> Self {
        Tree {
            val,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, val: i32) {
        let target_node = if val < self.val {
            &mut self.left
        } else if val > self.val {
            &mut self.right
        } else {
            return
        };

        match target_node {
            None => {
                let node = Tree::new(val);
                *target_node = Some(Box::new(node));
            }
            Some(node) => {
                node.add(val);
            }
        }
    }

    pub fn search(&self, val: i32) -> bool {
        match &val.cmp(&self.val) {
            Ordering::Equal => true,
            Ordering::Less => match &self.left {
                Some(node) => node.search(val),
                None => false,
            },
            Ordering::Greater => match &self.right {
                Some(node) => node.search(val),
                None => false,
            },
        }
    }

    pub fn minimum(&self) -> i32 {
        match &self.left {
            None => self.val,
            Some(node) => node.minimum(),
        }
    }

    pub fn maximum(&self) -> i32 {
        match &self.right {
            None => self.val,
            Some(node) => node.maximum(),
        }
    }

    pub fn traverse(&self) {

        match &self.left {
            None => (),
            Some(node) => node.traverse(),
        };

        //println!("{}", self.val);

        match &self.right {
            None => (),
            Some(node) => node.traverse(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut tree = Tree::new(2);

        tree.add(1);
        tree.add(2);
        tree.add(4);
        tree.add(3);
        tree.add(5);

        tree.traverse();

        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(6), false);
        assert_eq!(tree.minimum(), 1);
        assert_eq!(tree.maximum(), 5);
    }
}
