use closet::Closet;
use closet::Family;
use closet::Item;
use outfits::Error::MultipleItemsPerFamily;
use outfits::Error::UnknownItems;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Error<'a> {
    UnknownItems(Vec<&'a Item>),
    MultipleItemsPerFamily(BTreeMap<&'a Family, Vec<&'a Item>>),
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

pub fn find_first_valid_outfit<'a>(closet: Closet<'a>, selections: Vec<&'a Item>) -> Result<Outfit<'a>, Error<'a>> {
    if let Some(items) = find_unknown_items(&closet, &selections) {
        return Err(UnknownItems(items));
    }
    if let Some(items) = find_duplicate_items(&closet, &selections) {
        return Err(MultipleItemsPerFamily(items));
    }

    let selected_families: Vec<&Family> = selections.iter()
        .map(|item| closet.get_family(item))
        .filter(|family| family.is_some())
        .map(|family| family.unwrap())
        .collect();

    let items = closet.contents().iter()
        .filter(|&(family, _)| !selected_families.contains(family))
        .map(|(_, items)| items.first())
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .chain(&selections)
        .cloned()
        .collect::<Vec<&Item>>();

    return Ok(Outfit::new(items));
}

fn find_unknown_items<'a>(closet: &Closet, selections: &Vec<&'a Item>) -> Option<Vec<&'a Item>> {
    let unknown_items = selections.iter()
        .filter(|&&item| !(closet.get_family(&item).is_some()))
        .cloned()
        .collect::<Vec<&Item>>();

    if unknown_items.is_empty() {
        None
    } else {
        Some(unknown_items)
    }
}

fn find_duplicate_items<'a>(closet: &Closet<'a>, selections: &Vec<&'a Item>) -> Option<BTreeMap<&'a Family, Vec<&'a Item>>> {
    let duplicates: BTreeMap<&Family, Vec<&Item>> = selections.iter()
        .map(|item| (closet.get_family(item), item))
        .map(|(family, item)| (family.unwrap(), item))
        .fold(BTreeMap::new(), |mut duplicates, (family, &item)| {
            duplicates.entry(family).or_insert(vec![]).push(item);
            duplicates
        })
        .iter()
        .filter(|&(_, items)| items.len() > 1)
        .map(|(family, items)| (family.clone(), items.clone()))
        .collect();

    if duplicates.is_empty() {
        None
    } else {
        Some(duplicates)
    }
}
