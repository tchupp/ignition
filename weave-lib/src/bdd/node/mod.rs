use core::Item;
use std::collections::HashMap;
use std::fmt;
use std::ops::BitAnd;
use std::ops::Not;
use std::prelude::v1::Vec;

#[derive(Eq, PartialEq, Clone, Hash)]
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

    pub fn branch(id: &Item, low: Node, high: Node) -> Node {
        Node::Branch(id.clone(), Box::new(low), Box::new(high))
    }

    pub fn xor(id: &Item, sibling: Node) -> Node {
        Node::branch(id, sibling.clone(), !sibling)
    }

    pub fn nand(id: &Item, sibling: Node) -> Node {
        Node::branch(id, Node::TRUE_LEAF, !sibling)
    }

    pub fn reduce(node: &Node) -> Node {
        return match node {
            Node::Leaf(true) => Node::TRUE_LEAF,
            Node::Leaf(false) => Node::FALSE_LEAF,
            Node::Branch(id, ref low, ref high) => {
                let reduced_low = Node::reduce(low);
                let reduced_high = Node::reduce(high);

                if reduced_low == reduced_high {
                    return reduced_low;
                }

                return Node::branch(id, reduced_low, reduced_high);
            }
        };
    }

    pub fn reduce_iter(node: &Node) -> Node {
        let mut inner_stack: Vec<&Node> = vec![node];
        let mut stack: Vec<&Node> = vec![];

        while !inner_stack.is_empty() {
            let current = inner_stack.pop();

            if let Some(current_node) = current {
                if let Node::Branch(_id, ref low, ref high) = current_node {
                    stack.push(current_node);

                    if let Node::Branch(_id, ref _low, ref _high) = &**low {
                        inner_stack.push(low);
                    }
                    if let Node::Branch(_id, ref _low, ref _high) = &**high {
                        inner_stack.push(high);
                    }
                }
            }
        }
        stack.reverse();

        let reduce_cache = stack.iter()
            .fold(HashMap::new(), |mut reduce_cache: HashMap<&Node, Node>, &current_node| {
                reduce_cache.entry(current_node).or_insert_with(|| Node::reduce(current_node));
                reduce_cache
            });


        return reduce_cache.get(node).unwrap_or(node).clone();
    }

    pub fn restrict(node: &Node, item: &Item, selected: bool) -> Node {
        return match node {
            Node::Leaf(true) => Node::TRUE_LEAF,
            Node::Leaf(false) => Node::FALSE_LEAF,
            Node::Branch(id, ref low, ref high) => {
                if id == item {
                    if !selected {
                        return Node::restrict(&**low, item, selected);
                    } else {
                        return Node::restrict(&**high, item, selected);
                    }
                }

                let applied_low = Node::restrict(low, item, selected);
                let applied_high = Node::restrict(high, item, selected);

                return Node::branch(id, applied_low, applied_high);
            }
        };
    }
}

impl BitAnd for Node {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        if Node::TRUE_LEAF == rhs {
            return self;
        }
        if Node::TRUE_LEAF == self {
            return rhs;
        }

        if Node::FALSE_LEAF == rhs {
            return rhs;
        }
        if Node::FALSE_LEAF == self {
            return self;
        }
        if self == rhs {
            return self;
        }
        if self == !rhs.clone() {
            return Node::FALSE_LEAF;
        }

        if let Node::Branch(id, low, high) = self {
            return Node::branch(&id, *low & rhs.clone(), *high & rhs);
        }

        panic!("shouldn't get here");
    }
}

impl Not for Node {
    type Output = Self;

    fn not(self) -> Self {
        return match &self {
            Node::Leaf(true) => Node::FALSE_LEAF,
            Node::Leaf(false) => Node::TRUE_LEAF,
            Node::Branch(id, ref low, ref high) => {
                return Node::branch(&id, (**high).clone(), (**low).clone());
            }
        };
    }
}

#[cfg(test)]
mod reduce_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn or_can_be_reduced_if_low_and_high_are_equal() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let high_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let parent_branch = Node::branch(&blue_shirt, low_branch, high_branch);

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        assert_eq!(
            expected,
            actual
        );
    }


    #[test]
    fn sibling_relationship_cannot_be_reduced_in_nodes() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let low_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let high_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let parent_branch = Node::branch(&slacks, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&slacks, low_branch, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn exclusion_rule_can_be_reduced() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::TRUE_LEAF);
        let high_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let parent_branch = Node::branch(&blue_shirt, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, Node::TRUE_LEAF, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn inclusion_rule_can_be_reduced() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::TRUE_LEAF);
        let high_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let parent_branch = Node::branch(&blue_shirt, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, Node::TRUE_LEAF, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }
}

