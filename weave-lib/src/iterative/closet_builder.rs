use core::Family;
use core::Item;
use iterative::closet::Closet;
use iterative::closet_builder::Error::ConflictingFamilies;
use iterative::closet_builder::Error::ExclusionError;
use iterative::closet_builder::Error::InclusionError;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub enum Error<'a> {
    ConflictingFamilies(Vec<(&'a Item, Vec<&'a Family>)>),
    InclusionError(Vec<(&'a Family, Vec<&'a Item>)>),
    ExclusionError(Vec<(&'a Family, Vec<&'a Item>)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosetBuilder<'a> {
    contents: BTreeMap<&'a Family, Vec<&'a Item>>,
    item_index: BTreeMap<&'a Item, &'a Family>,
    exclusions: BTreeMap<&'a Item, Vec<&'a Item>>,
    inclusions: BTreeMap<&'a Item, Vec<&'a Item>>,
}

impl<'a> ClosetBuilder<'a> {
    pub fn new() -> ClosetBuilder<'a> {
        ClosetBuilder {
            contents: BTreeMap::new(),
            item_index: BTreeMap::new(),
            exclusions: BTreeMap::new(),
            inclusions: BTreeMap::new(),
        }
    }

    pub fn add_item(mut self, family: &'a Family, item: &'a Item) -> ClosetBuilder<'a> {
        &self.contents.entry(family)
            .or_insert(vec![])
            .push(item);

        &self.item_index.entry(item)
            .or_insert(family);

        self
    }

    pub fn add_exclusion_rule(mut self, selection: &'a Item, exclusion: &'a Item) -> ClosetBuilder<'a> {
        &self.exclusions.entry(selection)
            .or_insert(vec![])
            .push(exclusion);
        &self.exclusions.entry(exclusion)
            .or_insert(vec![])
            .push(selection);

        self
    }

    pub fn add_inclusion_rule(mut self, selection: &'a Item, inclusion: &'a Item) -> ClosetBuilder<'a> {
        &self.inclusions.entry(selection)
            .or_insert(vec![])
            .push(inclusion);

        self
    }

    pub fn must_build(self) -> Closet<'a> {
        self.build().expect("expected build to return Ok")
    }

    pub fn build(&self) -> Result<Closet<'a>, Error<'a>> {
        ClosetBuilder::validate(self)?;

        let contents = self.contents.clone();
        let item_index = self.item_index.clone();
        let exclusions = self.exclusions.clone();
        let inclusions = self.inclusions.clone();
        Ok(Closet::new(contents, item_index, exclusions, inclusions))
    }

    fn validate(&self) -> Result<(), Error<'a>> {
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

        return Ok(());
    }

    fn find_conflicting_families(&self) -> Vec<(&'a Item, Vec<&'a Family>)> {
        self.contents.iter()
            .flat_map(|(family, items)| {
                items.iter()
                    .map(|item| {
                        let item_family = self.item_index
                            .get(item)
                            .expect(&format!("item `{:?}` does not have family", item));

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
            .collect::<Vec<(&Item, Vec<&Family>)>>()
    }

    fn find_illegal_include_rules(&self) -> Vec<(&'a Family, Vec<&'a Item>)> {
        ClosetBuilder::find_illegal_rules(&self.inclusions, &self.item_index)
    }

    fn find_illegal_exclude_rules(&self) -> Vec<(&'a Family, Vec<&'a Item>)> {
        ClosetBuilder::find_illegal_rules(&self.exclusions, &self.item_index)
    }

    fn find_illegal_rules(rules: &BTreeMap<&'a Item, Vec<&'a Item>>, item_index: &BTreeMap<&'a Item, &'a Family>) -> Vec<(&'a Family, Vec<&'a Item>)> {
        let mut conflicts = rules.iter()
            .flat_map(|(selection, items)| {
                let selection_family = item_index
                    .get(selection)
                    .expect(&format!("item `{:?}` does not have family", selection));

                items.iter()
                    .map(|item| {
                        let item_family = item_index
                            .get(item)
                            .expect(&format!("item `{:?}` does not have family", item));

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

        conflicts.dedup_by(|a ,b| a.1 == b.1);
        conflicts
    }
}

