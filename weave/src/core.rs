use std::hash::Hash;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "item")]
pub enum ItemStatus<T> {
    Required(T),
    Excluded(T),
    Available(T),
    Selected(T),
}

impl<T: Clone + Ord + Hash + Eq> ItemStatus<T> {
    pub fn item(&self) -> &T {
        match self {
            ItemStatus::Required(item) => item,
            ItemStatus::Excluded(item) => item,
            ItemStatus::Available(item) => item,
            ItemStatus::Selected(item) => item,
        }
    }

    pub fn is_required(&self) -> bool {
        match self {
            ItemStatus::Required(_) => true,
            ItemStatus::Excluded(_) => false,
            ItemStatus::Available(_) => false,
            ItemStatus::Selected(_) => false,
        }
    }

    pub fn is_excluded(&self) -> bool {
        match self {
            ItemStatus::Required(_) => false,
            ItemStatus::Excluded(_) => true,
            ItemStatus::Available(_) => false,
            ItemStatus::Selected(_) => false,
        }
    }

    pub fn is_available(&self) -> bool {
        match self {
            ItemStatus::Required(_) => false,
            ItemStatus::Excluded(_) => false,
            ItemStatus::Available(_) => true,
            ItemStatus::Selected(_) => false,
        }
    }

    pub fn is_selected(&self) -> bool {
        match self {
            ItemStatus::Required(_) => false,
            ItemStatus::Excluded(_) => false,
            ItemStatus::Available(_) => false,
            ItemStatus::Selected(_) => true,
        }
    }

    pub fn is(&self, other: &T) -> bool {
        match self {
            ItemStatus::Required(inner) => inner == other,
            ItemStatus::Excluded(inner) => inner == other,
            ItemStatus::Available(inner) => inner == other,
            ItemStatus::Selected(inner) => inner == other,
        }
    }

    pub fn option_map<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<ItemStatus<U>> {
        match self {
            ItemStatus::Required(x) => f(x).map(ItemStatus::Required),
            ItemStatus::Excluded(x) => f(x).map(ItemStatus::Excluded),
            ItemStatus::Available(x) => f(x).map(ItemStatus::Available),
            ItemStatus::Selected(x) => f(x).map(ItemStatus::Selected),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ItemStatus<U> {
        match self {
            ItemStatus::Required(x) => ItemStatus::Required(f(x)),
            ItemStatus::Excluded(x) => ItemStatus::Excluded(f(x)),
            ItemStatus::Available(x) => ItemStatus::Available(f(x)),
            ItemStatus::Selected(x) => ItemStatus::Selected(f(x)),
        }
    }
}
