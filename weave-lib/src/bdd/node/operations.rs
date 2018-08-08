use bdd::node::Node;

pub trait Operation {
    fn eval(&self, f1: &Node, f2: &Node) -> Option<Node>;
}

pub struct AndOperation;

impl AndOperation {
    pub fn new() -> AndOperation {
        AndOperation {}
    }
}

impl Operation for AndOperation {
    fn eval(&self, f1: &Node, f2: &Node) -> Option<Node> {
        if &Node::TRUE_LEAF == f1 {
            return Some(f2.clone());
        }
        if &Node::TRUE_LEAF == f2 {
            return Some(f1.clone());
        }

        if let Node::Leaf(val_1) = f1 {
            if let Node::Leaf(val_2) = f2 {
                return Some(Node::Leaf(val_1 & val_2));
            }
        }

        None
    }
}
