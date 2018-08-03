use bdd::node::Node;
use core::Family;
use core::Item;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Closet {
    item_index: BTreeMap<Item, Family>,
    root: Node,
}

impl Closet {
    pub fn new(
        item_index: BTreeMap<Item, Family>,
        root: Node,
    ) -> Closet {
        Closet { item_index, root }
    }

    pub fn get_family(&self, item: &Item) -> Option<&Family> {
        self.item_index.get(item).map(|family| family)
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn apply(&self, item: &Item, selected: bool) -> Closet {
        let new_root = Node::apply(&self.root, item, selected);
        let new_root = Node::reduce(&new_root);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
        }
    }

    pub fn reduce(&self) -> Closet {
        let new_root = Node::reduce(&self.root);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
        }
    }

    pub fn reduce_iter(&self) -> Closet {
        let new_root = Node::reduce_iter(&self.root);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
        }
    }
}
