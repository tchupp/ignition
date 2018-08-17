use closet_builder::ClosetBuilderError;
use closet_builder::ClosetBuilderError::ConflictingFamilies;
use closet_builder::ClosetBuilderError::ExclusionError;
use closet_builder::ClosetBuilderError::InclusionError;
use core::Family;
use core::Item;
use iterative::closet::Closet;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClosetBuilder {
    contents: BTreeMap<Family, Vec<Item>>,
    item_index: BTreeMap<Item, Family>,
    exclusions: BTreeMap<Item, Vec<Item>>,
    inclusions: BTreeMap<Item, Vec<Item>>,
}

impl ClosetBuilder {
    pub fn new() -> ClosetBuilder {
        ClosetBuilder {
            contents: BTreeMap::new(),
            item_index: BTreeMap::new(),
            exclusions: BTreeMap::new(),
            inclusions: BTreeMap::new(),
        }
    }

    pub fn add_item(mut self, family: &Family, item: &Item) -> ClosetBuilder {
        self.contents.entry(family.clone())
            .or_insert_with(|| vec![])
            .push(item.clone());

        self.item_index.entry(item.clone())
            .or_insert_with(|| family.clone());

        self
    }

    pub fn add_exclusion_rule(mut self, selection: &Item, exclusion: &Item) -> ClosetBuilder {
        self.exclusions.entry(selection.clone())
            .or_insert_with(|| vec![])
            .push(exclusion.clone());
        self.exclusions.entry(exclusion.clone())
            .or_insert_with(|| vec![])
            .push(selection.clone());

        self
    }

    pub fn add_inclusion_rule(mut self, selection: &Item, inclusion: &Item) -> ClosetBuilder {
        self.inclusions.entry(selection.clone())
            .or_insert_with(|| vec![])
            .push(inclusion.clone());

        self
    }

    pub fn must_build(self) -> Closet {
        self.build().expect("expected build to return Closet")
    }

    pub fn build(&self) -> Result<Closet, ClosetBuilderError> {
        ClosetBuilder::validate(self)?;

        let contents = self.contents.clone();
        let item_index = self.item_index.clone();
        let exclusions = self.exclusions.clone();
        let inclusions = self.inclusions.clone();
        Ok(Closet::new(contents, item_index, exclusions, inclusions))
    }

    fn validate(&self) -> Result<(), ClosetBuilderError> {
        let conflicts = ClosetBuilder::find_conflicting_families(self);
        if !conflicts.is_empty() {
            return Err(ConflictingFamilies(conflicts));
        }

        let conflicts = ClosetBuilder::find_illegal_include_rules(self);
        if !conflicts.is_empty() {
            return Err(InclusionError(conflicts));
        }

        let conflicts = ClosetBuilder::find_illegal_exclude_rules(self);
        if !conflicts.is_empty() {
            return Err(ExclusionError(conflicts));
        }

        Ok(())
    }

    fn find_conflicting_families(&self) -> Vec<(Item, Vec<Family>)> {
        self.contents.iter()
            .flat_map(|(family, items)| {
                items.iter()
                    .map(|item| {
                        let item_family = self.item_index
                            .get(item)
                            .unwrap_or_else(|| panic!("item `{:?}` does not have family", item));

                        if item_family != family {
                            Some((item.clone(), vec![item_family.clone(), family.clone()]))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|conflict| conflict.is_some())
            .map(|conflict| conflict.unwrap())
            .collect::<Vec<(Item, Vec<Family>)>>()
    }

    fn find_illegal_include_rules(&self) -> Vec<(Family, Vec<Item>)> {
        ClosetBuilder::find_illegal_rules(&self.inclusions, &self.item_index)
    }

    fn find_illegal_exclude_rules(&self) -> Vec<(Family, Vec<Item>)> {
        ClosetBuilder::find_illegal_rules(&self.exclusions, &self.item_index)
    }

    fn find_illegal_rules(rules: &BTreeMap<Item, Vec<Item>>, item_index: &BTreeMap<Item, Family>) -> Vec<(Family, Vec<Item>)> {
        let mut conflicts = rules.iter()
            .flat_map(|(selection, items)| {
                let selection_family = item_index
                    .get(selection)
                    .unwrap_or_else(|| panic!("item `{:?}` does not have family", selection));

                items.iter()
                    .map(|item| {
                        let item_family = item_index
                            .get(item)
                            .unwrap_or_else(|| panic!("item `{:?}` does not have family", item));

                        if selection_family == item_family {
                            let mut items = vec![selection.clone(), item.clone()];
                            items.sort();

                            Some((selection_family.clone(), items))
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>()
            })
            .filter(|conflict| conflict.is_some())
            .map(|conflict| conflict.unwrap())
            .collect::<Vec<_>>();

        conflicts.dedup_by(|a, b| a.1 == b.1);
        conflicts
    }
}

