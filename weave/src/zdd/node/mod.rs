pub use self::arena::*;
use std::sync::RwLock;

mod arena;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NodeId(usize);

impl NodeId {
    pub fn is_empty(self) -> bool {
        self.0 == Node::FALSE.0
    }

    pub fn is_unit(self) -> bool {
        self.0 == Node::TRUE.0
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Priority(pub usize);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Node {
    Branch(Priority, NodeId, NodeId),
    Leaf(bool),
}

impl Node {
    pub const FALSE: NodeId = NodeId(0);
    pub const TRUE: NodeId = NodeId(1);

    pub fn branch<L, H>(priority: usize, low: L, high: H) -> Node where L: Into<NodeId>, H: Into<NodeId> {
        Node::Branch(Priority(priority), low.into(), high.into())
    }

    pub fn required_branch<H>(priority: usize, high: H) -> Node where H: Into<NodeId> {
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
