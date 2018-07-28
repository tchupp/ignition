use core::Family;
use core::Item;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Closet<'a> {
    contents: BTreeMap<&'a Family, Vec<&'a Item>>,
    item_index: BTreeMap<&'a Item, &'a Family>,
    exclusions: BTreeMap<&'a Item, Vec<&'a Item>>,
    inclusions: BTreeMap<&'a Item, Vec<&'a Item>>,
}

impl<'a> Closet<'a> {
    pub fn new(
        contents: BTreeMap<&'a Family, Vec<&'a Item>>,
        item_index: BTreeMap<&'a Item, &'a Family>,
        exclusions: BTreeMap<&'a Item, Vec<&'a Item>>,
        inclusions: BTreeMap<&'a Item, Vec<&'a Item>>,
    ) -> Closet<'a> {
        Closet { contents, item_index, exclusions, inclusions }
    }

    pub fn get_excluded_items(&self, items: &Vec<&Item>) -> HashSet<&'a Item> {
        let exclusions = &self.exclusions;

        items.iter()
            .map(|item| exclusions.get(item))
            .filter(|items| items.is_some())
            .flat_map(|items| items.unwrap())
            .cloned()
            .collect()
    }

    pub fn get_included_items(&self, items: &Vec<&Item>) -> HashSet<&'a Item> {
        let inclusions = &self.inclusions;

        items.iter()
            .map(|item| inclusions.get(item))
            .filter(|items| items.is_some())
            .flat_map(|items| items.unwrap())
            .cloned()
            .collect()
    }

    pub fn get_family(&self, item: &'a Item) -> Option<&'a Family> {
        self.item_index.get(item).map(|&family| family)
    }

    pub fn contents(&self) -> BTreeMap<&'a Family, Vec<&'a Item>> {
        self.contents.clone()
    }
}
