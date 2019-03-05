use hashbrown::hash_map::Entry::*;
use hashbrown::HashMap;

use super::Node;
use super::NodeId;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodeArena {
    nodes: Vec<Node>,
    node_index: HashMap<Node, NodeId>,
}

impl Default for NodeArena {
    fn default() -> Self {
        let mut arena = NodeArena { nodes: Vec::new(), node_index: HashMap::new() };
        arena.add(Node::Never);
        arena.add(Node::Always);
        arena
    }
}

impl NodeArena {
    pub fn new() -> Self {
        NodeArena::default()
    }

    pub fn add(&mut self, node: Node) -> NodeId {
        match self.node_index.entry(node) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                let index = NodeId(self.nodes.len());

                self.nodes.push(node);
                *entry.insert(index)
            }
        }
    }

    pub fn get_by_id(&self, index: NodeId) -> Option<&Node> {
        self.nodes.get(index.0)
    }

    pub fn get_by_node(&self, node: &Node) -> Option<&NodeId> {
        self.node_index.get(node)
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use super::NodeArena;
    use super::super::Priority;

    #[test]
    fn get_has_always_node_saved() {
        let arena = NodeArena::new();

        let always_node = arena.get_by_id(Node::ALWAYS).expect("Expected node to exist");

        assert_eq!(
            Node::Always,
            *always_node
        );
    }

    #[test]
    fn get_has_never_node_saved() {
        let arena = NodeArena::new();

        let never_node = arena.get_by_id(Node::NEVER).expect("Expected node to exist");

        assert_eq!(
            Node::Never,
            *never_node
        );
    }

    #[test]
    fn add_returns_unique_node_id_for_different_nodes() {
        let node1 = Node::branch(Priority(1), Node::NEVER, Node::ALWAYS);
        let node2 = Node::branch(Priority(2), Node::ALWAYS, Node::NEVER);

        let mut arena = NodeArena::new();

        let node1_id = arena.add(node1);
        let node2_id = arena.add(node2);

        assert_eq!(
            &node1,
            arena.get_by_id(node1_id).expect("Expected node to exist")
        );
        assert_eq!(
            &node2,
            arena.get_by_id(node2_id).expect("Expected node to exist")
        );
    }

    #[test]
    fn add_returns_same_node_id_for_same_nodes() {
        let node1 = Node::branch(Priority(1), Node::NEVER, Node::ALWAYS);

        let mut arena = NodeArena::new();

        let node1_id = arena.add(node1.clone());
        let node2_id = arena.add(node1.clone());

        assert_eq!(node1_id, node2_id);
    }
}