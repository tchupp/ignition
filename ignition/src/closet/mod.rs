use std::collections::BTreeSet;

use core::Item;
use weave::core::ItemStatus;
use weave::Tree;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Closet {
    tree: Tree<Item>,
}

impl Closet {
    pub fn new(tree: Tree<Item>) -> Closet {
        Closet { tree }
    }

    pub fn outfits(&self) -> BTreeSet<BTreeSet<Item>> {
        self.tree.combinations()
    }

    pub fn outfits_with(&self, selections: &[Item], exclusions: &[Item]) -> BTreeSet<BTreeSet<Item>> {
        self.tree.combinations_with(selections, exclusions)
    }

    pub fn options(&self, selections: &[Item], exclusions: &[Item]) -> Vec<ItemStatus<Item>> {
        self.tree.summarize(selections, exclusions)
    }
}

#[cfg(test)]
mod tests {
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

        let options = closet.options(&[], &[]);
        assert_eq!(
            vec![
                ItemStatus::Available(jeans),
                ItemStatus::Available(slacks),
                ItemStatus::Available(blue),
                ItemStatus::Available(red),
            ],
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

        let options = closet.options(&[red.clone()], &[]);
        assert_eq!(
            vec![
                ItemStatus::Required(slacks.clone()),
                ItemStatus::Excluded(jeans.clone()),
                ItemStatus::Excluded(blue.clone()),
                ItemStatus::Selected(red.clone()),
            ],
            options
        );

        let options = closet.options(&[blue.clone()], &[]);
        assert_eq!(
            vec![
                ItemStatus::Excluded(red.clone()),
                ItemStatus::Available(jeans.clone()),
                ItemStatus::Available(slacks.clone()),
                ItemStatus::Selected(blue.clone()),
            ],
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

        let options = closet.options(&[], &[red.clone()]);
        assert_eq!(
            vec![
                ItemStatus::Required(blue.clone()),
                ItemStatus::Excluded(red.clone()),
                ItemStatus::Available(jeans.clone()),
                ItemStatus::Available(slacks.clone()),
            ],
            options
        );

        let options = closet.options(&[], &[blue.clone()]);
        assert_eq!(
            vec![
                ItemStatus::Required(slacks.clone()),
                ItemStatus::Required(red.clone()),
                ItemStatus::Excluded(jeans.clone()),
                ItemStatus::Excluded(blue.clone()),
            ],
            options
        );
    }
}