use std::hash::Hash;
use std::iter::FromIterator;

use hashbrown::HashSet;
use itertools::Itertools;

use zdd2::Tree;

mod union;
mod intersect;

/// Forest is an immutable set of sets
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Forest<T: Hash + Eq> {
    Empty,
    Unit(Vec<T>),
    Many(HashSet<Vec<T>>),
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Into<Vec<Tree<T>>> for Forest<T> {
    fn into(self) -> Vec<Tree<T>> {
        match self {
            Forest::Empty => Vec::new(),
            Forest::Unit(set) => vec![Tree::many(&set)],
            Forest::Many(matrix) => matrix
                .into_iter()
                .map(|s| Tree::many(&s))
                .collect(),
        }
    }
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Forest<T> {
    pub fn empty() -> Self {
        Forest::Empty
    }

    pub fn unit(set: &[T]) -> Self {
        Forest::Unit(Self::unique(set))
    }

    pub fn many(matrix: &[Vec<T>]) -> Self {
        match matrix.len() {
            0 => Forest::empty(),
            1 => Forest::unit(&matrix[0]),
            _ => {
                let matrix = matrix.iter()
                    .cloned()
                    .map(|set| Self::unique(&set))
                    .collect();

                Forest::Many(matrix)
            }
        }
    }

    fn unique<B: FromIterator<T>>(set: &[T]) -> B {
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

    pub fn intersect(self, other: Self) -> Self {
        intersect::intersect(self, other)
    }

    pub fn union(self, other: Self) -> Self {
        union::union(self, other)
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
        let forest2: Forest<&str> = Forest::unit(&["1", "2"]);

        assert_eq!(forest1, forest2);
    }

    #[test]
    fn many_forest() {
        let forest1: Forest<&str> = Forest::many(&[vec!["1", "2"]]);
        let forest2: Forest<&str> = Forest::many(&[vec!["1", "2"]]);

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
    use zdd2::Forest;
    use zdd2::Tree;

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
            Vec::<Tree<&str>>::new(),
            Into::<Vec<_>>::into(forest.clone())
        );
    }
}

#[cfg(test)]
mod unit_forest_tests {
    use zdd2::Forest;
    use zdd2::Tree;

    #[test]
    fn unit_forest_has_size_1() {
        let forest: Forest<&str> = Forest::unit(&["1", "2"]);
        ;

        assert_eq!(1, forest.len());
    }

    #[test]
    fn unit_forest_is_empty() {
        let forest: Forest<&str> = Forest::unit(&["1", "2"]);
        ;

        assert_eq!(false, forest.is_empty());
    }

    #[test]
    fn unit_forest_into() {
        let forest: Forest<&str> = Forest::unit(&["1", "2"]);
        let expected: Vec<Tree<&str>> = vec![Tree::many(&["1", "2"])];

        assert_eq!(
            expected,
            Into::<Vec<_>>::into(forest.clone())
        );
    }
}

#[cfg(test)]
mod many_forest_tests {
    use zdd2::Forest;
    use zdd2::Tree;

    #[test]
    fn many_forest_has_size_1() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_eq!(2, forest.len());
    }

    #[test]
    fn many_forest_is_empty() {
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
        let expected: Vec<Tree<&str>> = vec![
            Tree::many(&["1", "2"]),
            Tree::many(&["2", "3"]),
        ];

        assert_eq!(
            expected,
            Into::<Vec<_>>::into(forest.clone())
        );
    }
}
