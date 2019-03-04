use std::hash::Hash;

use super::Forest;

pub fn intersect<T: Hash + Eq + Clone + Ord + Sync + Send>(forest1: Forest<T>, forest2: Forest<T>) -> Forest<T> {
    if forest1 == forest2 {
        return forest1;
    }

    match (&forest1, &forest2) {
        (Forest::Many(matrix1), Forest::Many(matrix2)) =>
            Forest::from_root(matrix1.intersect(matrix2)),
    }
}

#[cfg(test)]
mod tests {
    use super::Forest;

    #[test]
    fn intersect_returns_identity_when_both_trees_are_empty() {
        let tree1 = Forest::<&str>::empty();
        let tree2 = Forest::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_empty_when_left_is_empty_right_is_unit() {
        let tree1 = Forest::<&str>::empty();
        let tree2 = Forest::unit(&["1", "2"]);

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_empty_when_left_is_empty_right_is_many() {
        let tree1 = Forest::<&str>::empty();
        let tree2 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_empty_when_left_is_unit_right_is_empty() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_identity_when_trees_are_equal_unit() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::unit(&["1", "2"]);

        assert_eq!(
            Forest::unit(&["1", "2"]),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_empty_when_trees_are_disjoint_units() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::unit(&["2", "3"]);

        assert_eq!(
            Forest::empty(),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_unit_when_left_is_unit_right_is_many() {
        let tree1 = Forest::unit(&["1", "2"]);
        let tree2 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(
            Forest::unit(&["1", "2"]),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_identity_when_trees_are_equal_many() {
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
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_empty_when_trees_are_disjoint_many() {
        let tree1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let tree2 = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "4"]
        ]);

        assert_eq!(
            Forest::empty(),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_one_when_trees_are_have_commonality() {
        let tree1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"],
        ]);
        let tree2 = Forest::many(&[
            vec!["2", "3"],
            vec!["3", "4"],
            vec!["4", "5"],
        ]);

        assert_eq!(
            Forest::unit(&["2", "3"]),
            Forest::intersect(tree1, tree2)
        );
    }

    #[test]
    fn intersect_returns_many_when_trees_are_have_commonality() {
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
                vec!["2", "3"],
                vec!["3", "4"],
            ]),
            Forest::intersect(tree1, tree2)
        );
    }
}