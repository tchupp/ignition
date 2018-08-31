use bdd::node::Node;
use core::Family;
use core::Item;
use std::collections::BTreeMap;
use core::ItemStatus;

mod categorize;
mod complete_outfit;
mod node_count;
mod select;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Closet {
    item_index: BTreeMap<Item, Family>,
    summary: Vec<ItemStatus>,
    root: Node,
}

impl Closet {
    pub fn new(
        item_index: BTreeMap<Item, Family>,
        root: Node,
    ) -> Closet {
        Closet { item_index, summary: Vec::new(), root }
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn item_index(&self) -> &BTreeMap<Item, Family> {
        &self.item_index
    }

    pub fn get_family(&self, item: &Item) -> Option<&Family> {
        self.item_index.get(item)
    }
}
