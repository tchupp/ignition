use bdd::closet::Closet;
use core::Family;
use core::Item;
use std::collections::HashMap;

impl Closet {
    pub fn categorize(&self, items: &[&Item]) -> HashMap<Family, Vec<Item>> {
        let unknown_family = Family::new("UNKNOWN");

        items.iter()
            .map(|&item| (self.get_family(item).unwrap_or(&unknown_family), item))
            .map(|(family, item)| (family.clone(), item.clone()))
            .fold(HashMap::new(), |mut categories, (family, item): (Family, Item)| {
                categories.entry(family).or_insert_with(|| vec![]).push(item);
                categories
            })
    }
}

#[cfg(test)]
mod tests {
    use bdd::closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;
    use std::collections::HashMap;

    #[test]
    fn count_nodes_families_2_items_4() {
        let shirt1 = Item::new("shirts:1");
        let shirt2 = Item::new("shirts:2");

        let pants1 = Item::new("pants:1");
        let pants2 = Item::new("pants:2");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &shirt1)
            .add_item(&shirts, &shirt2)
            .add_item(&pants, &pants1)
            .add_item(&pants, &pants2);

        let closet = closet_builder.must_build();

        let expected = {
            let mut expected = HashMap::new();
            expected.entry(shirts).or_insert(vec![shirt1.clone(), shirt2.clone()]);
            expected.entry(pants).or_insert(vec![pants1.clone(), pants2.clone()]);
            expected
        };

        assert_eq!(expected, closet.categorize(&[&shirt1, &shirt2, &pants1, &pants2]));
    }
}