use core::Family;
use core::Item;
use core::Outfit;
use core::OutfitError;
use core::OutfitError::Validation;
use core::ValidationError::IncompatibleSelections;
use core::ValidationError::MultipleItemsPerFamily;
use core::ValidationError::UnknownItems;
use iterative::closet::Closet;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub fn complete_outfit(closet: Closet, selections: Vec<Item>) -> Result<Outfit, OutfitError> {
    let selections = selections.iter()
        .chain(&closet.get_included_items(&selections))
        .cloned()
        .collect::<Vec<Item>>();

    validate(&closet, &selections)?;

    let selections: BTreeMap<&Family, Item> = selections.iter()
        .map(|item| (closet.get_family(item), item))
        .map(|(family, item)| {
            let family = family.expect("validation should catch items with no family");
            (family, item.clone())
        })
        .collect();

    let selected_families: HashSet<&Family> = selections.keys().cloned().collect::<HashSet<_>>();
    let items: Vec<Item> = closet.contents().iter()
        .filter(|&(family, _)| !selected_families.contains(family))
        .fold(selections.clone(), |mut outfit: BTreeMap<&Family, Item>, (family, family_items): (&Family, &Vec<Item>)| {
            let outfit_items = &outfit.values().cloned().collect::<Vec<Item>>();
            let excluded_items = closet.get_excluded_items(outfit_items);
            let included_items = closet.get_included_items(outfit_items);

            let item = family_items.iter()
                .filter(|&item| !excluded_items.contains(item))
                .find(|&item| included_items.contains(item))
                .or(family_items.iter().find(|&item| !excluded_items.contains(item)));

            let item = match item {
                Some(i) => i.clone(),
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

fn validate(closet: &Closet, selections: &Vec<Item>) -> Result<(), OutfitError> {
    if let Some(items) = find_unknown_items(&closet, &selections) {
        return Err(Validation(UnknownItems(items)));
    }
    if let Some(items) = find_duplicate_items(&closet, &selections) {
        return Err(Validation(MultipleItemsPerFamily(items)));
    }
    if let Some(items) = find_conflicting_items(&closet, &selections) {
        return Err(Validation(IncompatibleSelections(items)));
    }

    return Ok(());
}

fn find_unknown_items(closet: &Closet, selections: &Vec<Item>) -> Option<Vec<Item>> {
    let unknown_items = selections.iter()
        .filter(|ref item| !(closet.get_family(item).is_some()))
        .cloned()
        .collect::<Vec<Item>>();

    if !unknown_items.is_empty() {
        Some(unknown_items)
    } else {
        None
    }
}

fn find_duplicate_items(closet: &Closet, selections: &Vec<Item>) -> Option<BTreeMap<Family, Vec<Item>>> {
    let duplicates: BTreeMap<Family, Vec<Item>> = selections.iter()
        .map(|item| (closet.get_family(item), item))
        .map(|(family, item): (Option<&Family>, &Item)| (family.unwrap(), item))
        .fold(BTreeMap::new(), |mut duplicates: BTreeMap<Family, Vec<Item>>, (family, item): (&Family, &Item)| {
            duplicates.entry(family.clone()).or_insert(vec![]).push(item.clone());
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

fn find_conflicting_items(closet: &Closet, selections: &Vec<Item>) -> Option<Vec<Item>> {
    let selections_set: HashSet<Item> = selections.iter().cloned().collect();

    return selections.iter()
        .map(|selection| (closet.get_excluded_items(&vec![selection.clone()]), selection.clone()))
        .map(|(excluded_items, selection)| {
            let illegal_selections = excluded_items
                .intersection(&selections_set)
                .cloned()
                .collect::<Vec<Item>>();
            (illegal_selections, selection)
        })
        .filter(|&(ref illegal_selections, _)| !illegal_selections.is_empty())
        .map(|(illegal_selections, selection): (Vec<Item>, Item)|
            illegal_selections.iter()
                .cloned()
                .chain(vec![selection])
                .collect::<Vec<Item>>()
        )
        .find(|illegal_selections| !illegal_selections.is_empty());
}
