use std::hash::Hash;

use hashbrown::HashSet;

use matrix::Forest;

pub fn product<T: Hash + Eq + Clone + Ord + Sync + Send>(forest1: Forest<T>, forest2: Forest<T>) -> Forest<T> {
    match (forest1, forest2) {
        (_, Forest::Empty) => Forest::empty(),
        (Forest::Empty, _) => Forest::empty(),

        (Forest::Unit(mut set1), Forest::Unit(set2)) => {
            set1.extend(set2);
            Forest::unit(&set1)
        }

        (Forest::Many(matrix), Forest::Unit(set)) => many_to_matrix(matrix, set),
        (Forest::Unit(set), Forest::Many(matrix)) => many_to_matrix(matrix, set),

        (Forest::Many(matrix1), Forest::Many(matrix2)) => {
            let matrix: Vec<Vec<T>> = matrix1.into_iter()
                .flat_map(|set1| {
                    matrix2.iter()
                        .map(|set2| {
                            let mut new_set = set1.clone();
                            new_set.extend_from_slice(set2);
                            new_set
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            Forest::many(&matrix)
        }
    }
}

fn many_to_matrix<T: Hash + Clone + Ord + Sync + Send>(matrix: HashSet<Vec<T>>, set2: Vec<T>) -> Forest<T> {
    let matrix: Vec<Vec<T>> = matrix.into_iter()
        .map(|mut set1| {
            set1.extend_from_slice(&set2);
            set1
        })
        .collect();

    Forest::many(&matrix)
}

#[cfg(test)]
mod tests {
    use super::Forest;

    #[test]
    fn empty_right_returns_empty() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn empty_left_returns_empty() {
        let forest1 = Forest::<&str>::empty();
        let forest2 = Forest::unit(&["1"]);

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn unit_forests_with_overlap_returns_many() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::unit(&["1"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn disjoint_unit_forests_returns_many_1() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::unit(&["3"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn disjoint_unit_forests_returns_unit() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::unit(&["3", "4"]);

        assert_eq!(
            Forest::unit(&["1", "2", "3", "4"]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_single_unit_forest_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = Forest::unit(&["4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "4"],
                vec!["2", "3", "4"]
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_double_unit_forest_with_overlap_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = Forest::unit(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3", "4"],
                vec!["2", "3", "4"]
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_unique_forest_with_overlap_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = Forest::unique(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
                vec!["2", "3"],
                vec!["2", "3", "4"]
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_unique_forest_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["5", "6"]
        ]);
        let forest2 = Forest::unique(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
                vec!["5", "6", "3"],
                vec!["5", "6", "4"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_double_unit_forest_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["5", "6"]
        ]);
        let forest2 = Forest::unit(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3", "4"],
                vec!["5", "6", "3", "4"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forests_returns_many_1() {
        let forest1 = Forest::unique(&["1", "2"]);
        let forest2 = Forest::unique(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "3"],
                vec!["1", "4"],
                vec!["2", "3"],
                vec!["2", "4"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forests_returns_many_2() {
        let forest1 = Forest::unique(&["1", "2"]);
        let forest2 = Forest::many(&[
            vec!["3", "4"],
            vec!["7", "8"]
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "3", "4"],
                vec!["1", "7", "8"],
                vec!["2", "3", "4"],
                vec!["2", "7", "8"],
            ]),
            Forest::product(forest1, forest2)
        );
    }
}