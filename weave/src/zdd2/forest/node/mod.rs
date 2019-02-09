use std::fmt;
use std::sync::RwLock;

use super::root::Priority;

use self::arena::*;

mod arena;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NodeId(usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", Node::from(self))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Node {
    Branch(Priority, NodeId, NodeId),
    Leaf(bool),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.fmt_inner(1))
    }
}

impl Node {
    fn fmt_inner(&self, indent: usize) -> String {
        match self {
            Node::Leaf(val) => format!("{}", val),
            Node::Branch(id, low, high) =>
                format!(
                    "{:?}:\n{}{}\n{}{}",
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
    pub const FALSE: NodeId = NodeId(0);
    pub const TRUE: NodeId = NodeId(1);

    pub fn branch<L, H>(id: Priority, low: L, high: H) -> Self where L: Into<NodeId>, H: Into<NodeId> {
        let low = Node::from(low.into());
        let high = Node::from(high.into());

        match (high, low) {
            (Node::Branch(h_id, h_low, h_high), _) if h_id < id => {
                return Node::Branch(h_id, h_low, Node::branch(id, low, h_high).into());
            }
            (_, Node::Branch(l_id, l_low, l_high)) if l_id < id => {
                return Node::Branch(l_id, Node::branch(id, l_low, high).into(), l_high);
            }
            _ => {}
        }

        Node::Branch(id, low.into(), high.into())
    }
}

impl<'a> From<Node> for NodeId {
    fn from(node: Node) -> Self {
        ARENA.write().unwrap()
            .add(node)
    }
}

impl<'a> From<&'a Node> for NodeId {
    fn from(node: &Node) -> Self {
        NodeId::from(*node)
    }
}

impl From<NodeId> for Node {
    fn from(node_id: NodeId) -> Self {
        *ARENA.read().unwrap()
            .get(node_id)
            .unwrap_or_else(|| panic!("Expected node to exist for: {:?}", node_id))
    }
}

impl<'a> From<&'a NodeId> for Node {
    fn from(node_id: &NodeId) -> Self {
        Node::from(*node_id)
    }
}

impl<'a> From<&'a Node> for Node {
    fn from(node: &Node) -> Self {
        *node
    }
}

lazy_static! {
    pub static ref ARENA: RwLock<NodeArena> = {
        let a = NodeArena::new();
        RwLock::new(a)
    };
}
