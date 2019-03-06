use super::Node;
use super::Priority;

type Matching = (Node, bool);

pub fn subset(root: Node, element: Priority) -> Node {
    subset_inner(root, element).0
}

fn subset_inner(root: Node, element: Priority) -> Matching {
    match root {
        Node::Branch(id, _low, high) if element == id => {
            let low = Node::NEVER;

            (Node::branch(id, low, high), true)
        }
        Node::Branch(id, low, high) => {
            let (low, keep_low) = reduce_branch(
                subset_inner(low.into(), element)
            );
            let (high, keep_high) = reduce_branch(
                subset_inner(high.into(), element)
            );

            let keep = keep_low || keep_high;

            (Node::branch(id, low, high), keep)
        }
        _ => (root, false),
    }
}

pub fn subset_not(root: Node, element: Priority) -> Node {
    match root {
        Node::Branch(id, low, _high) if element == id => {
            Node::from(low)
        }
        Node::Branch(id, low, high) => {
            let low = subset_not(low.into(), element);
            let high = subset_not(high.into(), element);

            Node::branch(id, low, high)
        }
        _ => root,
    }
}

pub fn subset_all(root: Node, elements: &[Priority]) -> Node {
    elements.iter()
        .map(|element| subset(root, element.to_owned()))
        .fold(Node::Always, Node::intersect)
}

pub fn subset_none(root: Node, elements: &[Priority]) -> Node {
    elements.iter()
        .map(|element| subset_not(root, element.to_owned()))
        .fold(Node::Always, Node::intersect)
}

fn reduce_branch((root, keep): Matching) -> Matching {
    match (root, keep) {
        (_root, false) => (Node::Never, keep),
        (root, true) => (root, keep)
    }
}
