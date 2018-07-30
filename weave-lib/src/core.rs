use std::collections::BTreeMap;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Family(String);

impl Family {
    pub fn new(id: &str) -> Family {
        Family(id.into())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Item(String);

impl Item {
    pub fn new(id: &str) -> Item {
        Item(id.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Outfit {
    items: Vec<Item>
}

impl Outfit {
    pub fn new(items: Vec<Item>) -> Outfit {
        Outfit { items }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Validation(ValidationError),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValidationError {
    UnknownItems(Vec<Item>),
    ConflictingItems(Vec<Item>),
    MultipleItemsPerFamily(BTreeMap<Family, Vec<Item>>),
}
