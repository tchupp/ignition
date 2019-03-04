use super::Node;

pub fn union(node1: Node, node2: Node) -> Node {
    if node1 == node2 {
        return node1;
    }

    let (id, low, high) = match (node1, node2) {
        (_, Node::Leaf(false)) => return node1,
        (Node::Leaf(false), _) => return node2,

        (Node::Leaf(true), Node::Leaf(true)) => return Node::Leaf(true),

        (Node::Branch(id, low, high), Node::Leaf(true)) => {
            let low = union(low.into(), node2);
            let high = Node::from(high);

            (id, low, high)
        }
        (Node::Leaf(true), Node::Branch(id, low, high)) => {
            let low = union(node1, low.into());
            let high = Node::from(high);

            (id, low, high)
        }

        (Node::Branch(id_1, low_1, high_1), Node::Branch(id_2, _, _)) if id_1 < id_2 => {
            let low = union(low_1.into(), node2);
            let high = Node::from(high_1);

            (id_1, low, high)
        }
        (Node::Branch(id_1, _, _), Node::Branch(id_2, low_2, high_2)) if id_1 > id_2 => {
            let low = union(node1, low_2.into());
            let high = Node::from(high_2);

            (id_2, low, high)
        }
        (Node::Branch(id_1, low_1, high_1), Node::Branch(_, low_2, high_2)) => {
            let low = union(low_1.into(), low_2.into());
            let high = union(high_1.into(), high_2.into());

            (id_1, low, high)
        }
    };

    Node::branch(id, low, high)
}

#[cfg(test)]
mod tests {
    use super::super::Forest;

    #[test]
    fn union_returns_identity_when_both_trees_are_empty() {
        let tree1 = Forest::<&str>::empty();
        let tree2 = Forest::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_unit_when_left_is_empty_right_is_unit() {
        let tree1 = Forest::<&str>::empty();
        let tree2 = Forest::unit(&["1", "2"]);

        assert_eq!(
            Forest::unit(&["1", "2"]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_many_when_left_is_empty_right_is_many() {
        let tree1 = Forest::<&str>::empty();
        let tree2 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["2", "3"]
            ]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_unit_when_left_is_unit_right_is_empty() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::<&str>::empty();

        assert_eq!(
            Forest::unit(&["1", "2"]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_identity_when_trees_are_equal_unit() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::unit(&["1", "2"]);

        assert_eq!(
            Forest::unit(&["1", "2"]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_many_when_trees_are_disjoint_units() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::unit(&["2", "3"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["2", "3"]
            ]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_many_when_left_is_unit_right_is_many() {
        let tree1 = Forest::unit(&["3", "4"]);
        let tree2 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["2", "3"],
                vec!["3", "4"],
            ]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_many_when_left_is_many_right_is_unit() {
        let tree1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let tree2 = Forest::unit(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["2", "3"],
                vec!["3", "4"],
            ]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_identity_when_trees_are_equal_many() {
        let tree1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let tree2 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["2", "3"]
            ]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_many_when_trees_are_disjoint_many() {
        let tree1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let tree2 = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "4"]
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["1", "3"],
                vec!["2", "3"],
                vec!["2", "4"]
            ]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_many_when_trees_are_have_commonality() {
        let tree1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"],
            vec!["3", "4"],
        ]);
        let tree2 = Forest::many(&[
            vec!["2", "3"],
            vec!["3", "4"],
            vec!["4", "5"],
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["2", "3"],
                vec!["3", "4"],
                vec!["4", "5"],
            ]),
            Forest::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_identity_when_left_is_unit_right_is_many() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["2", "3"]
            ]),
            Forest::union(tree1, tree2)
        );
    }
}
