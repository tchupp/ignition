use core::Item;
use std::ops::BitAnd;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Node {
    TrueLeaf,
    FalseLeaf,
    Branch(Item, Box<Node>, Box<Node>),
}

impl<'a> Node {
    pub fn branch(id: &Item, left: Node, right: Node) -> Node {
        Node::Branch(id.clone(), Box::new(left), Box::new(right))
    }

    pub fn xor(id: &Item, sibling: Node) -> Node {
        Node::branch(id, sibling.clone(), Node::prime(&sibling))
    }

    pub fn prime(node: &Node) -> Node {
        return match node {
            Node::TrueLeaf => Node::FalseLeaf,
            Node::FalseLeaf => Node::TrueLeaf,
            Node::Branch(id, ref left, ref right) => {
                return Node::branch(id, (**right).clone(), (**left).clone());
            }
        };
    }

    pub fn reduce(node: &Node) -> Node {
        return match node {
            Node::TrueLeaf => node.clone(),
            Node::FalseLeaf => node.clone(),
            Node::Branch(id, ref left, ref right) => {
                let reduced_left = Node::reduce(left);
                let reduced_right = Node::reduce(right);

                if reduced_left == reduced_right {
                    return reduced_left;
                }

                return Node::branch(id, reduced_left, reduced_right);
            }
        };
    }

    pub fn apply(node: &Node, item: &Item, selected: bool) -> Node {
        return match node {
            Node::TrueLeaf => Node::TrueLeaf,
            Node::FalseLeaf => Node::FalseLeaf,
            Node::Branch(id, ref left, ref right) => {
                if id == item {
                    if !selected {
                        let l = &**left;
                        return l.clone();
                    } else {
                        let r = &**right;
                        return r.clone();
                    }
                }

                let applied_left = Node::apply(left, item, selected);
                let applied_right = Node::apply(right, item, selected);

                return Node::reduce(&Node::branch(id, applied_left, applied_right));
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

        if let Node::Branch(id, left, right) = self {
            return Node::branch(&id, *left & rhs.clone(), *right & rhs);
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
    fn or_can_be_reduced_if_left_and_right_are_equal() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let right_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&blue_shirt, left_branch, right_branch);

        let actual = Node::reduce(&parent_branch);

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

        let left_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let right_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, left_branch.clone(), right_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&slacks, left_branch, right_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn exclusion_rule_can_be_reduced() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_branch = Node::branch(&jeans, TrueLeaf, TrueLeaf);
        let right_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let parent_branch = Node::branch(&blue_shirt, left_branch.clone(), right_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, TrueLeaf, right_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn inclusion_rule_can_be_reduced() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_branch = Node::branch(&jeans, TrueLeaf, TrueLeaf);
        let right_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&blue_shirt, left_branch.clone(), right_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, TrueLeaf, right_branch);
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

        let left_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let right_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, left_branch.clone(), right_branch.clone());

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

        let left_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let right_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, left_branch.clone(), right_branch.clone());

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
    use core::Family;
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