use std::collections::BTreeMap;
use std::collections::BTreeSet;

use itertools::Itertools;

use core::Family;
use core::Item;
use weave::ItemStatus;
use weave::Tree;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Closet {
    tree: Tree<Item>,
    item_index: BTreeMap<Item, Family>,
    selections: Vec<Item>,
    exclusions: Vec<Item>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ClosetError {
    UnknownItems(Vec<Item>),
}

impl Closet {
    pub fn new(tree: Tree<Item>, item_index: BTreeMap<Item, Family>, selections: Vec<Item>, exclusions: Vec<Item>) -> Closet {
        Closet { tree, item_index, selections, exclusions }
    }

    pub fn select(&self, selections: &[Item]) -> Result<Closet, ClosetError> {
        self.find_unknown_items(&[selections])?;

        let selections: Vec<Item> = chain(&self.selections[..], selections);

        let closet = Closet::new(self.tree.clone(), self.item_index.clone(), selections, vec![]);
        Ok(closet)
    }

    pub fn outfits(&self, selections: &[Item], exclusions: &[Item]) -> Result<BTreeSet<BTreeSet<Item>>, ClosetError> {
        self.find_unknown_items(&[selections, exclusions])?;

        let selections: Vec<Item> = chain(&self.selections[..], selections);

        Ok(self.tree.combinations_with(&selections[..], exclusions))
    }

    pub fn options(&self, selections: &[Item], exclusions: &[Item]) -> Result<BTreeMap<Family, Vec<ItemStatus<Item>>>, ClosetError> {
        self.find_unknown_items(&[selections, exclusions])?;

        let selections: Vec<Item> = chain(&self.selections[..], selections);

        let summary = self.tree.summarize(&selections[..], exclusions)
            .into_iter()
            .map(|status| (self.item_index.get(status.item()).unwrap(), status))
            .fold(BTreeMap::new(), |mut duplicates: BTreeMap<Family, Vec<ItemStatus<Item>>>, (family, status): (&Family, ItemStatus<Item>)| {
                duplicates.entry(family.clone()).or_insert_with(|| vec![]).push(status);
                duplicates
            });

        Ok(summary)
    }

    fn find_unknown_items(&self, items: &[&[Item]]) -> Result<(), ClosetError> {
        let unknown_items = items.iter()
            .flat_map(|items| items.iter())
            .filter(|item| self.item_index.get(item).is_none())
            .cloned()
            .collect::<Vec<_>>();

        if !unknown_items.is_empty() {
            return Err(ClosetError::UnknownItems(unknown_items));
        }
        Ok(())
    }
}

fn chain(v1: &[Item], v2: &[Item]) -> Vec<Item> {
    v1.iter()
        .chain(v2)
        .cloned()
        .unique()
        .sorted()
        .collect_vec()
}

#[cfg(test)]
mod options_tests {
    use closet::ClosetError;
    use closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;
    use weave::ItemStatus;

    #[test]
    fn options_with_empty_selections_has_all_options_available() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let options = closet.options(&[], &[]).unwrap();
        assert_eq!(
            btreemap! {
                shirts => vec![
                    ItemStatus::Available(blue),
                    ItemStatus::Available(red),
                ],
                pants => vec![
                    ItemStatus::Available(jeans),
                    ItemStatus::Available(slacks),
                ]
            },
            options
        );
    }

    #[test]
    fn options_with_one_selection_has_correct_options() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let options = closet.options(&[red.clone()], &[]).unwrap();
        assert_eq!(
            btreemap! {
                shirts.clone() => vec![
                    ItemStatus::Excluded(blue.clone()),
                    ItemStatus::Selected(red.clone()),
                ],
                pants.clone() => vec![
                    ItemStatus::Required(slacks.clone()),
                    ItemStatus::Excluded(jeans.clone()),
                ]
            },
            options
        );

        let options = closet.options(&[blue.clone()], &[]).unwrap();
        assert_eq!(
            btreemap! {
                shirts.clone() => vec![
                    ItemStatus::Excluded(red.clone()),
                    ItemStatus::Selected(blue.clone()),
                ],
                pants.clone() => vec![
                    ItemStatus::Available(jeans.clone()),
                    ItemStatus::Available(slacks.clone()),
                ]
            },
            options
        );
    }

    #[test]
    fn options_with_red_excluded_has_correct_options() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let options = closet.options(&[], &[red.clone()]).unwrap();
        assert_eq!(
            btreemap! {
                shirts.clone() => vec![
                    ItemStatus::Required(blue.clone()),
                    ItemStatus::Excluded(red.clone()),
                ],
                pants.clone() => vec![
                    ItemStatus::Available(jeans.clone()),
                    ItemStatus::Available(slacks.clone()),
                ]
            },
            options
        );

        let options = closet.options(&[], &[blue.clone()]).unwrap();
        assert_eq!(
            btreemap! {
                shirts.clone() => vec![
                    ItemStatus::Required(red.clone()),
                    ItemStatus::Excluded(blue.clone()),
                ],
                pants.clone() => vec![
                    ItemStatus::Required(slacks.clone()),
                    ItemStatus::Excluded(jeans.clone()),
                ]
            },
            options
        );
    }

    #[test]
    fn options_with_unknown_selection_returns_error() {
        let black = Item::new("shirts:black");
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let options = closet.options(&[black.clone()], &[]).unwrap_err();
        assert_eq!(
            ClosetError::UnknownItems(vec![black.clone()]),
            options
        );
    }

    #[test]
    fn options_with_unknown_exclusion_returns_error() {
        let black = Item::new("shirts:black");
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let options = closet.options(&[], &[black.clone()]).unwrap_err();
        assert_eq!(
            ClosetError::UnknownItems(vec![black.clone()]),
            options
        );
    }
}

