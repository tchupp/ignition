use bdd::closet::Closet;
use bdd::node::Node;
use core::Item;
use core::ItemStatus;
use itertools::Itertools;

impl Closet {
    pub fn select_item(&self, item: &Item) -> Closet {
        let item_index = self.item_index.clone();
        let root = Node::restrict(&self.root, item, true);
        let summary = Node::summarize(&root);

        let summary = self.summary.iter()
            .filter(|s| s.is_selected())
            .cloned()
            .chain(vec![ItemStatus::Selected(item.clone())])
            .chain(summary)
            .sorted();

        Closet { item_index, summary, root }
    }
}


#[cfg(test)]
mod tests {
    use bdd::closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;
    use core::ItemStatus;

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
        let closet = closet.select_item(&blue);

        let expected = vec![
            ItemStatus::Excluded(red),
            ItemStatus::Available(jeans),
            ItemStatus::Available(slacks),
            ItemStatus::Selected(blue)
        ];
        assert_eq!(&expected, closet.summary());
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
        let closet = closet.select_item(&red);

        let expected = vec![
            ItemStatus::Excluded(jeans),
            ItemStatus::Excluded(blue),
            ItemStatus::Available(slacks),
            ItemStatus::Selected(red)
        ];
        assert_eq!(&expected, closet.summary());
    }
}