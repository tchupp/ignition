use core::Item;
use std::collections::HashMap;
use std::fmt;
use std::ops::BitAnd;
use std::prelude::v1::Vec;

#[derive(Eq, PartialEq, Clone, Hash)]
pub enum Node {
    TrueLeaf,
    FalseLeaf,
    Branch(Item, Box<Node>, Box<Node>),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt_inner(2))
    }
}

impl Node {
    fn fmt_inner(&self, indent: usize) -> String {
        return match self {
            Node::TrueLeaf => format!("-- True"),
            Node::FalseLeaf => format!("-- False"),
            Node::Branch(id, ref low, ref high) =>
                format!(
                    "{:?}\n{}{}\n{}{}",
                    id,
                    "  ".repeat(indent),
                    low.fmt_inner(indent + 1),
                    "  ".repeat(indent),
                    high.fmt_inner(indent + 1)
                )
        };
    }
}

impl Node {
    pub const TRUE_LEAF: Node = Node::TrueLeaf;
    pub const FALSE_LEAF: Node = Node::FalseLeaf;

    pub fn branch(id: &Item, low: Node, high: Node) -> Node {
        Node::Branch(id.clone(), Box::new(low), Box::new(high))
    }

    pub fn xor(id: &Item, sibling: Node) -> Node {
        Node::branch(id, sibling.clone(), Node::prime(&sibling))
    }

    pub fn prime(node: &Node) -> Node {
        return match node {
            Node::TrueLeaf => Node::FALSE_LEAF,
            Node::FalseLeaf => Node::TRUE_LEAF,
            Node::Branch(id, ref low, ref high) => {
                return Node::branch(id, (**high).clone(), (**low).clone());
            }
        };
    }

    pub fn reduce(node: &Node) -> Node {
        return match node {
            Node::TrueLeaf => node.clone(),
            Node::FalseLeaf => node.clone(),
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

    pub fn apply(node: &Node, item: &Item, selected: bool) -> Node {
        return match node {
            Node::TrueLeaf => Node::TRUE_LEAF,
            Node::FalseLeaf => Node::FALSE_LEAF,
            Node::Branch(id, ref low, ref high) => {
                if id == item {
                    if !selected {
                        let l = &**low;
                        return l.clone();
                    } else {
                        let r = &**high;
                        return r.clone();
                    }
                }

                let applied_low = Node::apply(low, item, selected);
                let applied_high = Node::apply(high, item, selected);

                return Node::branch(id, applied_low, applied_high);
            }
        };
    }
}

impl BitAnd for Node {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        if let Node::TrueLeaf = rhs {
            return self;
        }
        if let Node::TrueLeaf = self {
            return rhs;
        }

        if let Node::FalseLeaf = rhs {
            return rhs;
        }
        if let Node::FalseLeaf = self {
            return self;
        }

        if let Node::Branch(id, low, high) = self {
            return Node::branch(&id, *low & rhs.clone(), *high & rhs);
        }

        panic!("shouldn't get here");
    }
}

#[cfg(test)]
mod reduce_tests {
    use bdd::node::Node;
    use bdd::node::Node::*;
    use core::Item;

