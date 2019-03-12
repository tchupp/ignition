use std::hash::Hash;
use std::iter::FromIterator;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

mod union;
mod intersect;
mod subset;
mod product;

/// Forest is an immutable set of sets
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Forest<T: Hash + Eq> {
    Empty,
    Unit(Vec<T>),
    Many(HashSet<Vec<T>>),
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Into<Vec<Vec<T>>> for Forest<T> {
    fn into(self) -> Vec<Vec<T>> {
        match self {
            Forest::Empty => Vec::new(),
            Forest::Unit(set) => vec![set],
            Forest::Many(matrix) => matrix
                .into_iter()
                .collect(),
        }
    }
}

impl<'a, T: Hash + Eq + Clone + Ord + Sync + Send> Into<Vec<Vec<T>>> for &'a Forest<T> {
    fn into(self) -> Vec<Vec<T>> {
        match self {
            Forest::Empty => Vec::new(),
            Forest::Unit(set) => vec![set.to_vec()],
            Forest::Many(matrix) => matrix
                .into_iter()
                .cloned()
                .collect(),
        }
    }
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Forest<T> {
    pub fn empty() -> Self {
        Forest::Empty
    }

    pub fn unit(set: &[T]) -> Self {
        Forest::Unit(Self::filter_repeats(set))
    }

    pub fn many(matrix: &[Vec<T>]) -> Self {
        match matrix.len() {
            0 => Forest::empty(),
            1 => Forest::unit(&matrix[0]),
            _ => {
                let matrix = matrix.iter()
                    .cloned()
                    .map(|set| Self::filter_repeats(&set))
                    .unique()
                    .collect();

                Forest::Many(matrix)
            }
        }
    }

    pub fn unique(set: &[T]) -> Self {
        let matrix: Vec<Vec<T>> = set.iter()
            .cloned()
            .map(|element| vec![element])
            .collect();

        Forest::many(&matrix)
    }

    fn filter_repeats<B: FromIterator<T>>(set: &[T]) -> B {
        set.iter().cloned().sorted().unique().collect::<B>()
    }

    pub fn len(&self) -> usize {
        match self {
            Forest::Empty => 0,
            Forest::Unit(_) => 1,
            Forest::Many(matrix) => matrix.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Forest::Empty => true,
            _ => false
        }
    }

    pub fn occurrences(&self) -> Vec<(T, usize)> {
        match self {
            Forest::Empty => vec![],
            Forest::Unit(set) => set.iter()
                .map(|item| (item.clone(), 1))
                .collect(),
            Forest::Many(matrix) => {
                matrix.iter()
                    .flatten()
                    .fold(HashMap::new(), |mut occurrences, item| {
                        *occurrences.entry(item.clone()).or_insert(0usize) += 1;
                        occurrences
                    })
                    .into_iter()
                    .sorted_by(|(item1, _), (item2, _)| Ord::cmp(item1, item2))
                    .collect()
            }
        }
    }

    pub fn intersect(self, other: Self) -> Self {
        intersect::intersect(self, other)
    }

    pub fn union(self, other: Self) -> Self {
        union::union(self, other)
    }

    pub fn product(self, other: Self) -> Self {
        product::product(self, other)
    }

    pub fn subset(self, element: T) -> Self {
        subset::subset(self, element)
    }

    pub fn subset_not(self, element: T) -> Self {
        subset::subset_not(self, element)
    }

    pub fn subset_all(self, elements: &[T]) -> Self {
        subset::subset_all(self, elements)
    }

    pub fn subset_none(self, elements: &[T]) -> Self {
        subset::subset_none(self, elements)
    }
}

#[cfg(test)]
mod eq_forest_tests {
    use super::Forest;

    #[test]
    fn empty_forest() {
        let forest1: Forest<&str> = Forest::empty();
        let forest2: Forest<&str> = Forest::empty();

        assert_eq!(forest1, forest2);
    }

    #[test]
    fn unit_forest() {
        let forest1: Forest<&str> = Forest::unit(&["1", "2"]);
        let forest2: Forest<&str> = Forest::unit(&["2", "1"]);

        assert_eq!(forest1, forest2);
    }

    #[test]
    fn many_forest() {
        let forest1: Forest<&str> = Forest::many(&[vec!["1", "2"]]);
        let forest2: Forest<&str> = Forest::many(&[vec!["2", "1"]]);

        assert_eq!(forest1, forest2);
    }

    #[test]
    fn many_forest_with_none() {
        let forest1 = Forest::<&str>::many(&[]);
        let forest2 = Forest::<&str>::empty();

        assert_eq!(forest1, forest2);
    }

    #[test]
    fn many_forest_with_one() {
        let forest1 = Forest::many(&[vec!["1"]]);
        let forest2 = Forest::unit(&["1"]);

        assert_eq!(forest1, forest2);
    }
}

#[cfg(test)]
mod empty_forest_tests {
    use super::Forest;

    #[test]
    fn empty_forest_has_size_0() {
        let forest: Forest<&str> = Forest::empty();

        assert_eq!(0, forest.len());
    }

    #[test]
    fn empty_forest_is_empty() {
        let forest: Forest<&str> = Forest::empty();

        assert_eq!(true, forest.is_empty());
    }

    #[test]
    fn empty_forest_into() {
        let forest: Forest<&str> = Forest::empty();

        assert_eq!(
            Vec::<Vec<&str>>::new(),
            Into::<Vec<_>>::into(forest.clone())
        );
    }
}

#[cfg(test)]
mod unit_forest_tests {
    use super::Forest;

    #[test]
    fn unit_forest_has_size_1() {
        let forest: Forest<&str> = Forest::unit(&["1", "2"]);

        assert_eq!(1, forest.len());
    }

    #[test]
    fn unit_forest_is_empty() {
        let forest: Forest<&str> = Forest::unit(&["1", "2"]);

        assert_eq!(false, forest.is_empty());
    }

    #[test]
    fn unit_forest_into() {
        let forest: Forest<&str> = Forest::unit(&["1", "2"]);
        let expected = vec![vec!["1", "2"]];

        assert_eq!(
            expected,
            Into::<Vec<_>>::into(forest.clone())
        );
    }
}

#[cfg(test)]
mod many_forest_tests {
    use super::Forest;

    #[test]
    fn many_forest_has_size_2() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(2, forest.len());
    }

    #[test]
    fn many_forest_is_not_empty() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(false, forest.is_empty());
    }

    #[test]
    fn many_forest_into() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let expected = vec![
            vec!["1", "2"],
            vec!["2", "3"],
        ];

        assert_eq!(
            expected,
            Into::<Vec<_>>::into(forest.clone())
        );
    }

    #[test]
    fn unique_forest_into() {
        let forest: Forest<&str> = Forest::unique(&["1", "2"]);
        let expected = vec![
            vec!["2"],
            vec!["1"],
        ];

        assert_eq!(
            expected,
            Into::<Vec<_>>::into(forest.clone())
        );
    }
}

