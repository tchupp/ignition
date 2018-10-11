use core::Item;
use itertools::Itertools;
use std::collections::HashMap;
use zdd::node::Node;
use zdd::node::NodeId;
use zdd::tree::Tree;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
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