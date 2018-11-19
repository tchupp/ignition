use std::collections::BTreeMap;
use std::collections::BTreeSet;

use core::Family;
use core::Item;
use weave::core::ItemStatus;
use weave::Tree;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Closet {
    tree: Tree<Item>,
    item_index: BTreeMap<Item, Family>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ClosetError {
    UnknownItems(Vec<Item>),
}

impl Closet {
    pub fn new(tree: Tree<Item>, item_index: BTreeMap<Item, Family>) -> Closet {
        Closet { tree, item_index }
    }

    pub fn outfits(&self) -> BTreeSet<BTreeSet<Item>> {
        self.tree.combinations()
    }

    pub fn outfits_with(&self, selections: &[Item], exclusions: &[Item]) -> BTreeSet<BTreeSet<Item>> {
        self.tree.combinations_with(selections, exclusions)
    }

    pub fn options(&self, selections: &[Item], exclusions: &[Item]) -> Result<BTreeMap<Family, Vec<ItemStatus<Item>>>, ClosetError> {
        self.find_unknown_items(&[selections, exclusions])?;

        let summary = self.tree.summarize(selections, exclusions)
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

#[cfg(test)]
mod tests {
    use closet::ClosetError;
    use closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;
    use weave::core::ItemStatus;

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
