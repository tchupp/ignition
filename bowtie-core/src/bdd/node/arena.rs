use bdd::node::Node;
use bdd::node::NodeId;
use std::collections::hash_map::Entry::*;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref ARENA: Mutex<Arena> = {
        let mut a = Arena::new();
        a.add(Node::TRUE_LEAF);
        a.add(Node::FALSE_LEAF);
        Mutex::new(a)
    };
}

#[derive(Default)]
pub struct Arena {
    nodes: Vec<Node>,
    node_index: HashMap<Node, NodeId>,
}

impl Arena {
    fn new() -> Arena {
        Arena::default()
    }

    fn add(&mut self, node: Node) -> NodeId {
        let index = match self.node_index.entry(node.clone()) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                let index = NodeId(self.nodes.len());

                self.nodes.push(node);
                *entry.insert(index)
            }
        };

        return index;
    }

    fn get(&self, index: NodeId) -> Option<&Node> {
        self.nodes.get(index.0)
    }

    fn must_get(&self, index: NodeId) -> &Node {
        self.nodes.get(index.0).expect("Expected node to exist")
    }

    fn count(&self) -> usize {
        self.nodes.len()
    }
}

pub fn add(node: Node) -> NodeId {
    let mut arena = ARENA.lock().unwrap();
    arena.add(node)
}

pub fn get(index: NodeId) -> Node {
    let arena = ARENA.lock().unwrap();
    arena.must_get(index).clone()
}

pub fn count() -> usize {
    let arena = ARENA.lock().unwrap();
    arena.count()
}

#[cfg(test)]
mod tests {
    use bdd::node::arena::Arena;
    use bdd::node::Node;

    #[test]
    fn add() {
        let node1 = Node::TRUE_LEAF;
        let node2 = Node::FALSE_LEAF;

        let mut arena = Arena::new();

        let node1_id = arena.add(node1.clone());
        let node2_id = arena.add(node2.clone());

        assert_eq!(&node1, arena.must_get(node1_id));
        assert_eq!(&node2, arena.must_get(node2_id));
    }

    #[test]
    fn add_does_not_duplicate() {
        let node1 = Node::TRUE_LEAF;

        let mut arena = Arena::new();

        let node1_id = arena.add(node1.clone());
        let node2_id = arena.add(node1.clone());

        assert_eq!(node1_id, node2_id);
    }
}