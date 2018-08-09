use bdd::node::Node;
use core::Item;

impl From<Item> for Node {
    fn from(item: Item) -> Self {
        Node::branch(&item, Node::FALSE_LEAF, Node::TRUE_LEAF)
    }
}

impl <'a> From<&'a Item> for Node {
    fn from(item: &Item) -> Self {
        Node::branch(item, Node::FALSE_LEAF, Node::TRUE_LEAF)
    }
}