use std::hash::Hash;

use hashbrown::HashSet;

use matrix::Forest;

mod intersect;
mod product;
mod union;

/// Tree is an immutable set of elements
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Tree<T: Hash + Eq + Clone + Ord> {
    Empty,
    One(T),
    Many(HashSet<T>),
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Default for Tree<T> {
    fn default() -> Self {
        Tree::empty()
    }
}

impl<T: Hash + Eq + Clone + Ord> Into<Vec<T>> for Tree<T> {
    fn into(self) -> Vec<T> {
        match self {
            Tree::Empty => Vec::new(),
            Tree::One(element) => vec![element],
            Tree::Many(set) => set.into_iter().collect(),
        }
    }
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Tree<T> {
    pub fn empty() -> Self {
        Tree::Empty
    }

    pub fn one(element: T) -> Self {
        Tree::One(element)
    }

    pub fn many(elements: &[T]) -> Self {
        match elements.len() {
            0 => Tree::Empty,
            1 => Tree::One(elements[0].clone()),
            _ => {
                let mut set = HashSet::with_capacity(elements.len());
                elements
                    .iter()
                    .cloned()
                    .for_each(|e| { set.insert(e); });

                Tree::Many(set)
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Tree::Empty => 0,
            Tree::One(_) => 1,
            Tree::Many(set) => set.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Tree::Empty => true,
            _ => false
        }
    }

    pub fn intersect(self, other: Self) -> Self {
        intersect::intersect(self, other)
    }

    pub fn union(self, other: Self) -> Self {
        union::union(self, other)
    }

    pub fn product(self, other: Self) -> Forest<T> {
        product::product(self, other)
    }
}

#[cfg(test)]
mod eq_tree_tests {
    use super::Tree;

    #[test]
    fn empty_tree() {
        let tree1: Tree<&str> = Tree::default();
        let tree2: Tree<&str> = Tree::default();

        assert_eq!(tree1, tree2);
    }

    #[test]
    fn single_element() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::one("1");

        assert_eq!(tree1, tree2);
    }

    #[test]
    fn many_element() {
        let tree1 = Tree::many(&["1", "2"]);
        let tree2 = Tree::many(&["1", "2"]);

        assert_eq!(tree1, tree2);
    }

    #[test]
    fn many_element_with_none() {
        let tree1 = Tree::<&str>::many(&[]);
        let tree2 = Tree::<&str>::default();

        assert_eq!(tree1, tree2);
    }

    #[test]
    fn many_element_with_one() {
        let tree1 = Tree::many(&["1"]);
        let tree2 = Tree::one("1");

        assert_eq!(tree1, tree2);
    }
}

#[cfg(test)]
mod empty_tree_tests {
    use super::Tree;

    #[test]
    fn empty_tree_is_default() {
        let tree: Tree<&str> = Tree::default();

        assert_eq!(0, tree.len());
    }

    #[test]
    fn empty_tree_has_size_0() {
        let tree: Tree<&str> = Tree::empty();

        assert_eq!(0, tree.len());
    }

    #[test]
    fn empty_tree_is_empty() {
        let tree: Tree<&str> = Tree::empty();

        assert_eq!(true, tree.is_empty());
    }

    #[test]
    fn empty_tree_into() {
        let tree: Tree<&str> = Tree::empty();

        assert_eq!(Vec::<&str>::new(), Into::<Vec<_>>::into(tree.clone()));
    }
}

#[cfg(test)]
mod one_element_tree_tests {
    use super::Tree;

    #[test]
    fn one_element_tree_has_size_1() {
        let tree = Tree::one("1");

        assert_eq!(1, tree.len());
    }

    #[test]
    fn one_element_tree_is_not_empty() {
        let tree = Tree::one("1");

        assert_eq!(false, tree.is_empty());
    }

    #[test]
    fn one_element_tree_into() {
        let tree = Tree::one("1");

        assert_eq!(vec!["1"], Into::<Vec<_>>::into(tree.clone()));
    }
}

#[cfg(test)]
mod many_element_tree_tests {
    use super::Tree;

    #[test]
    fn many_element_tree_has_size_n() {
        let tree = Tree::many(&["1", "2"]);

        assert_eq!(2, tree.len());
    }

    #[test]
    fn many_element_tree_is_not_empty() {
        let tree = Tree::many(&["1", "2"]);

        assert_eq!(false, tree.is_empty());
    }

    #[test]
    fn many_element_tree_into() {
        let tree = Tree::many(&["1", "2"]);

        assert_eq!(vec!["1", "2"], Into::<Vec<_>>::into(tree.clone()));
    }
}
