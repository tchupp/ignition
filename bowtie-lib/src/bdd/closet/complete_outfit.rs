use bdd::closet::Closet;
use bdd::node::Node;
use core::Item;
use core::Outfit;
use core::OutfitError;
use core::OutfitError::Validation;
use core::ValidationError::UnknownItems;
use core::ValidationError::MultipleItemsPerFamily;
use core::ValidationError::ConflictingItems;
use std::collections::BTreeMap;
use core::Family;

pub fn complete_outfit(closet: &Closet, selections: Vec<Item>) -> Result<Outfit, OutfitError> {
    validate(&closet, &selections)?;

    let mut root: Node = selections.iter()
        .fold(closet.root().clone(), |new_root, selection| Node::restrict(&new_root, selection, true));

    let mut outfit_items = selections;
    loop {
        match root.clone() {
            Node::Branch(id, low, high) => {
                let high = Node::from(high);
                if &high != &Node::FALSE_LEAF {
                    outfit_items.push(id);
                    root = high;
                } else {
                    root = Node::from(low);
                }
            },
            Node::Leaf(_val) => {
                outfit_items.sort();
                return Ok(Outfit::new(outfit_items));
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
