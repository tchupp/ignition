use core::Item;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Node<'a> {
    TrueLeaf,
    FalseLeaf,
    Branch(&'a Item, Box<Node<'a>>, Box<Node<'a>>),
}

impl<'a> Node<'a> {
    fn branch(id: &'a Item, left: Node<'a>, right: Node<'a>) -> Node<'a> {
        Node::Branch(id, Box::new(left), Box::new(right))
    }
}

fn reduce<'a>(node: &'a Node<'a>) -> Node<'a> {
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

fn apply<'a>(node: &'a Node<'a>, item: &'a Item, selected: bool) -> Node<'a> {
    return match node {
        Node::TrueLeaf => node.clone(),
        Node::FalseLeaf => node.clone(),
        Node::Branch(id, ref left, ref right) => {
            if id == &item {
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
        let false_node = FalseLeaf;
        let true_node = TrueLeaf;

        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_branch = Node::branch(&jeans, false_node.clone(), true_node.clone());
        let right_branch = Node::branch(&jeans, false_node.clone(), true_node.clone());
        let parent_branch = Node::branch(&blue_shirt, left_branch, right_branch);

        let actual = reduce(&parent_branch);

        let expected = Node::branch(&jeans, false_node, true_node);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn sibling_relationship_cannot_be_reduced_in_nodes() {
        let false_node = FalseLeaf;
        let true_node = TrueLeaf;

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let left_branch = Node::branch(&jeans, true_node.clone(), false_node.clone());
        let right_branch = Node::branch(&jeans, false_node.clone(), true_node.clone());
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
        let false_node = FalseLeaf;
        let true_node = TrueLeaf;

        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_branch = Node::branch(&jeans, true_node.clone(), true_node.clone());
        let right_branch = Node::branch(&jeans, true_node.clone(), false_node.clone());
        let parent_branch = Node::branch(&blue_shirt, left_branch.clone(), right_branch.clone());

        let actual = reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, true_node.clone(), right_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn inclusion_rule_can_be_reduced() {
        let false_node = FalseLeaf;
        let true_node = TrueLeaf;

        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_branch = Node::branch(&jeans, true_node.clone(), true_node.clone());
        let right_branch = Node::branch(&jeans, false_node.clone(), true_node.clone());
        let parent_branch = Node::branch(&blue_shirt, left_branch.clone(), right_branch.clone());

        let actual = reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, true_node.clone(), right_branch);
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
        let false_node = FalseLeaf;
        let true_node = TrueLeaf;

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let left_branch = Node::branch(&jeans, true_node.clone(), false_node.clone());
        let right_branch = Node::branch(&jeans, false_node.clone(), true_node.clone());
        let parent_branch = Node::branch(&slacks, left_branch.clone(), right_branch.clone());

        let actual = apply(&parent_branch, &jeans, true);

        let expected = Node::branch(&slacks, false_node.clone(), true_node.clone());
        assert_eq!(
            expected,
            actual
        );


        let actual = apply(&parent_branch, &jeans, false);

        let expected = Node::branch(&slacks, true_node.clone(), false_node.clone());
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn selecting_parent_returns_correct_node() {
        let false_node = FalseLeaf;
        let true_node = TrueLeaf;

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let left_branch = Node::branch(&jeans, true_node.clone(), false_node.clone());
        let right_branch = Node::branch(&jeans, false_node.clone(), true_node.clone());
        let parent_branch = Node::branch(&slacks, left_branch.clone(), right_branch.clone());

        let actual = apply(&parent_branch, &slacks, true);

        let expected = Node::branch(&jeans, false_node.clone(), true_node.clone());
        assert_eq!(
            expected,
            actual
        );


        let actual = apply(&parent_branch, &slacks, false);

        let expected = Node::branch(&jeans, true_node.clone(), false_node.clone());
        assert_eq!(
            expected,
            actual
        );
    }
}