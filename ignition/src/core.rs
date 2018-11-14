#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Family(String);

impl Family {
    pub fn new<S>(id: S) -> Family where S: Into<String> {
        Family(id.into())
    }
}

impl From<Family> for String {
    fn from(family: Family) -> Self {
        family.0
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item(String);

impl Item {
    pub fn new<S>(id: S) -> Item where S: Into<String> {
        Item(id.into())
    }
}

impl From<Item> for String {
    fn from(item: Item) -> Self {
        item.0
    }
}