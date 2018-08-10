use bdd::node::Node;
use core::Item;

impl Node {
    pub fn restrict(node: &Node, item: &Item, selected: bool) -> Node {
        return match node {
            Node::Leaf(true) => Node::TRUE_LEAF,
            Node::Leaf(false) => Node::FALSE_LEAF,
            Node::Branch(id, ref low, ref high) => {
                if id == item {
                    if !selected {
                        return (**low).clone();
                    } else {
                        return (**high).clone();
                    }
                }

                let restricted_low = Node::restrict(low, item, selected);
                let restricted_high = Node::restrict(high, item, selected);

                if restricted_low == restricted_high {
                    return restricted_low;
                }

                return Node::branch(id, restricted_low, restricted_high);
            }
        };
    }
}

#[cfg(test)]
mod restrict_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn selecting_child_returns_correct_node() {
        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

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
        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

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