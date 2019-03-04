use std::hash::Hash;

use super::Tree;

use self::root::ForestRoot;

mod node;
mod root;

mod union;
mod intersect;
mod subset;
mod product;

/// Forest is an immutable set of sets
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Forest<T: Hash + Eq + Clone + Ord>(ForestRoot<T>);

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Into<Vec<Tree<T>>> for Forest<T> {
    fn into(self) -> Vec<Tree<T>> {
        match self {
            Forest(matrix) => matrix.trees()
                .into_iter()
                .map(|set| Tree::many(&set))
                .collect(),
        }
    }
}

impl<'a, T: Hash + Eq + Clone + Ord + Sync + Send> Into<Vec<Tree<T>>> for &'a Forest<T> {
    fn into(self) -> Vec<Tree<T>> {
        match self {
            Forest(matrix) => matrix.trees()
                .into_iter()
                .map(|set| Tree::many(&set))
                .collect(),
        }
    }
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Forest<T> {
    pub fn empty() -> Self {
        Forest(ForestRoot::empty())
    }

    pub fn unit(set: &[T]) -> Self {
        Forest(ForestRoot::unit(set))
    }

    pub fn many(matrix: &[Vec<T>]) -> Self {
        Forest(ForestRoot::many(&matrix))
    }

    pub fn from_root(root: ForestRoot<T>) -> Self {
        Forest(root)
    }

    pub fn unique(set: &[T]) -> Self {
        Forest(ForestRoot::unique(set))
    }

    pub fn len(&self) -> usize {
        match self {
            Forest(matrix) => matrix.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Forest(matrix) => matrix.is_empty(),
        }
    }

    pub fn trees(&self) -> Vec<Tree<T>> {
        Into::<_>::into(self)
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

    pub fn subset_many(self, elements: &[T]) -> Self {
        subset::subset_many(self, elements)
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
    use super::Tree;

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
    use super::Forest;
    use super::Tree;

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
        let expected: Vec<Tree<&str>> = vec![Tree::many(&["1", "2"])];

        assert_eq!(
            expected,
            Into::<Vec<_>>::into(forest.clone())
        );
    }
}

#[cfg(test)]
mod many_forest_tests {
    use super::Forest;
    use super::Tree;

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
        let expected: Vec<Tree<&str>> = vec![
            Tree::many(&["1", "2"]),
            Tree::many(&["2", "3"]),
        ];

        assert_eq!(
            expected,
            Into::<Vec<_>>::into(forest.clone())
        );
    }

    #[test]
    fn unique_forest_into() {
        let forest: Forest<&str> = Forest::unique(&["1", "2"]);
        let expected: Vec<Tree<&str>> = vec![
            Tree::many(&["1"]),
            Tree::many(&["2"]),
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
