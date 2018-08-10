use core::Item;
use std::fmt;

mod apply;
mod bit_operations;
mod from_item;
mod reduce;
mod restrict;
mod operations;

#[derive(Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum Node {
    Branch(Item, Box<Node>, Box<Node>),
    Leaf(bool),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.fmt_inner(1))
    }
}

impl Node {
    fn fmt_inner(&self, indent: usize) -> String {
        return match self {
            Node::Leaf(val) => format!("| {}", val),
            Node::Branch(id, ref low, ref high) =>
                format!(
                    "| {:?}:\n{}{}\n{}{}",
                    id,
                    "| ".repeat(indent),
                    low.fmt_inner(indent + 1),
                    "| ".repeat(indent),
                    high.fmt_inner(indent + 1)
                )
        };
    }
}

impl Node {
    pub const TRUE_LEAF: Node = Node::Leaf(true);
    pub const FALSE_LEAF: Node = Node::Leaf(false);

    pub fn branch<L, H>(id: &Item, low: L, high: H) -> Node where L: Into<Node>, H: Into<Node> {
        Node::Branch(id.clone(), Box::new(low.into()), Box::new(high.into()))
    }
}

impl<'a> From<&'a Node> for Node {
    fn from(node: &Node) -> Self {
        node.clone()
    }
}

impl From<Box<Node>> for Node {
    fn from(node: Box<Node>) -> Self {
        *node.clone()
    }
}

impl<'a> From<&'a Box<Node>> for Node {
    fn from(node: &Box<Node>) -> Self {
        *node.clone()
    }
}