#[cfg(test)]
mod outfits_tests {
    use closet::ClosetError;
    use closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;

    #[test]
    fn outfits_with_empty_selections_has_all_outfits_available() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let outfits = closet.outfits(&[], &[]).unwrap();
        assert_eq!(
            btreeset!(
                btreeset!(blue.clone(), jeans.clone()),
                btreeset!(red.clone(), slacks.clone()),
                btreeset!(blue.clone(), slacks.clone()),
            ),
            outfits
        );
    }

    #[test]
    fn outfits_with_one_selection_has_correct_outfits() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let outfits = closet.outfits(&[red.clone()], &[]).unwrap();
        assert_eq!(
            btreeset!(
                btreeset!(red.clone(), slacks.clone()),
            ),
            outfits
        );

        let outfits = closet.outfits(&[blue.clone()], &[]).unwrap();
        assert_eq!(
            btreeset!(
                btreeset!(blue.clone(), jeans.clone()),
                btreeset!(blue.clone(), slacks.clone()),
            ),
            outfits
        );
    }

    #[test]
    fn outfits_with_red_excluded_has_correct_outfits() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let outfits = closet.outfits(&[], &[red.clone()]).unwrap();
        assert_eq!(
            btreeset!(
                btreeset!(blue.clone(), jeans.clone()),
                btreeset!(blue.clone(), slacks.clone()),
            ),
            outfits
        );

        let outfits = closet.outfits(&[], &[blue.clone()]).unwrap();
        assert_eq!(
            btreeset!(
                btreeset!(red.clone(), slacks.clone()),
            ),
            outfits
        );
    }

    #[test]
    fn outfits_with_unknown_selection_returns_error() {
        let black = Item::new("shirts:black");
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let outfits = closet.outfits(&[black.clone()], &[]).unwrap_err();
        assert_eq!(
            ClosetError::UnknownItems(vec![black.clone()]),
            outfits
        );
    }

    #[test]
    fn outfits_with_unknown_exclusion_returns_error() {
        let black = Item::new("shirts:black");
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.build()
            .expect("expected build to return Closet");

        let outfits = closet.outfits(&[], &[black.clone()]).unwrap_err();
        assert_eq!(
            ClosetError::UnknownItems(vec![black.clone()]),
            outfits
        );
    }
}

#[cfg(test)]
mod select_tests {
    use closet::Closet;
    use closet::ClosetError;
    use closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;

    #[test]
    fn order_doesnt_matter_when_selecting_multiple() {
        let red = Item::new("shirts:red");
        let jeans = Item::new("pants:jeans");
        let closet = build_closet();

        let closet1 = closet.select(&[red.clone(), jeans.clone()]).unwrap();
        let closet2 = closet.select(&[jeans.clone(), red.clone()]).unwrap();
        assert_eq!(
            closet1,
            closet2
        );
    }

    #[test]
    fn selecting_duplicates_doesnt_matter() {
        let red = Item::new("shirts:red");
        let closet = build_closet();

        let closet1 = closet.select(&[red.clone(), red.clone()]).unwrap();
        let closet2 = closet.select(&[red.clone()]).unwrap();
        assert_eq!(
            closet1,
            closet2
        );
    }

    #[test]
    fn selecting_red_is_the_same_as_filtering_outfits() {
        let red = Item::new("shirts:red");

        let closet_pre = build_closet();
        let outfits_pre = closet_pre.outfits(&[red.clone()], &[]);
        let options_pre = closet_pre.options(&[red.clone()], &[]);

        let closet = closet_pre.select(&[red.clone()]).unwrap();
        let outfits = closet.outfits(&[], &[]);
        let options = closet.options(&[], &[]);

        assert_eq!(
            outfits_pre,
            outfits
        );
        assert_eq!(
            options_pre,
            options
        );
    }

    #[test]
    fn selecting_multiple_is_the_same_as_filtering_outfits() {
        let red = Item::new("shirts:red");
        let slacks = Item::new("pants:slacks");

        let closet_pre = build_closet();
        let outfits_pre = closet_pre.outfits(&[red.clone(), slacks.clone()], &[]);
        let options_pre = closet_pre.options(&[red.clone(), slacks.clone()], &[]);

        let closet = closet_pre.select(&[red.clone(), slacks.clone()]).unwrap();
        let outfits = closet.outfits(&[], &[]);
        let options = closet.options(&[], &[]);

        assert_eq!(
            outfits_pre,
            outfits
        );
        assert_eq!(
            options_pre,
            options
        );
    }

    #[test]
    fn selecting_black_returns_an_error() {
        let black = Item::new("shirts:black");

        let closet = build_closet();

        let outfits = closet.select(&[black.clone()]).unwrap_err();
        assert_eq!(
            ClosetError::UnknownItems(vec![black.clone()]),
            outfits
        );
    }

    fn build_closet() -> Closet {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &slacks)
            .add_item(&pants, &jeans)
            .add_exclusion_rule(&red, &jeans);

        closet_builder.build()
            .expect("expected build to return Closet")
    }
}
