pub use self::arena::*;
use std::fmt;
use std::sync::RwLock;

mod arena;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NodeId(usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", Node::from(self))
    }
}

pub type Priority = usize;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

    pub fn branch<L, H>(priority: Priority, low: L, high: H) -> Node where L: Into<NodeId>, H: Into<NodeId> {
        Node::Branch(priority, low.into(), high.into())
    }

    pub fn required_branch<H>(priority: Priority, high: H) -> Node where H: Into<NodeId> {
        Node::branch(priority, Node::FALSE, high.into())
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
