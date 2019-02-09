use std::hash::Hash;

use zdd2::Forest;

pub fn union<T: Hash + Eq + Clone + Ord + Sync + Send>(forest1: Forest<T>, forest2: Forest<T>) -> Forest<T> {
    if forest1 == forest2 {
        return forest1;
    }

    match (&forest1, &forest2) {
        (_, Forest::Empty) => forest1.clone(),
        (Forest::Empty, _) => forest2.clone(),

        (Forest::Unit(set1), Forest::Unit(set2)) =>
            Forest::many(&[set1.clone(), set2.clone()]),

        (Forest::Many(matrix), Forest::Unit(set)) => Forest::Many(matrix.extend(&set)),
        (Forest::Unit(set), Forest::Many(matrix)) => Forest::Many(matrix.extend(&set)),

        (Forest::Many(matrix1), Forest::Many(matrix2)) =>
            Forest::Many(matrix1.union(matrix2)),
    }
}

#[cfg(test)]
mod tests {
    use zdd2::Forest;

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
}