#[cfg(test)]
mod restrict_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn selecting_child_returns_correct_node() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let low_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let high_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let parent_branch = Node::branch(&slacks, low_branch.clone(), high_branch.clone());

        let actual = Node::restrict(&parent_branch, &jeans, true);

        let expected = Node::branch(&slacks, Node::FALSE_LEAF, Node::TRUE_LEAF);
        assert_eq!(
            expected,
            actual
        );


        let actual = Node::restrict(&parent_branch, &jeans, false);

        let expected = Node::branch(&slacks, Node::TRUE_LEAF, Node::FALSE_LEAF);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn selecting_parent_returns_correct_node() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let low_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let high_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let parent_branch = Node::branch(&slacks, low_branch.clone(), high_branch.clone());

        let actual = Node::restrict(&parent_branch, &slacks, true);

        let expected = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        assert_eq!(
            expected,
            actual
        );


        let actual = Node::restrict(&parent_branch, &slacks, false);

        let expected = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
        assert_eq!(
            expected,
            actual
        );
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
        let jeans = Item::new("jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), Node::TRUE_LEAF & pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() & Node::TRUE_LEAF);

        assert_eq!(Node::FALSE_LEAF, Node::FALSE_LEAF & pants_family.clone());
        assert_eq!(Node::FALSE_LEAF, pants_family.clone() & Node::FALSE_LEAF);
    }

    #[test]
    fn and_with_identical_branches_is_a_tautology() {
        let jeans = Item::new("jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(pants_family.clone(), pants_family.clone() & pants_family.clone());
    }

    #[test]
    fn and_with_opposite_branches_is_always_false() {
        let jeans = Item::new("jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let prime_pants_family = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

        assert_eq!(Node::FALSE_LEAF, pants_family.clone() & prime_pants_family.clone());
    }

    #[test]
    fn and_two_branches() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let blue_false_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let blue_true_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let blue_branch = Node::branch(&blue, blue_false_branch.clone(), blue_true_branch.clone());

        let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let slacks_true_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let slacks_branch = Node::branch(&slacks, slacks_false_branch.clone(), slacks_true_branch.clone());

        let combo_node_1 = {
            let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, blue_branch.clone());
            let slacks_true_branch = Node::branch(&jeans, blue_branch.clone(), Node::FALSE_LEAF);
            let slacks_branch = Node::branch(&slacks, slacks_false_branch.clone(), slacks_true_branch.clone());

            slacks_branch
        };
        assert_eq!(combo_node_1, slacks_branch.clone() & blue_branch.clone());

        let combo_node_2 = {
            let blue_false_branch = Node::branch(&red, Node::FALSE_LEAF, slacks_branch.clone());
            let blue_true_branch = Node::branch(&red, slacks_branch.clone(), Node::FALSE_LEAF);
            let blue_branch = Node::branch(&blue, blue_false_branch.clone(), blue_true_branch.clone());

            blue_branch
        };
        assert_eq!(combo_node_2, blue_branch & slacks_branch);
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
    fn nand_leaf_node_with_branch() {
        let jeans = Item::new("jeans");

        let pants_family = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let nand_pants_family = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

        assert_eq!(nand_pants_family.clone(), !(Node::TRUE_LEAF & pants_family.clone()));
        assert_eq!(nand_pants_family.clone(), !(pants_family.clone() & Node::TRUE_LEAF));
        assert_eq!(nand_pants_family.clone(), Node::nand(&jeans, Node::TRUE_LEAF));

        assert_eq!(Node::TRUE_LEAF, !(Node::FALSE_LEAF & pants_family.clone()));
        assert_eq!(Node::TRUE_LEAF, !(pants_family.clone() & Node::FALSE_LEAF));
    }

    #[test]
    fn nand_two_branches() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let blue_false_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let blue_true_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let blue_branch = Node::branch(&blue, blue_false_branch.clone(), blue_true_branch.clone());

        let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let slacks_true_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let slacks_branch = Node::branch(&slacks, slacks_false_branch.clone(), slacks_true_branch.clone());

        let combo_node_1 = {
            let slacks_high_branch = Node::branch(&jeans, Node::FALSE_LEAF, blue_branch.clone());
            let slacks_low_branch = Node::branch(&jeans, blue_branch.clone(), Node::FALSE_LEAF);
            let slacks_branch = Node::branch(&slacks, slacks_low_branch.clone(), slacks_high_branch.clone());

            slacks_branch
        };
        assert_eq!(combo_node_1, !(slacks_branch.clone() & blue_branch.clone()));

        let combo_node_2 = {
            let blue_high_branch = Node::branch(&red, Node::FALSE_LEAF, slacks_branch.clone());
            let blue_low_branch = Node::branch(&red, slacks_branch.clone(), Node::FALSE_LEAF);
            let blue_branch = Node::branch(&blue, blue_low_branch.clone(), blue_high_branch.clone());

            blue_branch
        };
        assert_eq!(combo_node_2, !(blue_branch & slacks_branch));
    }

    #[test]
    fn nand_two_items_from_different_families() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");

        let combo_node_1 = {
            let blue_low_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
            let shirts_family_branch = Node::branch(&blue, blue_low_branch.clone(), Node::FALSE_LEAF);

            let pants_family_branch = Node::branch(&jeans, Node::FALSE_LEAF, shirts_family_branch);

            pants_family_branch
        };
        let actual = {
            let blue_high_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let blue_low_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
            let shirts_family_branch = Node::branch(&blue, blue_low_branch.clone(), blue_high_branch.clone());

            let pants_family_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);

            let root = pants_family_branch & shirts_family_branch;

            let jeans_exclude_blue = {
                let jeans_exclude_blue = Node::restrict(&root, &jeans, true);
                let jeans_exclude_blue = Node::restrict(&jeans_exclude_blue, &blue, false);
                jeans_exclude_blue
            };

            root & jeans_exclude_blue
        };

        assert_eq!(combo_node_1, actual);
    }
}