use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Closet<'a> {
    contents: BTreeMap<&'a Family, Vec<&'a Item>>,
    item_index: BTreeMap<&'a Item, &'a Family>,
    exclusions: BTreeMap<&'a Item, Vec<&'a Item>>,
}

impl<'a> Closet<'a> {
    pub fn new() -> Closet<'a> {
        Closet {
            contents: BTreeMap::new(),
            item_index: BTreeMap::new(),
            exclusions: BTreeMap::new(),
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

        Closet { contents, item_index, exclusions }
    }

    pub fn add_exclusion_rule(&self, selection: &'a Item, exclusion: &'a Item) -> Closet {
        let contents = self.contents.clone();
        let item_index = self.item_index.clone();

        let mut exclusions = self.exclusions.clone();
        exclusions.entry(selection)
            .or_insert(vec![])
            .push(exclusion);
        exclusions.entry(exclusion)
            .or_insert(vec![])
            .push(selection);

        Closet { contents, item_index, exclusions }
    }

    pub fn get_excluded_items(&self, items: &Vec<&Item>) -> Vec<&'a Item> {
        let exclusions = &self.exclusions;

        items.iter()
            .map(|item| exclusions.get(item))
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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Family(String);

impl Family {
    pub fn new(id: &str) -> Family {
        Family(id.into())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Item(String);

impl Item {
    pub fn new(id: &str) -> Item {
        Item(id.into())
    }
}
