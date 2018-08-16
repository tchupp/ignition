use bdd::closet::Closet;
use bdd::node;
use bdd::node::Node;
use core::Family;
use core::Item;
use core::Outfit;
use core::OutfitError;
use core::OutfitError::Validation;
use core::ValidationError::IncompatibleSelections;
use core::ValidationError::MultipleItemsPerFamily;
use core::ValidationError::UnknownItems;
use std::collections::BTreeMap;

impl Closet {
    pub fn complete_outfit(&self, selections: Vec<Item>) -> Result<Outfit, OutfitError> {
        validate(self, &selections)?;

        let mut root: Node = selections.iter()
            .fold(self.root().clone(), |new_root, selection| Node::restrict(&new_root, selection, true));

        let mut outfit_items = selections;
        loop {
            match root {
                Node::Branch(id, low, high) => {
                    let high = node::get(high);
                    let low = node::get(low);

                    match high {
                        Node::Leaf(false) => root = low,
                        _ => {
                            outfit_items.push(id);
                            root = high;
                        }
                    }
                }
                Node::Leaf(_val) => {
                    outfit_items.sort();
                    return Ok(Outfit::new(outfit_items));
                }
            }
        }
    }
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
    let root: Node = selections.iter()
        .fold(closet.root().clone(), |new_root, selection| Node::restrict(&new_root, selection, true));

    let mut outfit_items = selections.clone();
    loop {
        match root {
            Node::Leaf(false) => {
                outfit_items.sort();
                return Some(outfit_items);
            }
            _ => {
                return None;
            }
        }
    }
}

