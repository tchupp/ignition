use closet::Closet;
use closet::Family;
use closet::Item;
use outfits::Error::Validation;
use outfits::ValidationError::ConflictingItems;
use outfits::ValidationError::MultipleItemsPerFamily;
use outfits::ValidationError::UnknownItems;
use std::collections::BTreeMap;
use std::collections::HashSet;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Outfit<'a> {
    items: Vec<&'a Item>
}

impl<'a> Outfit<'a> {
    pub fn new(items: Vec<&'a Item>) -> Outfit {
        Outfit { items }
    }
}

pub fn complete_outfit<'a>(closet: Closet<'a>, selections: Vec<&'a Item>) -> Result<Outfit<'a>, Error<'a>> {
    let selections = selections.iter()
        .chain(closet.get_included_items(&selections).iter())
        .cloned()
        .collect();

    validate(&closet, &selections)?;

    let selections: BTreeMap<&Family, &Item> = selections.iter()
        .map(|item| (closet.get_family(item), item))
        .map(|(family, item)| {
            let family = family.expect("validation should catch items with no family");
            (family, item.clone())
        })
        .collect();

    let selected_families: HashSet<&Family> = selections.keys().cloned().collect::<HashSet<_>>();
    let items: Vec<&Item> = closet.contents().iter()
        .filter(|&(family, _)| !selected_families.contains(family))
        .fold(selections.clone(), |mut outfit: BTreeMap<&Family, &Item>, (family, family_items): (&&Family, &Vec<&Item>)| {
            let outfit_items = &outfit.values().cloned().collect::<Vec<&Item>>();
            let excluded_items = closet.get_excluded_items(outfit_items);
            let included_items = closet.get_included_items(outfit_items);

            let item = family_items.iter()
                .filter(|&item| !excluded_items.contains(item))
                .find(|&item| included_items.contains(item))
                .or(family_items.iter().find(|&item| !excluded_items.contains(item)));

            let item = match item {
                Some(i) => i,
                None => panic!("We only end up here during a conflict"),
            };
            outfit.entry(family).or_insert(item);
            outfit
        })
        .iter()
        .map(|(_family, item)| item.clone())
        .collect();

    return Ok(Outfit::new(items));
}

fn validate<'a>(closet: &Closet<'a>, selections: &Vec<&'a Item>) -> Result<(), Error<'a>> {
    if let Some(items) = find_unknown_items(&closet, &selections) {
        return Err(Validation(UnknownItems(items)));
    }
    if let Some(items) = find_duplicate_items(&closet, &selections) {
        return Err(Validation(MultipleItemsPerFamily(items)));
    }
    if let Some(items) = find_conflicting_items(&closet, &selections) {
        return Err(Validation(ConflictingItems(items)));
    }

    return Ok(());
}

fn find_unknown_items<'a>(closet: &Closet, selections: &Vec<&'a Item>) -> Option<Vec<&'a Item>> {
    let unknown_items = selections.iter()
        .filter(|&&item| !(closet.get_family(&item).is_some()))
        .cloned()
        .collect::<Vec<&Item>>();

    if !unknown_items.is_empty() {
        Some(unknown_items)
    } else {
        None
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

    if !duplicates.is_empty() {
        Some(duplicates)
    } else {
        None
    }
}

fn find_conflicting_items<'a>(closet: &Closet<'a>, selections: &Vec<&'a Item>) -> Option<Vec<&'a Item>> {
    let selections_set: HashSet<&Item> = selections.iter().cloned().collect();

    return selections.iter()
        .map(|&selection| (closet.get_excluded_items(&vec![selection]), selection))
        .map(|(excluded_items, selection)| {
            let illegal_selections = excluded_items
                .intersection(&selections_set)
                .cloned()
                .collect::<Vec<&Item>>();
            (illegal_selections, selection)
        })
        .filter(|&(ref illegal_selections, _)| !illegal_selections.is_empty())
        .map(|(illegal_selections, selection): (Vec<&Item>, &Item)|
            illegal_selections.iter()
                .cloned()
                .chain(vec![selection])
                .collect::<Vec<&Item>>()
        )
        .find(|illegal_selections| !illegal_selections.is_empty());
}
