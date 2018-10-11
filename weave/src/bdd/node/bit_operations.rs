use bdd::node::apply::apply;
use bdd::node::Node;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;

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

        match (f1, f2) {
            (Node::Leaf(val_1), Node::Leaf(val_2)) => Some(Node::Leaf(val_1 & val_2)),
            _ => None
        }
    }
}

pub struct OrOperation;

impl OrOperation {
    pub fn new() -> OrOperation {
        OrOperation {}
    }
}

impl Operation for OrOperation {
    fn eval(&self, f1: &Node, f2: &Node) -> Option<Node> {
        if &Node::FALSE_LEAF == f1 {
            return Some(f2.clone());
        }
        if &Node::FALSE_LEAF == f2 {
            return Some(f1.clone());
        }

        match (f1, f2) {
            (Node::Leaf(val_1), Node::Leaf(val_2)) => Some(Node::Leaf(val_1 | val_2)),
            _ => None
        }
    }
}


impl BitOr for Node {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        apply(&self, &rhs, &OrOperation::new())
    }
}

impl BitAnd for Node {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        apply(&self, &rhs, &AndOperation::new())
    }
}

impl BitXor for Node {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        ((self.clone() & !rhs.clone()) | (!self.clone() & rhs.clone()))
    }
}

impl Not for Node {
    type Output = Self;

    fn not(self) -> Self {
        match &self {
            Node::Leaf(true) => Node::FALSE_LEAF,
            Node::Leaf(false) => Node::TRUE_LEAF,
            Node::Branch(id, low, high) => Node::branch(id, *high, *low)
        }
    }
}

#[cfg(test)]
mod bitand_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn and_leaf_nodes() {
        assert_eq!(Node::TRUE_LEAF, Node::TRUE_LEAF & Node::TRUE_LEAF);
        assert_eq!(Node::FALSE_LEAF, Node::FALSE_LEAF & Node::TRUE_LEAF);

        assert_eq!(Node::FALSE_LEAF, Node::TRUE_LEAF & Node::FALSE_LEAF);
        assert_eq!(Node::FALSE_LEAF, Node::FALSE_LEAF & Node::FALSE_LEAF);
    }

    #[test]
    fn and_leaf_node_with_branch() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);

        assert_eq!(pants_family.clone(), Node::TRUE_LEAF & pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() & Node::TRUE_LEAF);

        assert_eq!(Node::FALSE_LEAF, Node::FALSE_LEAF & pants_family.clone());
        assert_eq!(Node::FALSE_LEAF, pants_family.clone() & Node::FALSE_LEAF);
    }

    #[test]
    fn and_with_identical_branches_is_a_tautology() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);

        assert_eq!(pants_family.clone(), pants_family.clone() & pants_family.clone());
    }

    #[test]
    fn and_with_opposite_branches_is_always_false() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);
        let prime_pants_family = Node::negative_branch(&jeans);

        assert_eq!(Node::FALSE_LEAF, pants_family.clone() & prime_pants_family.clone());
    }

    #[test]
    fn and_two_branches() {
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

        let expected = {
            let slacks_low_branch = Node::branch(&jeans, Node::FALSE_LEAF, &blue_branch);
            let slacks_high_branch = Node::branch(&jeans, &blue_branch, Node::FALSE_LEAF);
            let slacks_branch = Node::branch(&slacks, slacks_low_branch, slacks_high_branch);

            slacks_branch
        };
        assert_eq!(expected, slacks_branch.clone() & blue_branch.clone());
        assert_eq!(expected, blue_branch & slacks_branch);
    }
}

#[cfg(test)]
mod bitnand_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn nand_leaf_nodes() {
        assert_eq!(Node::FALSE_LEAF, !(Node::TRUE_LEAF & Node::TRUE_LEAF));
        assert_eq!(Node::TRUE_LEAF, !(Node::FALSE_LEAF & Node::TRUE_LEAF));

