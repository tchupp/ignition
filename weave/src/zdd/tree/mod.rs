use core::Item;
pub use self::universe::*;
use std::fmt;
use zdd::node::Node;
use zdd::node::NodeId;

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

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn is_unit(&self) -> bool {
        self.root.is_unit()
    }
}

#[cfg(test)]
mod tests {
    use core::Item;
    use super::Universe;
    use zdd::node::Node;
    use zdd::node::NodeId;

    #[test]
    fn universe_can_create_empty_tree() {
        let universe = Universe::default();

        assert_eq!(
            Node::FALSE,
            universe.empty_tree().root
        );

        assert!(
            universe.empty_tree().is_empty()
        );
    }

    #[test]
    fn universe_can_create_unit_tree() {
        let universe = Universe::default();

        assert_eq!(
            Node::TRUE,
            universe.unit_tree().root
        );
        assert!(
            universe.unit_tree().is_unit()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_none() {
        let universe = Universe::default();

        assert_eq!(
            universe.unit_tree(),
            universe.tree(vec![])
        )
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_one() {
        let item = Item::new("1");

        let universe = Universe::from(vec![item.clone()]);

        assert_eq!(
            NodeId::from(Node::required_branch(0, Node::TRUE)),
            universe.tree(vec![item]).root
        )
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_multiple() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let expected = Node::required_branch(
            0,
            Node::required_branch(1, Node::TRUE));

        assert_eq!(
            NodeId::from(expected),
            universe.tree(vec![item1, item2]).root
        )
    }

    #[test]
    fn universe_can_create_tree_that_represents_an_empty_set_unknown_items_are_ignored() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::default();

        assert_eq!(
            universe.unit_tree(),
            universe.tree(vec![item1, item2])
        )
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_unknown_items_are_ignored() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone()]);
        let expected = Node::required_branch(0, Node::TRUE);

        assert_eq!(
            NodeId::from(expected),
            universe.tree(vec![item1, item2]).root
        )
    }
}
