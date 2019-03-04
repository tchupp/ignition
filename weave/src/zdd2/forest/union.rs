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
