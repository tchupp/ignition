use std::collections::BTreeMap;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Family(String);

impl Family {
    pub fn new<S>(id: S) -> Family where S: Into<String> {
        Family(id.into())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Item(String);

impl Item {
    pub fn new<S>(id: S) -> Item where S: Into<String> {
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
pub enum OutfitError {
    Validation(ValidationError),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValidationError {
    UnknownItems(Vec<Item>),
    ConflictingItems(Vec<Item>),
    MultipleItemsPerFamily(BTreeMap<Family, Vec<Item>>),
}
