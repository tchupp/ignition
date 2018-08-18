use std::collections::BTreeMap;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Family(String);

impl Family {
    pub fn new<S>(id: S) -> Family where S: Into<String> {
        Family(id.into())
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Item(String);

impl Item {
    pub fn new<S>(id: S) -> Item where S: Into<String> {
        Item(id.into())
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Outfit {
    items: Vec<Item>
}

impl Outfit {
    pub fn new(items: Vec<Item>) -> Outfit {
        Outfit { items }
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum OutfitError {
    Validation(ValidationError),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValidationError {
    UnknownItems(Vec<Item>),
    IncompatibleSelections(Vec<Item>),
    MultipleItemsPerFamily(BTreeMap<Family, Vec<Item>>),
}
