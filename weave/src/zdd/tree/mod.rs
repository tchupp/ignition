use core::Item;
pub use self::universe::*;
use std::collections::BTreeSet;
use std::fmt;
use zdd::node::Node;
use zdd::node::NodeId;

mod combinations;
mod universe;

#[derive(Clone, Eq, PartialEq)]
pub struct Tree {
    root: NodeId,
    universe: Universe,
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

impl Tree {
    fn empty(universe: Universe) -> Tree {
        Tree { root: Node::FALSE, universe }
    }

    fn unit(universe: Universe) -> Tree {
        Tree { root: Node::TRUE, universe }
    }

    fn from_root<R>(universe: Universe, root: R) -> Tree where R: Into<NodeId> {
        Tree { root: root.into(), universe }
    }

    pub fn combinations(&self) -> BTreeSet<BTreeSet<Item>> {
        combinations::combinations(self)
    }
}

#[cfg(test)]
mod tests {
    use core::Item;
    use super::Universe;

    #[test]
    fn universe_can_create_empty_tree() {
        let universe = Universe::default();
        let empty_tree = universe.empty_tree();

        assert_eq!(
            btreeset!(),
            empty_tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_unit_tree() {
        let universe = Universe::default();
        let unit_tree = universe.unit_tree();

        assert_eq!(
            btreeset!(btreeset!()),
            unit_tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_none() {
        let universe = Universe::default();
        let tree = universe.tree(vec![]);

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
            universe.tree(vec![item]).combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_multiple() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.tree(vec![item1.clone(), item2.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone())),
            tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_an_empty_set_unknown_items_are_ignored() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::default();
        let tree = universe.tree(vec![item1, item2]);

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
        let tree = universe.tree(vec![item1.clone(), item2.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        )
    }
}
