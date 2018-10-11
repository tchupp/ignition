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
