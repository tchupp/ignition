use core::Item;
use std::fmt;

mod apply;
mod arena;
mod bit_operations;
mod hash_structure;
mod reduce;
mod restrict;
mod operations;
mod summarize;

#[derive(Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Node {
    Branch(Item, NodeId, NodeId),
    Leaf(bool),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NodeId(usize);

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.fmt_inner(1))
    }
}

impl Node {
    fn fmt_inner(&self, indent: usize) -> String {
        match self {
            Node::Leaf(val) => format!("| {}", val),
            Node::Branch(id, low, high) =>
                format!(
                    "| {:?}:\n{}{}\n{}{}",
                    id,
                    "| ".repeat(indent),
                    Node::from(low).fmt_inner(indent + 1),
                    "| ".repeat(indent),
                    Node::from(high).fmt_inner(indent + 1)
                ),
        }
    }
}

impl Node {
    pub const TRUE_LEAF: Node = Node::Leaf(true);
    pub const FALSE_LEAF: Node = Node::Leaf(false);

    pub fn branch<L, H>(id: &Item, low: L, high: H) -> Node where L: Into<NodeId>, H: Into<NodeId> {
        Node::Branch(id.clone(), low.into(), high.into())
    }

    pub fn positive_branch(id: &Item) -> Node {
        Node::branch(id, Node::FALSE_LEAF, Node::TRUE_LEAF)
    }

    pub fn negative_branch(id: &Item) -> Node {
        Node::branch(id, Node::TRUE_LEAF, Node::FALSE_LEAF)
    }
}

impl<'a> From<Node> for NodeId {
    fn from(node: Node) -> Self {
        arena::add(node)
    }
}

impl<'a> From<&'a Node> for NodeId {
    fn from(node: &Node) -> Self {
        arena::add(node.clone())
    }
}

impl From<NodeId> for Node {
    fn from(node_id: NodeId) -> Self {
        arena::get(node_id)
    }
}

impl<'a> From<&'a NodeId> for Node {
    fn from(node_id: &NodeId) -> Self {
        arena::get(*node_id)
    }
}

impl<'a> From<&'a Node> for Node {
    fn from(node: &Node) -> Self {
        node.clone()
    }
}
