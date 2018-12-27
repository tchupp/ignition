use std::collections::BTreeMap;

use itertools::Itertools;

use ClosetBuilderError::{CompoundError, ExclusionFamilyConflict, ExclusionMissingFamily, InclusionFamilyConflict, InclusionMissingFamily, MultipleFamiliesRegistered};
use core::Family;
use core::Item;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClosetBuilderError {
    InclusionMissingFamily(Item),
    ExclusionMissingFamily(Item),
    MultipleFamiliesRegistered(Item, Vec<Family>),
    InclusionFamilyConflict(Family, Vec<Item>),
    ExclusionFamilyConflict(Family, Vec<Item>),
    CompoundError(Vec<ClosetBuilderError>),
}

pub(crate) fn validate_closet(
    contents: &BTreeMap<Family, Vec<Item>>,
    item_index: &BTreeMap<Item, Family>,
    exclusions: &BTreeMap<Item, Vec<Item>>,
    inclusions: &BTreeMap<Item, Vec<Item>>,
) -> Result<(), ClosetBuilderError> {
    let conflicts =
        vec![
            find_conflicting_families(contents, item_index),
            find_illegal_rules(exclusions, item_index, ExclusionFamilyConflict, ExclusionMissingFamily),
            find_illegal_rules(inclusions, item_index, InclusionFamilyConflict, InclusionMissingFamily)
        ]
            .iter()
            .flat_map(|conflicts| conflicts)
            .unique()
            .cloned()
            .collect::<Vec<_>>();

    match conflicts.len() {
        0 => Ok(()),
        1 => Err(conflicts.first().unwrap().clone()),
        _ => Err(CompoundError(conflicts)),
    }
}

fn find_conflicting_families(
    contents: &BTreeMap<Family, Vec<Item>>,
    item_index: &BTreeMap<Item, Family>,
) -> Vec<ClosetBuilderError> {
    contents.iter()
        .flat_map(|(family, items)| {
            items.iter()
                .filter_map(|item| {
                    let item_family = match item_index.get(item) {
                        None => panic!("illegal state! we should never had an item in the index with no family"),
                        Some(item_family) => item_family,
                    };

                    if item_family != family {
                        Some(MultipleFamiliesRegistered(item.clone(), vec![item_family.clone(), family.clone()]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<ClosetBuilderError>>()
}

fn find_illegal_rules(
    rules: &BTreeMap<Item, Vec<Item>>,
    item_index: &BTreeMap<Item, Family>,
    conflict_error: fn(Family, Vec<Item>) -> ClosetBuilderError,
    missing_family_error: fn(Item) -> ClosetBuilderError,
) -> Vec<ClosetBuilderError> {
    let find_selections_and_items_without_families = |(selection, items): (&Item, &Vec<Item>)| {
        let selection_family = match item_index.get(selection) {
            None => return vec![missing_family_error(selection.clone())],
            Some(selection_family) => selection_family,
        };

        items.iter()
            .filter_map(|item| {
                let item_family = match item_index.get(item) {
                    None => return Some(missing_family_error(item.clone())),
                    Some(item_family) => item_family,
                };

                if selection_family == item_family {
                    let mut items = vec![selection.clone(), item.clone()];
                    items.sort();

                    Some(conflict_error(selection_family.clone(), items))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    rules.iter()
        .flat_map(find_selections_and_items_without_families)
        .collect::<Vec<_>>()
}