        assert_eq!(Node::TRUE_LEAF, !(Node::TRUE_LEAF & Node::FALSE_LEAF));
        assert_eq!(Node::TRUE_LEAF, !(Node::FALSE_LEAF & Node::FALSE_LEAF));
    }

    #[test]
    fn nand_two_branches() {
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

        let expected = {
            let slacks_high_branch = Node::branch(&jeans, Node::FALSE_LEAF, &blue_branch);
            let slacks_low_branch = Node::branch(&jeans, &blue_branch, Node::FALSE_LEAF);
            let slacks_branch = Node::branch(&slacks, slacks_low_branch, slacks_high_branch);

            slacks_branch
        };
        assert_eq!(expected, !(slacks_branch.clone() & blue_branch.clone()));
        assert_eq!(expected, !(blue_branch & slacks_branch));
    }

    #[test]
    fn nand_two_items_from_different_families() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");

        let expected = {
            let red_branch = Node::negative_branch(&red);
            let blue_branch = Node::branch(&blue, red_branch, Node::FALSE_LEAF);
            let jeans_branch = Node::branch(&jeans, Node::FALSE_LEAF, blue_branch);

            jeans_branch
        };
        let actual = {
            let blue_high_branch = Node::positive_branch(&red);
            let blue_low_branch = Node::negative_branch(&red);
            let shirts_family_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

            let pants_family_branch = Node::positive_branch(&jeans);

            let root = pants_family_branch & shirts_family_branch;

            let jeans_exclude_blue = {
                let jeans_exclude_blue = Node::restrict(&root, &jeans, true);
                let jeans_exclude_blue = Node::restrict(&jeans_exclude_blue, &blue, false);
                jeans_exclude_blue
            };

            root & jeans_exclude_blue
        };

        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod bitor_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn or_leaf_nodes() {
        assert_eq!(Node::TRUE_LEAF, Node::TRUE_LEAF | Node::TRUE_LEAF);
        assert_eq!(Node::TRUE_LEAF, Node::FALSE_LEAF | Node::TRUE_LEAF);
        assert_eq!(Node::TRUE_LEAF, Node::TRUE_LEAF | Node::FALSE_LEAF);
        assert_eq!(Node::FALSE_LEAF, Node::FALSE_LEAF | Node::FALSE_LEAF);
    }

    #[test]
    fn or_leaf_node_with_branch() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);

        assert_eq!(Node::TRUE_LEAF, Node::TRUE_LEAF | pants_family.clone());
        assert_eq!(Node::TRUE_LEAF, pants_family.clone() | Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), Node::FALSE_LEAF | pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() | Node::FALSE_LEAF);
    }

    #[test]
    fn or_with_identical_branches_is_a_tautology() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);

        assert_eq!(pants_family.clone(), pants_family.clone() | pants_family.clone());
    }

    #[test]
    fn or_with_opposite_branches_is_always_false() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);
        let prime_pants_family = Node::negative_branch(&jeans);

        assert_eq!(Node::TRUE_LEAF, pants_family.clone() | prime_pants_family.clone());
    }

    #[test]
    fn or_two_branches() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let expected = {
            let red_branch = Node::positive_branch(&red);
            let blue_branch = Node::branch(&blue, red_branch, Node::TRUE_LEAF);
            let slacks_branch = Node::branch(&slacks, blue_branch, Node::TRUE_LEAF);
            let jeans_branch = Node::branch(&jeans, slacks_branch, Node::TRUE_LEAF);

            jeans_branch
        };

        let actual = Node::positive_branch(&red) | Node::positive_branch(&blue) | Node::positive_branch(&jeans) | Node::positive_branch(&slacks);
        assert_eq!(expected, actual);

        let actual = Node::positive_branch(&blue) | Node::positive_branch(&red) | Node::positive_branch(&slacks) | Node::positive_branch(&jeans);
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod bitxor_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn xor_leaf_nodes() {
        assert_eq!(Node::FALSE_LEAF, Node::TRUE_LEAF ^ Node::TRUE_LEAF);
        assert_eq!(Node::TRUE_LEAF, Node::FALSE_LEAF ^ Node::TRUE_LEAF);
        assert_eq!(Node::TRUE_LEAF, Node::TRUE_LEAF ^ Node::FALSE_LEAF);
        assert_eq!(Node::FALSE_LEAF, Node::FALSE_LEAF ^ Node::FALSE_LEAF);
    }

    #[test]
    fn xor_leaf_node_with_branch() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);

        assert_eq!(!pants_family.clone(), Node::TRUE_LEAF ^ pants_family.clone());
        assert_eq!(!pants_family.clone(), pants_family.clone() ^ Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), Node::FALSE_LEAF ^ pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() ^ Node::FALSE_LEAF);
    }

    #[test]
    fn xor_with_identical_branches_is_always_false() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);

        assert_eq!(Node::FALSE_LEAF, pants_family.clone() ^ pants_family.clone());
    }

    #[test]
    fn xor_with_opposite_branches_is_always_true() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::positive_branch(&jeans);
        let prime_pants_family = Node::negative_branch(&jeans);

        assert_eq!(Node::TRUE_LEAF, pants_family.clone() ^ prime_pants_family.clone());
    }

    #[test]
    fn xor_between_two_different_positive_items_expands() {
        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let jeans_branch = Node::positive_branch(&jeans);
        let slacks_branch = Node::positive_branch(&slacks);

        let expected = {
            let slacks_branch = Node::positive_branch(&slacks);
            let slacks_branch_prime = Node::negative_branch(&slacks);

            Node::branch(&jeans, slacks_branch, slacks_branch_prime)
        };

        assert_eq!(expected, jeans_branch ^ slacks_branch);
    }

    #[test]
    fn xor_two_branches() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let expected = {
            let blue_low_branch = Node::positive_branch(&red);
            let blue_high_branch = Node::negative_branch(&red);
            let blue_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

            let jeans_low_branch = Node::branch(&slacks, blue_branch.clone(), !blue_branch.clone());
            let jeans_high_branch = Node::branch(&slacks, !blue_branch.clone(), blue_branch.clone());
            let jeans_branch = Node::branch(&jeans, jeans_low_branch, jeans_high_branch);

            jeans_branch
        };

        let actual = Node::positive_branch(&red) ^ Node::positive_branch(&blue) ^ Node::positive_branch(&jeans) ^ Node::positive_branch(&slacks);
        assert_eq!(expected, actual);

        let actual = Node::positive_branch(&blue) ^ Node::positive_branch(&red) ^ Node::positive_branch(&slacks) ^ Node::positive_branch(&jeans);
        assert_eq!(expected, actual);
    }
}