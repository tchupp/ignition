use closet_builder::ClosetBuilderError::{CompoundError, ConflictingFamilies, ExclusionError, InclusionError, MissingFamily};
use core::Family;
use core::Item;
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum ClosetBuilderError {
    MissingFamily(Item),
    ConflictingFamilies(Item, Vec<Family>),
    InclusionError(Family, Vec<Item>),
    ExclusionError(Family, Vec<Item>),
    CompoundError(Vec<ClosetBuilderError>),
}

pub fn validate_closet(
    contents: &BTreeMap<Family, Vec<Item>>,
    item_index: &BTreeMap<Item, Family>,
    exclusions: &BTreeMap<Item, Vec<Item>>,
    inclusions: &BTreeMap<Item, Vec<Item>>,
) -> Result<(), ClosetBuilderError> {
    let conflicts =
        vec![
            find_conflicting_families(contents, item_index),
            find_illegal_rules(exclusions, item_index, |family, items| ExclusionError(family, items)),
            find_illegal_rules(inclusions, item_index, |family, items| InclusionError(family, items))
        ]
            .iter()
            .flat_map(|conflicts| conflicts)
            .unique()
            .cloned()
            .collect::<Vec<_>>();

    return match conflicts.len() {
        0 => Ok(()),
        1 => Err(conflicts.first().unwrap().clone()),
        _ => Err(CompoundError(conflicts)),
    };
}

fn find_conflicting_families(contents: &BTreeMap<Family, Vec<Item>>, item_index: &BTreeMap<Item, Family>) -> Vec<ClosetBuilderError> {
    contents.iter()
        .flat_map(|(family, items)| {
            items.iter()
                .filter_map(|item| {
                    let item_family = match item_index.get(item) {
                        None => return Some(MissingFamily(item.clone())),
                        Some(item_family) => item_family,
                    };

                    if item_family != family {
                        Some(ConflictingFamilies(item.clone(), vec![item_family.clone(), family.clone()]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<ClosetBuilderError>>()
}

fn find_illegal_rules<F: Fn(Family, Vec<Item>) -> ClosetBuilderError>(
    rules: &BTreeMap<Item, Vec<Item>>,
    item_index: &BTreeMap<Item, Family>,
    rule_error: F,
) -> Vec<ClosetBuilderError> {
    let find_selections_and_items_without_families = |(selection, items): (&Item, &Vec<Item>)| {
        let selection_family = match item_index.get(selection) {
            None => return vec![MissingFamily(selection.clone())],
            Some(selection_family) => selection_family,
        };

        items.iter()
            .filter_map(|item| {
                let item_family = match item_index.get(item) {
                    None => return Some(MissingFamily(item.clone())),
                    Some(item_family) => item_family,
                };

                if selection_family == item_family {
                    let mut items = vec![selection.clone(), item.clone()];
                    items.sort();

                    Some(rule_error(selection_family.clone(), items))
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