use bdd::node::Node;
use core::Family;
use core::Item;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Closet {
    item_index: BTreeMap<Item, Family>,
    root: Node,
    exclusions: BTreeMap<Item, Vec<Item>>,
}

impl Closet {
    pub fn new(
        item_index: BTreeMap<Item, Family>,
        root: Node,
        exclusions: BTreeMap<Item, Vec<Item>>,
    ) -> Closet {
        Closet { item_index, root, exclusions }
    }

    pub fn get_family(&self, item: &Item) -> Option<&Family> {
        self.item_index.get(item).map(|family| family)
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn apply(&self, item: &Item, selected: bool) -> Closet {
        let mut new_root = Node::apply(&self.root, item, selected);

        if selected {
            if let Some(exclusions) = self.exclusions.get(item) {
                for exclusion in exclusions {
                    new_root = Node::apply(&new_root, exclusion, false);
                }
            }
        }
        new_root = Node::reduce(&new_root);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
            exclusions: self.exclusions.clone(),
        }
    }

    pub fn reduce(&self) -> Closet {
        let new_root = Node::reduce(&self.root);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
            exclusions: self.exclusions.clone(),
        }
    }

    pub fn reduce_iter(&self) -> Closet {
        let new_root = Node::reduce_iter(&self.root);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
            exclusions: self.exclusions.clone(),
        }
    }
}
