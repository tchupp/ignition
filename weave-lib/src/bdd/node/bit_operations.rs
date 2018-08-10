use bdd::node::apply::apply;
use bdd::node::Node;
use bdd::node::operations::AndOperation;
use bdd::node::operations::OrOperation;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;

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
        (self.clone() & !(rhs.clone())) | (!self.clone() & (rhs.clone()))
    }
}

impl Not for Node {
    type Output = Self;

    fn not(self) -> Self {
        return match &self {
            Node::Leaf(true) => Node::FALSE_LEAF,
            Node::Leaf(false) => Node::TRUE_LEAF,
            Node::Branch(id, ref low, ref high) => {
                return Node::branch(&id, high, low);
            }
        };
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

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), Node::TRUE_LEAF & pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() & Node::TRUE_LEAF);

        assert_eq!(Node::FALSE_LEAF, Node::FALSE_LEAF & pants_family.clone());
        assert_eq!(Node::FALSE_LEAF, pants_family.clone() & Node::FALSE_LEAF);
    }

    #[test]
    fn and_with_identical_branches_is_a_tautology() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), pants_family.clone() & pants_family.clone());
    }

    #[test]
    fn and_with_opposite_branches_is_always_false() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let prime_pants_family = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

        assert_eq!(Node::FALSE_LEAF, pants_family.clone() & prime_pants_family.clone());
    }

    #[test]
    fn and_two_branches() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let blue_low_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let blue_high_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let blue_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

        let slacks_low_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let slacks_high_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
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

        let blue_low_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let blue_high_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let blue_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

        let slacks_low_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let slacks_high_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
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
            let red_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
            let blue_branch = Node::branch(&blue, red_branch, Node::FALSE_LEAF);
            let jeans_branch = Node::branch(&jeans, Node::FALSE_LEAF, blue_branch);

            jeans_branch
        };
        let actual = {
            let blue_high_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let blue_low_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
            let shirts_family_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

            let pants_family_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

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

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(Node::TRUE_LEAF, Node::TRUE_LEAF | pants_family.clone());
        assert_eq!(Node::TRUE_LEAF, pants_family.clone() | Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), Node::FALSE_LEAF | pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() | Node::FALSE_LEAF);
    }

    #[test]
    fn or_with_identical_branches_is_a_tautology() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), pants_family.clone() | pants_family.clone());
    }

    #[test]
    fn or_with_opposite_branches_is_always_false() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let prime_pants_family = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

        assert_eq!(Node::TRUE_LEAF, pants_family.clone() | prime_pants_family.clone());
    }

    #[test]
    fn or_two_branches() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let expected = {
            let red_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let blue_branch = Node::branch(&blue, red_branch, Node::TRUE_LEAF);
            let slacks_branch = Node::branch(&slacks, blue_branch, Node::TRUE_LEAF);
            let jeans_branch = Node::branch(&jeans, slacks_branch, Node::TRUE_LEAF);

            jeans_branch
        };

        let actual = Node::from(&red) | Node::from(&blue) | Node::from(&jeans) | Node::from(&slacks);
        assert_eq!(expected, actual);

        let actual = Node::from(&blue) | Node::from(&red) | Node::from(&slacks) | Node::from(&jeans);
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

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(!pants_family.clone(), Node::TRUE_LEAF ^ pants_family.clone());
        assert_eq!(!pants_family.clone(), pants_family.clone() ^ Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), Node::FALSE_LEAF ^ pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() ^ Node::FALSE_LEAF);
    }

    #[test]
    fn xor_with_identical_branches_is_always_false() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(Node::FALSE_LEAF, pants_family.clone() ^ pants_family.clone());
    }

    #[test]
    fn xor_with_opposite_branches_is_always_true() {
        let jeans = Item::new("pants:jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let prime_pants_family = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

        assert_eq!(Node::TRUE_LEAF, pants_family.clone() ^ prime_pants_family.clone());
    }

    #[test]
    fn xor_two_branches() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let expected = {
            let blue_low_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let blue_high_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
            let blue_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

            let jeans_low_branch = Node::branch(&slacks, blue_branch.clone(), !blue_branch.clone());
            let jeans_high_branch = Node::branch(&slacks, !blue_branch.clone(), blue_branch.clone());
            let jeans_branch = Node::branch(&jeans, jeans_low_branch, jeans_high_branch);

            jeans_branch
        };

        let actual = Node::from(&red) ^ Node::from(&blue) ^ Node::from(&jeans) ^ Node::from(&slacks);
        assert_eq!(expected, actual);

        let actual = Node::from(&blue) ^ Node::from(&red) ^ Node::from(&slacks) ^ Node::from(&jeans);
        assert_eq!(expected, actual);
    }
}