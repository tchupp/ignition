use bdd::closet::Closet;
use bdd::node::Node;
use core::Item;

impl Closet {
    pub fn select_item(&self, item: &Item) -> Closet {
        let item_index = self.item_index.clone();

        let root = Node::restrict(&self.root, item, true);
        let mut selections = self.selections.clone();
        selections.push(item.clone());

        Closet { item_index, selections, root }
    }
}


#[cfg(test)]
mod tests {
    use bdd::closet::Closet;
    use bdd::closet_builder::ClosetBuilder;
    use bdd::node::Node;
    use core::Family;
    use core::Item;
    use std::collections::BTreeMap;

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

        let expected = {
            let mut item_index = BTreeMap::new();
            item_index.insert(blue.clone(), shirts.clone());
            item_index.insert(red.clone(), shirts.clone());
            item_index.insert(jeans.clone(), pants.clone());
            item_index.insert(slacks.clone(), pants.clone());

            let root = {
                let red_branch = Node::negative_branch(&red);
                let jeans_low_branch = Node::positive_branch(&slacks) & red_branch.clone();
                let jeans_high_branch = Node::negative_branch(&slacks) & red_branch.clone();

                Node::branch(&jeans, jeans_low_branch, jeans_high_branch)
            };

            Closet {
                item_index,
                selections: vec![blue],
                root,
            }
        };

        assert_eq!(expected, closet);
    }

    #[test]
    fn one_selection_families_2_items_4_one_exclusion() {
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

        let expected = {
            let mut item_index = BTreeMap::new();
            item_index.insert(blue.clone(), shirts.clone());
            item_index.insert(red.clone(), shirts.clone());
            item_index.insert(jeans.clone(), pants.clone());
            item_index.insert(slacks.clone(), pants.clone());

            let root = {
                let jeans_low_branch = Node::positive_branch(&slacks) & Node::negative_branch(&blue);

                Node::branch(&jeans, jeans_low_branch, Node::FALSE_LEAF)
            };

            Closet {
                item_index,
                selections: vec![red],
                root,
            }
        };

        assert_eq!(expected, closet);
    }
}