use std::fmt;
use std::hash::Hash;

use itertools::Itertools;

use self::node::Node;
use self::node::NodeId;
use self::node::Priority;
use self::universe::Universe;

#[macro_use]
mod node;
mod universe;
mod trees;

#[cfg(test)]
mod union;
#[cfg(test)]
mod intersect;
mod subset;
#[cfg(test)]
mod product;

/// Forest is an immutable set of sets
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Forest<T: Hash + Eq + Clone + Ord> {
    root: NodeId,
    universe: Universe<T>,
}

impl<T: Hash + Eq + Clone + Ord + fmt::Debug> fmt::Debug for Forest<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.fmt_inner(self.root.into(), 1))
    }
}

impl<T: Hash + Eq + Clone + Ord + fmt::Debug> Forest<T> {
    fn fmt_inner(&self, root: Node, indent: usize) -> String {
        match root {
            Node::Branch(id, low, high) =>
                format!(
                    "{} - {:?}: {:?}\n{}{}\n{}{}",
                    NodeId::from(root),
                    id,
                    self.universe.get_item(id).unwrap(),
                    "| ".repeat(indent),
                    self.fmt_inner(Node::from(low), indent + 1),
                    "| ".repeat(indent),
                    self.fmt_inner(Node::from(high), indent + 1)
                ),
            Node::Always => String::from("Always"),
            Node::Never => String::from("Never"),
        }
    }
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> Forest<T> {
    pub fn empty() -> Self {
        let universe = Universe::default();
        let root = Node::NEVER;

        Forest { root, universe }
    }

    pub fn unit(items: &[T]) -> Self {
        let universe = Universe::from_items(items);
        let root = universe.get_priorities::<Node>(items);

        Forest { root: root.into(), universe }
    }

    pub fn many(matrix: &[Vec<T>]) -> Self {
        let universe = Universe::from_matrix(matrix);

        let root = matrix.iter()
            .map(|items| universe.get_priorities::<Node>(items))
            .fold(Node::Never, Node::union);

        Forest { root: root.into(), universe }
    }

    pub fn unique(set: &[T]) -> Self {
        let universe = Universe::from_items(set);

        let root = universe.get_priorities::<Vec<_>>(set)
            .into_iter()
            .fold(Node::Never, |root, item| Node::branch(item, root, Node::Always));

        Forest { root: root.into(), universe }
    }

    fn canonical(root: impl Into<NodeId>, universe: Universe<T>) -> Self {
        let trees = trees::trees(root.into())
            .into_iter()
            .map(|set| universe.get_items::<Vec<_>>(&set))
            .collect::<Vec<_>>();

        Self::many(&trees)
    }

    pub fn len(&self) -> usize {
        trees::trees(self.root).len()
    }

    pub fn is_empty(&self) -> bool {
        trees::trees(self.root).is_empty()
    }

    pub fn trees(&self) -> Vec<Vec<T>> {
        trees::trees(self.root)
            .into_iter()
            .map(|set| self.universe.get_items::<Vec<_>>(&set))
            .collect()
    }

    pub fn occurrences(&self) -> Vec<(T, usize)> {
        self.universe.occurrences()
            .clone()
            .into_iter()
            .sorted_by(|(item1, _), (item2, _)| Ord::cmp(item1, item2))
            .collect()
    }

    pub fn intersect(self, other: Self) -> Self {
        let (universe, self_root, other_root) = translate_roots(
            (&self.universe, self.root.into()),
            (&other.universe, other.root.into()),
        );
        let root = Node::intersect(self_root, other_root);

        Self::canonical(root, universe)
    }

    pub fn union(self, other: Self) -> Self {
        let (universe, self_root, other_root) = translate_roots(
            (&self.universe, self.root.into()),
            (&other.universe, other.root.into()),
        );
        let root = Node::union(self_root, other_root);

        Self::canonical(root, universe)
    }

    pub fn product(self, other: Self) -> Self {
        let (universe, self_root, other_root) = translate_roots(
            (&self.universe, self.root.into()),
            (&other.universe, other.root.into()),
        );
        let root = Node::product(self_root, other_root);

        Self::canonical(root, universe)
    }

    pub fn subset(self, element: T) -> Self {
        subset::subset(self, element)
    }

    pub fn subset_not(self, element: T) -> Self {
        subset::subset_not(self, element)
    }

    pub fn subset_all(self, elements: &[T]) -> Self {
        subset::subset_many(
            self,
            elements,
            &|_| Forest::empty(),
            Node::subset_all,
        )
    }

    pub fn subset_none(self, elements: &[T]) -> Self {
        subset::subset_many(
            self,
            elements,
            &|forest| forest,
            Node::subset_none,
        )
    }
}

fn translate_roots<T: Hash + Eq + Clone + Ord>((self_universe, self_root): (&Universe<T>, Node), (other_universe, other_root): (&Universe<T>, Node)) -> (Universe<T>, Node, Node) {
    let universe = Universe::merge(self_universe, other_universe);

    let self_root = translate_root(self_universe, &universe, self_root);
    let other_root = translate_root(other_universe, &universe, other_root);

    (universe, self_root, other_root)
}

fn translate_root<T: Hash + Eq + Clone + Ord>(old_universe: &Universe<T>, new_universe: &Universe<T>, root: Node) -> Node {
    match root {
        Node::Branch(id, low, high) => {
            let low = translate_root(old_universe, new_universe, low.into());
            let high = translate_root(old_universe, new_universe, high.into());

            let item = old_universe.get_item(id).unwrap();
            let id = new_universe.get_priority(item).unwrap();

            Node::branch(id, low, high)
        }
        _ => root
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
    fn unit_forest_with_none() {
        let forest1 = Forest::<&str>::unit(&[]);
        let forest2 = Forest::<&str>::empty();

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
            forest.trees()
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
        let expected: Vec<Vec<&str>> = vec![vec!["1", "2"]];

        assert_eq!(
            expected,
            forest.trees()
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
        let expected: Vec<Vec<&str>> = vec![
            vec!["1", "2"],
            vec!["2", "3"],
        ];

        assert_eq!(
            expected,
            forest.trees()
        );
    }

    #[test]
    fn unique_forest_into() {
        let forest: Forest<&str> = Forest::unique(&["1", "2"]);
        let expected: Vec<Vec<&str>> = vec![
            vec!["1"],
            vec!["2"],
        ];

        assert_eq!(
            expected,
            forest.trees()
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
