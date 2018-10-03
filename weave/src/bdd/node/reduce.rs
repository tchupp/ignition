use bdd::node::Node;

impl Node {
    pub fn reduce(node: &Node) -> Node {
        match node {
            Node::Leaf(true) => Node::TRUE_LEAF,
            Node::Leaf(false) => Node::FALSE_LEAF,
            Node::Branch(id, low, high) => {
                let low = Node::from(low);
                let high = Node::from(high);

                let reduced_low = Node::reduce(&low);
                let reduced_high = Node::reduce(&high);

                if reduced_low == reduced_high {
                    return reduced_low;
                }

                Node::branch(id, reduced_low, reduced_high)
            },
        }
    }
}

#[cfg(test)]
mod reduce_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn or_can_be_reduced_if_low_and_high_are_equal() {
        let jeans = Item::new("pants:jeans");
        let blue_shirt = Item::new("shirts:blue");

        let low_branch = Node::positive_branch(&jeans);
        let high_branch = Node::positive_branch(&jeans);
        let parent_branch = Node::branch(&blue_shirt, low_branch, high_branch);

        let actual = Node::reduce(&parent_branch);

        let expected = Node::positive_branch(&jeans);
        assert_eq!(
            expected,
            actual
        );
    }


    #[test]
    fn sibling_relationship_cannot_be_reduced_in_nodes() {
        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let low_branch = Node::negative_branch(&jeans);
        let high_branch = Node::positive_branch(&jeans);
        let parent_branch = Node::branch(&slacks, &low_branch, &high_branch);

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&slacks, low_branch, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn exclusion_rule_can_be_reduced() {
        let jeans = Item::new("pants:jeans");
        let blue_shirt = Item::new("shirts:blue");

        let low_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::TRUE_LEAF);
        let high_branch = Node::negative_branch(&jeans);
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
        let jeans = Item::new("pants:jeans");
        let blue_shirt = Item::new("shirts:blue");

        let low_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::TRUE_LEAF);
        let high_branch = Node::positive_branch(&jeans);
        let parent_branch = Node::branch(&blue_shirt, low_branch.clone(), high_branch.clone());

        let actual = Node::reduce(&parent_branch);

        let expected = Node::branch(&blue_shirt, Node::TRUE_LEAF, high_branch);
        assert_eq!(
            expected,
            actual
        );
    }
}