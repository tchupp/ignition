use std::hash::Hash;

use types;

pub use self::forest::Forest;
pub use self::tree::Tree;

mod forest;
mod tree;

impl<T: Hash + Eq + Clone + Ord + Sync + Send> types::Forest<T> for Forest<T> {
    fn empty() -> Self {
        Forest::empty()
    }

    fn unit(set: &[T]) -> Self {
        Forest::unit(set)
    }

    fn many(matrix: &[Vec<T>]) -> Self {
        Forest::many(matrix)
    }

    fn unique(set: &[T]) -> Self {
        Forest::unique(set)
    }

    fn len(&self) -> usize {
        Forest::len(self)
    }

    fn is_empty(&self) -> bool {
        Forest::is_empty(self)
    }

    fn trees(&self) -> Vec<Vec<T>> {
        Into::<Vec<_>>::into(self)
    }

    fn intersect(self, other: Self) -> Self {
        Forest::intersect(self, other)
    }

    fn union(self, other: Self) -> Self {
        Forest::union(self, other)
    }

    fn product(self, other: Self) -> Self {
        Forest::product(self, other)
    }

    fn subset(self, element: T) -> Self {
        Forest::subset(self, element)
    }

    fn subset_all(self, elements: &[T]) -> Self {
        Forest::subset_all(self, elements)
    }
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> types::Tree<T> for Tree<T> {
    type Forest = Forest<T>;

    fn empty() -> Self {
        Tree::empty()
    }

    fn one(element: T) -> Self {
        Tree::one(element)
    }

    fn many(elements: &[T]) -> Self {
        Tree::many(elements)
    }

    fn len(&self) -> usize {
        Tree::len(self)
    }

    fn is_empty(&self) -> bool {
        Tree::is_empty(self)
    }

    fn intersect(self, other: Self) -> Self {
        Tree::intersect(self, other)
    }

    fn union(self, other: Self) -> Self {
        Tree::union(self, other)
    }

    fn product(self, other: Self) -> Forest<T> {
        Tree::product(self, other)
    }
}
