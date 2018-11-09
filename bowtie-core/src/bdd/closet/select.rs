use bdd::closet::Closet;
use bdd::node::Node;
use core::Item;
use core::ItemStatus;
use core::SelectItemError;
use itertools::Itertools;

impl Closet {
    pub fn select_item(&self, item: &Item) -> Result<Closet, SelectItemError> {
        validate_selection_is_known(&self, item)?;
        validate_selection_not_excluded(&self.summary, item)?;

        let item_index = self.item_index.clone();
        let root = Node::restrict(&self.root, item, true);
        let summary = Node::summarize(&root);

        let summary = self.summary.iter()
            .filter(|s| s.is_selected())
            .cloned()
            .chain(vec![ItemStatus::Selected(item.clone())])
            .chain(summary)
            .unique()
            .sorted();

        Ok(Closet { item_index, summary, root })
    }
}

fn validate_selection_is_known(closet: &Closet, item: &Item) -> Result<(), SelectItemError> {
    closet.get_family(item)
        .map_or_else(|| Err(SelectItemError::UnknownItem(item.clone())), |_| Ok(()))
}

fn validate_selection_not_excluded(summary: &[ItemStatus], item: &Item) -> Result<(), SelectItemError> {
    summary.into_iter()
        .filter(|s| s.is_excluded())
        .find(|status| status.is(item))
        .map_or_else(|| Ok(()), |_| Err(SelectItemError::ExcludedItem { excluded: item.clone() }))
}

#[cfg(test)]
mod tests {
    use bdd::closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;
    use core::ItemStatus;
    use core::SelectItemError;

    #[test]
    fn one_selection_families_2_items_4() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);

        let closet = closet_builder.must_build();
        let closet = closet.select_item(&blue).expect("expected Closet, but was ");

        let expected = vec![
            ItemStatus::Excluded(red),
            ItemStatus::Available(jeans),
            ItemStatus::Available(slacks),
            ItemStatus::Selected(blue)
        ];
        assert_eq!(&expected, closet.summary());
    }

    #[test]
    fn one_selection_families_2_items_4_selecting_two_same_item() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);

        let closet = closet_builder.must_build();
        let closet = closet.select_item(&blue).expect("expected Closet, but was ");
        let closet = closet.select_item(&blue).expect("expected Closet, but was ");

        let expected = vec![
            ItemStatus::Excluded(red),
            ItemStatus::Available(jeans),
            ItemStatus::Available(slacks),
            ItemStatus::Selected(blue)
        ];
        assert_eq!(&expected, closet.summary());
    }

    #[test]
    fn one_selection_families_2_items_4_selecting_unknown_item() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let black = Item::new("shirts:black");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);

        let closet = closet_builder.must_build();

        let expected = Err(SelectItemError::UnknownItem(black.clone()));
        assert_eq!(
            expected,
            closet.select_item(&black)
        );
    }

    #[test]
    fn one_selection_families_2_items_4_one_exclusion_rule() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();
        let closet = closet.select_item(&red).expect("expected Closet, but was ");

        let expected = vec![
            ItemStatus::Excluded(jeans),
            ItemStatus::Excluded(blue),
            ItemStatus::Available(slacks),
            ItemStatus::Selected(red)
        ];
        assert_eq!(&expected, closet.summary());
    }

    #[test]
    fn one_selection_families_2_items_4_one_inclusion_rule() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_inclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();
        let closet = closet.select_item(&red).expect("expected Closet, but was ");

        let expected = vec![
            ItemStatus::Excluded(slacks),
            ItemStatus::Excluded(blue),
            ItemStatus::Available(jeans),
            ItemStatus::Selected(red)
        ];
        assert_eq!(&expected, closet.summary());
    }

    #[test]
    fn one_selection_families_2_items_4_selecting_two_same_family() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);

        let closet = closet_builder.must_build();
        let closet = closet.select_item(&red).expect("expected Closet, but was ");

        let expected = Err(SelectItemError::ExcludedItem { excluded: blue.clone() });
        assert_eq!(
            expected,
            closet.select_item(&blue)
        );
    }
}