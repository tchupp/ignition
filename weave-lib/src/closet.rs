use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Closet<'a> {
    pub contents: BTreeMap<&'a Family, Vec<&'a Item>>,
    item_index: BTreeMap<&'a Item, &'a Family>,
}

impl<'a> Closet<'a> {
    pub fn new() -> Closet<'a> {
        Closet {
            contents: BTreeMap::new(),
            item_index: BTreeMap::new(),
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

        Closet { contents, item_index }
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