    #[test]
    fn or_can_be_reduced_if_low_and_high_are_equal() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&blue_shirt, low_branch, high_branch);

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn or_can_be_reduced_if_low_and_high_are_equal_iter() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&blue_shirt, low_branch, high_branch);

        let actual = Node::reduce_iter(&parent_branch);

        let expected = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn sibling_relationship_cannot_be_reduced_in_nodes() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let low_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&slacks, low_branch, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn sibling_relationship_cannot_be_reduced_in_nodes_iter() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let low_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce_iter(&parent_branch);

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

        let low_branch = Node::branch(&jeans, TrueLeaf, TrueLeaf);
        let high_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let parent_branch = Node::branch(&blue_shirt, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, TrueLeaf, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn exclusion_rule_can_be_reduced_iter() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, TrueLeaf, TrueLeaf);
        let high_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let parent_branch = Node::branch(&blue_shirt, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce_iter(&parent_branch);

        let expected = Node::branch(&blue_shirt, TrueLeaf, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn inclusion_rule_can_be_reduced() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, TrueLeaf, TrueLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&blue_shirt, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, TrueLeaf, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn inclusion_rule_can_be_reduced_iter() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let low_branch = Node::branch(&jeans, TrueLeaf, TrueLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&blue_shirt, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce_iter(&parent_branch);

        let expected = Node::branch(&blue_shirt, TrueLeaf, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }
}

#[cfg(test)]
mod apply_tests {
    use bdd::node::Node;
    use bdd::node::Node::FalseLeaf;
    use bdd::node::Node::TrueLeaf;
    use core::Item;

    #[test]
    fn selecting_child_returns_correct_node() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let low_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, low_branch.clone(), high_branch.clone());

        let actual = Node::apply(&parent_branch, &jeans, true);

        let expected = Node::branch(&slacks, FalseLeaf, TrueLeaf);
        assert_eq!(
            expected,
            actual
        );


        let actual = Node::apply(&parent_branch, &jeans, false);

        let expected = Node::branch(&slacks, TrueLeaf, FalseLeaf);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn selecting_parent_returns_correct_node() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let low_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let high_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, low_branch.clone(), high_branch.clone());

        let actual = Node::apply(&parent_branch, &slacks, true);

        let expected = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        assert_eq!(
            expected,
            actual
        );


        let actual = Node::apply(&parent_branch, &slacks, false);

        let expected = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        assert_eq!(
            expected,
            actual
        );
    }
}

#[cfg(test)]
mod bitand_tests {
    use bdd::node::Node;
    use bdd::node::Node::FalseLeaf;
    use bdd::node::Node::TrueLeaf;
    use core::Item;

    #[test]
    fn and_leaf_nodes() {
        assert_eq!(TrueLeaf, TrueLeaf & TrueLeaf);
        assert_eq!(FalseLeaf, FalseLeaf & TrueLeaf);

        assert_eq!(FalseLeaf, TrueLeaf & FalseLeaf);
        assert_eq!(FalseLeaf, FalseLeaf & FalseLeaf);
    }

    #[test]
    fn and_leaf_node_with_branch() {
        let jeans = Item::new("jeans");

        let pants_family = Node::branch(&jeans, FalseLeaf, TrueLeaf);

        assert_eq!(pants_family.clone(), TrueLeaf & pants_family.clone());
        assert_eq!(pants_family.clone(), pants_family.clone() & TrueLeaf);

        assert_eq!(FalseLeaf, FalseLeaf & pants_family.clone());
        assert_eq!(FalseLeaf, pants_family.clone() & FalseLeaf);
    }

    #[test]
    fn and_two_branches() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let blue_false_branch = Node::branch(&red, FalseLeaf, TrueLeaf);
        let blue_true_branch = Node::branch(&red, TrueLeaf, FalseLeaf);
        let blue_branch = Node::branch(&blue, blue_false_branch.clone(), blue_true_branch.clone());

        let slacks_false_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let slacks_true_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let slacks_branch = Node::branch(&slacks, slacks_false_branch.clone(), slacks_true_branch.clone());

        let combo_node_1 = {
            let slacks_false_branch = Node::branch(&jeans, FalseLeaf, blue_branch.clone());
            let slacks_true_branch = Node::branch(&jeans, blue_branch.clone(), FalseLeaf);
            let slacks_branch = Node::branch(&slacks, slacks_false_branch.clone(), slacks_true_branch.clone());

            slacks_branch
        };
        assert_eq!(combo_node_1, slacks_branch.clone() & blue_branch.clone());

        let combo_node_2 = {
            let blue_false_branch = Node::branch(&red, FalseLeaf, slacks_branch.clone());
            let blue_true_branch = Node::branch(&red, slacks_branch.clone(), FalseLeaf);
            let blue_branch = Node::branch(&blue, blue_false_branch.clone(), blue_true_branch.clone());

            blue_branch
        };
        assert_eq!(combo_node_2, blue_branch & slacks_branch);
    }
}