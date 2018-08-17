use core::Family;
use core::Item;
use std::collections::BTreeMap;
use std::collections::HashSet;

mod categorize;
mod complete_outfit;

#[derive(Debug, Clone, PartialEq)]
pub struct Closet {
    contents: BTreeMap<Family, Vec<Item>>,
    item_index: BTreeMap<Item, Family>,
    exclusions: BTreeMap<Item, Vec<Item>>,
    inclusions: BTreeMap<Item, Vec<Item>>,
}

impl Closet {
    pub fn new(
        contents: BTreeMap<Family, Vec<Item>>,
        item_index: BTreeMap<Item, Family>,
        exclusions: BTreeMap<Item, Vec<Item>>,
        inclusions: BTreeMap<Item, Vec<Item>>,
    ) -> Closet {
        Closet { contents, item_index, exclusions, inclusions }
    }

    pub fn get_excluded_items(&self, items: &[Item]) -> HashSet<Item> {
        let exclusions = &self.exclusions;

        items.iter()
            .map(|item| exclusions.get(item))
            .filter(|items| items.is_some())
            .flat_map(|items| items.unwrap())
            .cloned()
            .collect()
    }

    pub fn get_included_items(&self, items: &[Item]) -> HashSet<Item> {
        let inclusions = &self.inclusions;

        items.iter()
            .map(|item| inclusions.get(item))
            .filter(|items| items.is_some())
            .flat_map(|items| items.unwrap())
            .cloned()
            .collect()
    }

    pub fn get_family(&self, item: &Item) -> Option<&Family> {
        self.item_index.get(item)
    }

    pub fn contents(&self) -> &BTreeMap<Family, Vec<Item>> {
        &self.contents
    }
}
