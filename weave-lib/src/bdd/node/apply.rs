use bdd::node::Node;
use bdd::node::operations::Operation;
use core::Item;
use std::cmp::Ordering;

fn get_node(id: &Item, low: &Node, high: &Node) -> Node {
    if low == high {
        return low.clone();
    }

    return Node::branch(id, low, high);
}

fn get_first_id(f1: &Node, f2: &Node) -> Option<Item> {
    match f1 {
        Node::Leaf(_) => {
            match f2 {
                Node::Leaf(_) => None,
                Node::Branch(id_2, ref _low, ref _high) => Some(id_2.clone())
            }
        }
        Node::Branch(id_1, ref _low, ref _high) => {
            match f2 {
                Node::Leaf(_) => Some(id_1.clone()),
                Node::Branch(id_2, ref _low, ref _high) => match id_1.cmp(&id_2) {
                    Ordering::Less => Some(id_1.clone()),
                    Ordering::Equal => Some(id_1.clone()),
                    Ordering::Greater => Some(id_2.clone()),
                }
            }
        }
    }
}

pub fn apply(f1: &Node, f2: &Node, op: &Operation) -> Node {
    if let Some(result) = op.eval(f1, f2) {
        return result;
    }

    let first_id = get_first_id(f1, f2).expect("BLAH");
    let (f1_l, f1_h) = if let Node::Branch(id, ref low, ref high) = f1 {
        if &first_id == id {
            (&**low, &**high)
        } else {
            (f1, f1)
        }
    } else {
        (f1, f1)
    };

    let (f2_l, f2_h) = if let Node::Branch(id, ref low, ref high) = f2 {
        if &first_id == id {
            (&**low, &**high)
        } else {
            (f2, f2)
        }
    } else {
        (f2, f2)
    };

    let low = apply(f1_l, f2_l, op);
    let high = apply(f1_h, f2_h, op);
    return get_node(&first_id, &low, &high);
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