use std::hash::Hash;

use hashbrown::HashSet;

use zdd2::Forest;

pub fn subset<T: Hash + Eq + Clone + Ord + Sync + Send>(forest: Forest<T>, element: T) -> Forest<T> {
    match &forest {
        Forest::Unit(set) if set.contains(&element) => forest.clone(),
        Forest::Many(matrix) => {
            let forest: Vec<Vec<T>> = matrix.into_iter()
                .cloned()
                .filter(|set| set.contains(&element))
                .collect();

            Forest::many(&forest)
        }
        _ => Forest::empty(),
    }
}

pub fn subset_many<T: Hash + Eq + Clone + Ord + Sync + Send>(forest: Forest<T>, elements: &[T]) -> Forest<T> {
    if elements.is_empty() {
        return forest;
    }

    match &forest {
        Forest::Unit(set) =>
            if elements.into_iter().all(|e| set.contains(e)) {
                forest.clone()
            } else {
                Forest::empty()
            },
        Forest::Many(matrix) => {
            let forest: Vec<Vec<T>> = matrix.into_iter()
                .cloned()
                .filter(|set| elements.into_iter().all(|e| set.contains(e)))
                .collect();

            Forest::many(&forest)
        }
        _ => Forest::empty()
    }
}

#[cfg(test)]
mod subset_tests {
    use zdd2::Forest;

    #[test]
    fn subset_of_empty_returns_empty() {
        let forest: Forest<&str> = Forest::empty();
        let element = "1";

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_disjoint_returns_empty() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let element = "1";

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_containing_returns_identity() {
        let forest: Forest<&str> = Forest::unit(&["1", "3"]);
        let element = "1";

        assert_eq!(
            Forest::unit(&["1", "3"]),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_containing_returns_unit() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"]
        ]);
        let element = "1";

        assert_eq!(
            Forest::unit(&["1", "3"]),
            Forest::subset(forest, element)
        );
    }
}

#[cfg(test)]
mod subset_many_tests {
    use zdd2::Forest;

    #[test]
    fn subset_many_of_empty_returns_empty() {
        let forest: Forest<&str> = Forest::empty();
        let elements = &["1"];

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_empty_elements_returns_identity() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let elements = &[];

        assert_eq!(
            Forest::unit(&["2", "3"]),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_one_element_returns_identity() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let elements = &["2"];

        assert_eq!(
            Forest::unit(&["2", "3"]),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_disjoint_elements_returns_empty() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let elements = &["1", "2"];

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_one_element_returns_many() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"],
            vec!["1", "2"]
        ]);
        let elements = &["3"];

        assert_eq!(
            Forest::many(&[
                vec!["1", "3"],
                vec!["2", "3"],
            ]),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_many_elements_returns_unit() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"],
            vec!["1", "2"]
        ]);
        let elements = &["1", "3"];

        assert_eq!(
            Forest::unit(&["1", "3"]),
            Forest::subset_many(forest, elements)
        );
    }
}