#[cfg(test)]
mod random_tests {
    use super::Forest;

    #[test]
    fn product_of_two_forests_of_two() {
        let forest = Forest::unique(&["1-1", "1-2", "1-3"])
            .product(Forest::unique(&["2-1", "2-2", "2-3"]));

        assert_eq!(9, forest.len());

        let expected = Forest::many(&[
            vec!["1-3", "2-1"],
            vec!["1-3", "2-2"],
            vec!["2-3", "1-2"],
            vec!["1-1", "2-2"],
            vec!["1-2", "2-2"],
            vec!["2-1", "1-2"],
            vec!["1-3", "2-3"],
            vec!["1-1", "2-1"],
            vec!["1-1", "2-3"],
        ]);

        assert_eq!(
            expected,
            forest
        );
    }

    #[test]
    fn product_of_three_forests_of_three() {
        let forest = Forest::unique(&["1-1", "1-2", "1-3"])
            .product(Forest::unique(&["2-1", "2-2", "2-3"]))
            .product(Forest::unique(&["3-1", "3-2", "3-3"]));

        assert_eq!(27, forest.len());

        let expected = Forest::many(&[
            vec!["1-1", "2-1", "3-1"],
            vec!["1-1", "2-1", "3-2"],
            vec!["1-1", "2-1", "3-3"],
            vec!["1-1", "2-2", "3-1"],
            vec!["1-1", "2-2", "3-2"],
            vec!["1-1", "2-2", "3-3"],
            vec!["1-1", "2-3", "3-1"],
            vec!["1-1", "2-3", "3-2"],
            vec!["1-1", "2-3", "3-3"],
            vec!["1-2", "2-1", "3-1"],
            vec!["1-2", "2-1", "3-2"],
            vec!["1-2", "2-1", "3-3"],
            vec!["1-2", "2-2", "3-1"],
            vec!["1-2", "2-2", "3-2"],
            vec!["1-2", "2-2", "3-3"],
            vec!["1-2", "2-3", "3-1"],
            vec!["1-2", "2-3", "3-2"],
            vec!["1-2", "2-3", "3-3"],
            vec!["1-3", "2-1", "3-1"],
            vec!["1-3", "2-1", "3-2"],
            vec!["1-3", "2-1", "3-3"],
            vec!["1-3", "2-2", "3-1"],
            vec!["1-3", "2-2", "3-2"],
            vec!["1-3", "2-2", "3-3"],
            vec!["1-3", "2-3", "3-1"],
            vec!["1-3", "2-3", "3-2"],
            vec!["1-3", "2-3", "3-3"],
        ]);

        assert_eq!(
            expected,
            forest
        );
    }
}
