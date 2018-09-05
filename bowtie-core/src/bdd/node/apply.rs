use bdd::node;
use bdd::node::Node;
use bdd::node::operations::Operation;
use core::Item;
use std::cmp::Ordering;

pub fn apply(node1: &Node, node2: &Node, op: &Operation) -> Node {
    if let Some(result) = op.eval(node1, node2) {
        return result;
    }

    let first_id = get_first_id(node1, node2).expect("BLAH");

    let (node1_low, node1_high) = split_branch(node1, &first_id);
    let (node2_low, node2_high) = split_branch(node2, &first_id);

    let low = apply(&node1_low, &node2_low, op);
    let high = apply(&node1_high, &node2_high, op);

    if low == high {
        return low.clone();
    }

    Node::branch(&first_id, low, high)
}

fn get_first_id(node1: &Node, node2: &Node) -> Option<Item> {
    match node1 {
        Node::Leaf(_) =>
            match node2 {
                Node::Leaf(_) => None,
                Node::Branch(id_2, _low, _high) => Some(id_2.clone())
            },
        Node::Branch(id_1, _low, _high) =>
            match node2 {
                Node::Leaf(_) => Some(id_1.clone()),
                Node::Branch(id_2, _low, _high) =>
                    match id_1.cmp(&id_2) {
                        Ordering::Less => Some(id_1.clone()),
                        Ordering::Equal => Some(id_1.clone()),
                        Ordering::Greater => Some(id_2.clone()),
                    },
            }
    }
}

fn split_branch(node: &Node, first_id: &Item) -> (Node, Node) {
    if let Node::Branch(id, low, high) = node {
        if first_id == id {
            let low = node::get(*low);
            let high = node::get(*high);
            return (low, high);
        }
    };

    (node.clone(), node.clone())
}

#[cfg(test)]
mod tests {
    use bdd::node::Node;
    use bdd::node::operations::AndOperation;
    use core::Item;
    use super::apply;

    #[test]
    fn apply_test() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let blue_low_branch = Node::positive_branch(&red);
        let blue_high_branch = Node::negative_branch(&red);
        let blue_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

        let slacks_low_branch = Node::positive_branch(&jeans);
        let slacks_high_branch = Node::negative_branch(&jeans);
        let slacks_branch = Node::branch(&slacks, slacks_low_branch, slacks_high_branch);

        let actual = apply(&slacks_branch, &blue_branch, &AndOperation::new());

        let expected = {
            let slacks_low_branch = Node::branch(&jeans, Node::FALSE_LEAF, &blue_branch);
            let slacks_high_branch = Node::branch(&jeans, blue_branch, Node::FALSE_LEAF);
            let slacks_branch = Node::branch(&slacks, slacks_low_branch, slacks_high_branch);

            slacks_branch
        };
        assert_eq!(expected, actual);
    }
}