use core::Item;

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
}

pub fn reduce(node: &Node) -> Node {
    return match node {
        Node::TrueLeaf => node.clone(),
        Node::FalseLeaf => node.clone(),
        Node::Branch(id, ref left, ref right) => {
            let reduced_left = reduce(left);
            let reduced_right = reduce(right);

            if reduced_left == reduced_right {
                return reduced_left;
            }

            return Node::branch(id, reduced_left, reduced_right);
        }
    };
}

pub fn apply(node: &Node, item: &Item, selected: bool) -> Node {
    return match node {
        Node::TrueLeaf => node.clone(),
        Node::FalseLeaf => node.clone(),
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

            let applied_left = apply(left, item, selected);
            let applied_right = apply(right, item, selected);

            return Node::branch(id, applied_left, applied_right);
        }
    };
}

#[cfg(test)]
mod reduce_tests {
    use core::Item;
    use super::Node;
    use super::Node::*;
    use super::reduce;

    #[test]
    fn or_can_be_reduced_if_left_and_right_are_equal() {
        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let right_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&blue_shirt, left_branch, right_branch);

        let actual = reduce(&parent_branch);

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

        let actual = reduce(&parent_branch);

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

        let actual = reduce(&parent_branch);

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

        let actual = reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, TrueLeaf, right_branch);
        assert_eq!(
            expected,
            actual
        );
    }
}

#[cfg(test)]
mod apply_tests {
    use core::Item;
    use super::apply;
    use super::Node;
    use super::Node::FalseLeaf;
    use super::Node::TrueLeaf;

    #[test]
    fn selecting_child_returns_correct_node() {
        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let left_branch = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        let right_branch = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        let parent_branch = Node::branch(&slacks, left_branch.clone(), right_branch.clone());

        let actual = apply(&parent_branch, &jeans, true);

        let expected = Node::branch(&slacks, FalseLeaf, TrueLeaf);
        assert_eq!(
            expected,
            actual
        );


        let actual = apply(&parent_branch, &jeans, false);

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

        let actual = apply(&parent_branch, &slacks, true);

        let expected = Node::branch(&jeans, FalseLeaf, TrueLeaf);
        assert_eq!(
            expected,
            actual
        );


        let actual = apply(&parent_branch, &slacks, false);

        let expected = Node::branch(&jeans, TrueLeaf, FalseLeaf);
        assert_eq!(
            expected,
            actual
        );
    }
}