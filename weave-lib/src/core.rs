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
pub struct Outfit<'a> {
    items: Vec<&'a Item>
}

impl<'a> Outfit<'a> {
    pub fn new(items: Vec<&'a Item>) -> Outfit {
        Outfit { items }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error<'a> {
    Validation(ValidationError<'a>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValidationError<'a> {
    UnknownItems(Vec<&'a Item>),
    ConflictingItems(Vec<&'a Item>),
    MultipleItemsPerFamily(BTreeMap<&'a Family, Vec<&'a Item>>),
}
