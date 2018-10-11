use core::Item;
use itertools::Itertools;
use std::collections::HashMap;
use zdd::node::Node;
use zdd::node::NodeId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Universe {
    items: Vec<Item>
}

impl From<Vec<Item>> for Universe {
    fn from(items: Vec<Item>) -> Self {
        Universe { items }
    }
}

impl Universe {
    pub fn empty_tree(&self) -> Tree {
        Tree::empty(self.clone())
    }

    pub fn unit_tree(&self) -> Tree {
        Tree::unit(self.clone())
    }

    pub fn tree(&self, items: Vec<Item>) -> Tree {
        let item_map = self.items.iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<HashMap<_, _>>();

        let root = items.into_iter()
            .filter_map(|item| item_map.get(&item))
            .cloned()
            .sorted()
            .into_iter()
            .rev()
            .fold(Node::TRUE, |next, id| NodeId::from(Node::required_branch(id, next)));

        Tree::from_root(self.clone(), root)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tree {
    root: NodeId,
    universe: Universe,
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
        let universe = Universe::from(vec![]);

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
        let universe = Universe::from(vec![]);

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
        let universe = Universe::from(vec![]);

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
}