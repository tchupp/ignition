use std::collections::BTreeMap;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Family(String);

impl Family {
    pub fn new<S>(id: S) -> Family where S: Into<String> {
        Family(id.into())
    }
}

impl From<Item> for String {
    fn from(item: Item) -> Self {
        item.0
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Item(String);

impl Item {
    pub fn new<S>(id: S) -> Item where S: Into<String> {
        Item(id.into())
    }
}

impl From<Family> for String {
    fn from(family: Family) -> Self {
        family.0
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ItemStatus {
    Excluded(Item),
    Available(Item),
    Selected(Item),
}

impl ItemStatus {
    pub fn is_excluded(&self) -> bool {
        match self {
            ItemStatus::Excluded(_) => true,
            ItemStatus::Available(_) => false,
            ItemStatus::Selected(_) => false,
        }
    }

    pub fn is_available(&self) -> bool {
        match self {
            ItemStatus::Excluded(_) => false,
            ItemStatus::Available(_) => true,
            ItemStatus::Selected(_) => false,
        }
    }

    pub fn is_selected(&self) -> bool {
        match self {
            ItemStatus::Excluded(_) => false,
            ItemStatus::Available(_) => false,
            ItemStatus::Selected(_) => true,
        }
    }

    pub fn is(&self, other: &Item) -> bool {
        match self {
            ItemStatus::Excluded(inner) => inner == other,
            ItemStatus::Available(inner) => inner == other,
            ItemStatus::Selected(inner) => inner == other,
        }
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
    UnknownItems(Vec<Item>),
    IncompatibleSelections(Vec<Item>),
    MultipleItemsPerFamily(BTreeMap<Family, Vec<Item>>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum SelectItemError {
    ExcludedItem { excluded: Item },
    UnknownItem(Item),
}
