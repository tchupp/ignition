/// Forest is an immutable set of sets
pub trait Forest<T: Clone> {
    type Tree: Tree<T>;

    fn empty() -> Self;
    fn unit(set: &[T]) -> Self;
    fn many(matrix: &[Vec<T>]) -> Self;
    fn unique(set: &[T]) -> Self;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn trees(&self) -> Vec<Self::Tree>;

    fn intersect(self, other: Self) -> Self;
    fn union(self, other: Self) -> Self;
    fn product(self, other: Self) -> Self;

    fn subset(self, element: T) -> Self;
    fn subset_all(self, elements: &[T]) -> Self;
}

/// Tree is an immutable set of elements
pub trait Tree<T: Clone>: Sized {
    type Forest: Forest<T, Tree=Self>;

    fn empty() -> Self;
    fn one(element: T) -> Self;
    fn many(elements: &[T]) -> Self;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn intersect(self, other: Self) -> Self;
    fn union(self, other: Self) -> Self;
    fn product(self, other: Self) -> Self::Forest;
}
