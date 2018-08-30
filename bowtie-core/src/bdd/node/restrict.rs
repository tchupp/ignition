use bdd::node;
use bdd::node::Node;
use core::Item;

impl Node {
    pub fn restrict(node: &Node, item: &Item, selected: bool) -> Node {
        match node {
            Node::Leaf(true) => Node::TRUE_LEAF,
            Node::Leaf(false) => Node::FALSE_LEAF,
            Node::Branch(id, low, high) => {
                let low = node::get(*low);
                let high = node::get(*high);

                if id == item {
                    return if !selected { low } else { high };
                }

                let restricted_low = Node::restrict(&low, item, selected);
                let restricted_high = Node::restrict(&high, item, selected);

                if restricted_low == restricted_high {
                    return restricted_low;
                }

                Node::branch(id, restricted_low, restricted_high)
            }
        }
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

        let low_branch = Node::negative_branch(&jeans);
        let high_branch = Node::positive_branch(&jeans);
        let parent_branch = Node::branch(&slacks, low_branch, high_branch);

        let actual = Node::restrict(&parent_branch, &jeans, true);

        let expected = Node::positive_branch(&slacks);
        assert_eq!(
            expected,
            actual
        );


        let actual = Node::restrict(&parent_branch, &jeans, false);

        let expected = Node::negative_branch(&slacks);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn selecting_parent_returns_correct_node() {
        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let low_branch = Node::negative_branch(&jeans);
        let high_branch = Node::positive_branch(&jeans);
        let parent_branch = Node::branch(&slacks, low_branch, high_branch);

        let actual = Node::restrict(&parent_branch, &slacks, true);

        let expected = Node::positive_branch(&jeans);
        assert_eq!(
            expected,
            actual
        );


        let actual = Node::restrict(&parent_branch, &slacks, false);

        let expected = Node::negative_branch(&jeans);
        assert_eq!(
            expected,
            actual
        );
    }
}