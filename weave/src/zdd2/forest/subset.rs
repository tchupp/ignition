use std::hash::Hash;

use super::Forest;

pub fn subset<T: Hash + Eq + Clone + Ord + Sync + Send>(forest: Forest<T>, element: T) -> Forest<T> {
    match forest {
        Forest(matrix) =>
            Forest::from_root(matrix.subset(element)),
    }
}

pub fn subset_many<T: Hash + Eq + Clone + Ord + Sync + Send>(forest: Forest<T>, elements: &[T]) -> Forest<T> {
    if elements.is_empty() {
        return forest;
    }

    match forest {
        Forest(matrix) =>
            Forest::from_root(matrix.subset_many(elements)),
    }
}

#[cfg(test)]
mod subset_tests {
    use super::Forest;

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
    fn subset_of_disjoint_unit_returns_empty() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let element = "1";

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_disjoint_many_returns_empty() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"]
        ]);
        let element = "4";

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
    fn subset_of_containing_returns_unit_1() {
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

    #[test]
    fn subset_of_containing_returns_unit_2() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"]
        ]);
        let element = "2";

        assert_eq!(
            Forest::unit(&["2", "3"]),
            Forest::subset(forest, element)
        );
    }
}

#[cfg(test)]
mod subset_many_tests {
    use super::Forest;

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
