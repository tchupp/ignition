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
    pub fn new() -> Closet<'a> {
        Closet {
            contents: BTreeMap::new(),
            item_index: BTreeMap::new(),
            exclusions: BTreeMap::new(),
            inclusions: BTreeMap::new(),
        }
    }

    pub fn add_item(&self, family: &'a Family, item: &'a Item) -> Closet {
        let mut contents = self.contents.clone();
        contents.entry(family)
            .or_insert(vec![])
            .push(item);

        let mut item_index = self.item_index.clone();
        item_index.entry(item)
            .or_insert(family);

        let exclusions = self.exclusions.clone();
        let inclusions = self.inclusions.clone();

        Closet { contents, item_index, exclusions, inclusions }
    }

    pub fn add_exclusion_rule(&self, selection: &'a Item, exclusion: &'a Item) -> Closet {
        let contents = self.contents.clone();
        let item_index = self.item_index.clone();
        let inclusions = self.inclusions.clone();

        let mut exclusions = self.exclusions.clone();
        exclusions.entry(selection)
            .or_insert(vec![])
            .push(exclusion);
        exclusions.entry(exclusion)
            .or_insert(vec![])
            .push(selection);

        Closet { contents, item_index, exclusions, inclusions }
    }

    pub fn add_inclusion_rule(&self, selection: &'a Item, inclusion: &'a Item) -> Closet {
        let contents = self.contents.clone();
        let item_index = self.item_index.clone();
        let exclusions = self.exclusions.clone();

        let mut inclusions = self.inclusions.clone();
        inclusions.entry(selection)
            .or_insert(vec![])
            .push(inclusion);

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
