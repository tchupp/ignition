pub use self::universe::*;
use std::collections::BTreeSet;
use std::fmt;
use std::hash::Hash;
use zdd::node::Node;
use zdd::node::NodeId;

mod combinations;
mod intersect;
mod union;
mod universe;

#[derive(Clone, Eq, PartialEq)]
pub struct Tree<T> {
    root: NodeId,
    universe: Universe<T>,
}

impl<T> fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

impl<T: Clone + Ord + Hash> Tree<T> {
    fn empty(universe: Universe<T>) -> Tree<T> {
        Tree { root: Node::FALSE, universe }
    }

    fn unit(universe: Universe<T>) -> Tree<T> {
        Tree { root: Node::TRUE, universe }
    }

    fn from_root<R>(universe: Universe<T>, root: R) -> Tree<T> where R: Into<NodeId> {
        Tree { root: root.into(), universe }
    }

    pub fn combinations(&self) -> BTreeSet<BTreeSet<T>> {
        combinations::combinations(self.root)
            .into_iter()
            .map(|set| self.universe.get_items(&set))
            .collect::<BTreeSet<_>>()
    }

    pub fn onset(&self, inclusions: &BTreeSet<T>) -> BTreeSet<BTreeSet<T>> {
        combinations::combinations(self.root)
            .into_iter()
            .map(|set| self.universe.get_items(&set))
            .filter(|set| set.intersection(inclusions).cloned().collect::<BTreeSet<_>>() == *inclusions)
            .collect::<BTreeSet<_>>()
    }

    pub fn offset(&self, exclusions: &BTreeSet<T>) -> BTreeSet<BTreeSet<T>> {
        combinations::combinations(self.root)
            .into_iter()
            .map(|set| self.universe.get_items(&set))
            .filter(|set| set.intersection(exclusions).collect::<BTreeSet<_>>().len() == 0)
            .collect::<BTreeSet<_>>()
    }

    pub fn union(&self, other: &Tree<T>) -> Tree<T> {
        let root = union::union(
            self.root.into(),
            other.root.into());

        Tree::from_root(self.universe.clone(), root)
    }

    pub fn intersect(&self, other: &Tree<T>) -> Tree<T> {
        let root = intersect::intersect(
            self.root.into(),
            other.root.into());

        Tree::from_root(self.universe.clone(), root)
    }
}

#[cfg(test)]
mod tests {
    use core::Item;
    use super::Universe;

    #[test]
    fn universe_can_create_empty_tree() {
        let universe: Universe<Item> = Universe::default();
        let empty_tree = universe.empty_tree();

        assert_eq!(
            btreeset!(),
            empty_tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_unit_tree() {
        let universe: Universe<Item> = Universe::default();
        let unit_tree = universe.unit_tree();

        assert_eq!(
            btreeset!(btreeset!()),
            unit_tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_none() {
        let universe: Universe<Item> = Universe::default();
        let tree = universe.tree(&[]);

        assert_eq!(
            universe.unit_tree(),
            tree
        );
        assert_eq!(
            btreeset!(btreeset!()),
            tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_one() {
        let item = Item::new("1");

        let universe = Universe::from(vec![item.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item.clone())),
            universe.tree(&[item]).combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_multiple() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.tree(&[item1.clone(), item2.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone())),
            tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_an_empty_set_unknown_items_are_ignored() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe: Universe<Item> = Universe::default();
        let tree = universe.tree(&[item1, item2]);

        assert_eq!(
            universe.unit_tree(),
            tree.clone()
        );
        assert_eq!(
            btreeset!(btreeset!()),
            tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_unknown_items_are_ignored() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone()]);
        let tree = universe.tree(&[item1.clone(), item2.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        )
    }
}
