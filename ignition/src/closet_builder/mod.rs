use std::collections::BTreeMap;

use itertools::Itertools;

use closet::Closet;
use core::Family;
use core::Item;
use weave::Tree;
use weave::Universe;

pub use self::validation::ClosetBuilderError;

mod validation;

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

    pub fn add_items(self, family: &Family, items: &[Item]) -> ClosetBuilder {
        items.into_iter()
            .fold(self, |closet_builder, item| closet_builder.add_item(family, item))
    }

    pub fn build(&self) -> Result<Closet, ClosetBuilderError> {
        validation::validate_closet(
            &self.contents,
            &self.item_index,
            &self.exclusions,
            &self.inclusions,
        )?;

        let universe: Universe<Item> = self.contents.iter()
            .flat_map(|(_, items)| items)
            .cloned()
            .collect_vec()
            .into();

        let tree = self.contents.iter()
            .map(|(_, siblings)| sibling_relationship(&universe, siblings))
            .fold(universe.unit_tree(), |acc, tree| Tree::product(&acc, &tree));

        Ok(Closet::new(tree))
    }
}

fn sibling_relationship(universe: &Universe<Item>, siblings: &[Item]) -> Tree<Item> {
    universe.unique_tree(siblings)
}

#[cfg(test)]
mod no_rules_tests {
    use closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;

    #[test]
    fn one_family_with_two_items() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let shirts = Family::new("shirts");

        let closet_builder = ClosetBuilder::new()
            .add_items(&shirts, &[red.clone()])
            .add_items(&shirts, &[blue.clone()]);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let expected = btreeset![
            btreeset![blue],
            btreeset![red]
        ];
        assert_eq!(
            expected,
            closet.outfits()
        );
    }

    #[test]
    fn two_families_with_two_item_each() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_items(&shirts, &[red.clone(), blue.clone()])
            .add_items(&pants, &[jeans.clone(), slacks.clone()]);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let expected = btreeset![
            btreeset![red.clone(), jeans.clone()],
            btreeset![red.clone(), slacks.clone()],
            btreeset![blue.clone(), jeans.clone()],
            btreeset![blue.clone(), slacks.clone()],
        ];
        assert_eq!(
            expected,
            closet.outfits()
        );
    }
}