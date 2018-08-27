use bdd::closet::Closet;
use bdd::node::Node;
use closet_builder::ClosetBuilderError;
use closet_builder::ClosetBuilderError::ConflictingFamilies;
use closet_builder::ClosetBuilderError::ExclusionError;
use closet_builder::ClosetBuilderError::InclusionError;
use core::Family;
use core::Item;
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
        self.validate()?;

        let root = self.contents.iter()
            .map(|(_, items)| ClosetBuilder::sibling_relationship(items))
            .fold(Node::TRUE_LEAF, |other, family_node| other & family_node);

        let root = self.exclusions.iter()
            .flat_map(|(selection, exclusions)| exclusions.iter().map(|exclusion| (selection, exclusion)).collect::<Vec<_>>())
            .map(|(selection, exclusion)| ClosetBuilder::exclusion_relationship(selection, exclusion))
            .fold(root, |new_root, exclusion| new_root & exclusion);

        let root = self.inclusions.iter()
            .flat_map(|(selection, inclusions)| inclusions.iter().map(|exclusion| (selection, exclusion)).collect::<Vec<_>>())
            .map(|(selection, inclusion)| ClosetBuilder::inclusion_relationship(selection, inclusion))
            .fold(root, |new_root, inclusion| new_root & inclusion);

        let item_index = self.item_index.clone();
        Ok(Closet::new(item_index, root))
    }

    fn sibling_relationship(items: &[Item]) -> Node {
        let all_nodes = items.iter()
            .map(|item| (item, Node::negative_branch(item)))
            .collect::<BTreeMap<&Item, Node>>();

        items.iter()
            .map(|item| {
                let mut all_nodes = all_nodes.clone();
                all_nodes.insert(item, Node::positive_branch(item));

                all_nodes.into_iter()
                    .fold(Node::TRUE_LEAF, |new_root, (_, node)| new_root & node)
            })
            .fold(Node::FALSE_LEAF, |other, item| other | item)
    }

    fn exclusion_relationship(selection: &Item, exclusion: &Item) -> Node {
        Node::negative_branch(selection) | Node::negative_branch(exclusion)
    }

    fn inclusion_relationship(selection: &Item, exclusion: &Item) -> Node {
        Node::negative_branch(selection) | Node::positive_branch(exclusion)
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

#[cfg(test)]
mod no_rules_tests {
    use bdd::node::Node;
    use core::Family;
    use core::Item;
    use super::ClosetBuilder;

    #[test]
    fn two_families_with_one_item_each() {
        let blue = Item::new("shirts:blue");
        let jeans = Item::new("pants:jeans");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans);

        let closet = closet_builder.must_build();

        let expected_cousin_node = {
            let high_branch = Node::positive_branch(&blue);
            let parent_branch = Node::branch(&jeans, Node::FALSE_LEAF, high_branch);

            parent_branch
        };

        assert_eq!(
            &expected_cousin_node,
            closet.root()
        );


        let both_selected = {
            let closet = closet.select_item(&jeans);
            closet.select_item(&blue)
        };
        assert_eq!(
            &Node::TRUE_LEAF,
            both_selected.root()
        );


        let blue_selected = {
            let closet = closet.select_item(&blue);
            closet.exclude_item(&jeans)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            blue_selected.root()
        );


        let jeans_selected = {
            let closet = closet.select_item(&jeans);
            closet.exclude_item(&blue)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            jeans_selected.root()
        );
    }

    #[test]
    fn one_families_with_two_items() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let shirts = Family::new("shirts");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue);

        let closet = closet_builder.must_build();

        let expected_sibling_node = {
            let low_branch = Node::positive_branch(&red);
            let high_branch = Node::negative_branch(&red);
            let parent_branch = Node::branch(&blue, low_branch, high_branch);

            parent_branch
        };
        assert_eq!(
            &expected_sibling_node,
            closet.root()
        );


        let red_selected = {
            let closet = closet.select_item(&red);
            closet.exclude_item(&blue)
        };
        assert_eq!(
            &Node::TRUE_LEAF,
            red_selected.root()
        );


        let blue_selected = {
            let closet = closet.select_item(&blue);
            closet.exclude_item(&red)
        };
        assert_eq!(
            &Node::TRUE_LEAF,
            blue_selected.root()
        );


        let both_selected = {
            let closet = closet.select_item(&blue);
            closet.select_item(&red)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            both_selected.root()
        );
    }

    #[test]
    fn two_families_with_two_items() {
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

        let blue_low_branch = Node::positive_branch(&red);
        let blue_high_branch = Node::negative_branch(&red);
        let blue_branch = Node::branch(&blue, blue_low_branch, blue_high_branch);

        let jeans_low_branch = Node::branch(&slacks, Node::FALSE_LEAF, &blue_branch);
        let jeans_high_branch = Node::branch(&slacks, blue_branch, Node::FALSE_LEAF);
        let jeans_branch = Node::branch(&jeans, jeans_low_branch, jeans_high_branch);

        let expected_sibling_node = jeans_branch;
        assert_eq!(
            &expected_sibling_node,
            closet.root()
        );


        let red_selected = {
            let closet = closet.exclude_item(&blue);
            closet.select_item(&red)
        };
        let expected = {
            let jeans_low_branch = Node::positive_branch(&slacks);
            let jeans_high_branch = Node::negative_branch(&slacks);

            Node::branch(&jeans, jeans_low_branch, jeans_high_branch)
        };
        assert_eq!(
            &expected,
            red_selected.root()
        );


        let blue_selected = {
            let closet = closet.select_item(&blue);
            closet.exclude_item(&red)
        };
        let expected = {
            let jeans_low_branch = Node::positive_branch(&slacks);
            let jeans_high_branch = Node::negative_branch(&slacks);

            Node::branch(&jeans, jeans_low_branch, jeans_high_branch)
        };
        assert_eq!(
            &expected,
            blue_selected.root()
        );


        let both_selected = {
            let closet = closet.select_item(&blue);
            closet.select_item(&red)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            both_selected.root()
        );
    }

    #[test]
    fn one_families_with_three_items() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let black = Item::new("shirts:black");

        let shirts = Family::new("shirts");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&shirts, &black);

        let closet = closet_builder.must_build();

        let blue_low_branch = Node::positive_branch(&red);
        let blue_high_branch = Node::negative_branch(&red);
        let black_low_branch = Node::branch(&blue, blue_low_branch, &blue_high_branch);
        let black_high_branch = Node::branch(&blue, blue_high_branch, Node::FALSE_LEAF);

        let black_branch = Node::branch(&black, black_low_branch, black_high_branch);

        let expected_sibling_node = black_branch;
        assert_eq!(
            &expected_sibling_node,
            closet.root()
        );
    }

    #[test]
    fn one_families_with_four_items() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let black = Item::new("shirts:black");
        let grey = Item::new("shirts:grey");

        let shirts = Family::new("shirts");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&shirts, &black)
            .add_item(&shirts, &grey);

        let closet = closet_builder.must_build();

        let grey_low_branch = Node::positive_branch(&red);
        let grey_high_branch = Node::negative_branch(&red);
        let blue_low_branch = Node::branch(&grey, grey_low_branch, &grey_high_branch);
        let blue_high_branch = Node::branch(&grey, grey_high_branch, Node::FALSE_LEAF);
        let black_low_branch = Node::branch(&blue, blue_low_branch, &blue_high_branch);
        let black_high_branch = Node::branch(&blue, blue_high_branch, Node::FALSE_LEAF);

        let black_branch = Node::branch(&black, black_low_branch, black_high_branch);

        let expected_sibling_node = black_branch;
        assert_eq!(
            &expected_sibling_node,
            closet.root()
        );
    }

    #[test]
    fn one_families_with_five_items() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let black = Item::new("shirts:black");
        let grey = Item::new("shirts:grey");
        let green = Item::new("shirts:green");

        let shirts = Family::new("shirts");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&shirts, &black)
            .add_item(&shirts, &grey)
            .add_item(&shirts, &green);

        let closet = closet_builder.must_build();

        let grey_low_branch = Node::positive_branch(&red);
        let grey_high_branch = Node::negative_branch(&red);
        let green_low_branch = Node::branch(&grey, grey_low_branch, &grey_high_branch);
        let green_high_branch = Node::branch(&grey, grey_high_branch, Node::FALSE_LEAF);
        let blue_low_branch = Node::branch(&green, green_low_branch, &green_high_branch);
        let blue_high_branch = Node::branch(&green, green_high_branch, Node::FALSE_LEAF);
        let black_low_branch = Node::branch(&blue, blue_low_branch, &blue_high_branch);
        let black_high_branch = Node::branch(&blue, blue_high_branch, Node::FALSE_LEAF);

        let black_branch = Node::branch(&black, black_low_branch, black_high_branch);

        let expected_sibling_node = black_branch;
        assert_eq!(
            &expected_sibling_node,
            closet.root()
        );
    }
}

#[cfg(test)]
mod exclude_rules_tests {
    use bdd::node::Node;
    use core::Family;
    use core::Item;
    use super::ClosetBuilder;

    #[test]
    fn exclusion_rule_produces_expected_bdd() {
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

        let closet = closet_builder.must_build();

        let expected = {
            let shirts_branch = Node::positive_branch(&red) ^ Node::positive_branch(&blue);
            let pants_branch = Node::positive_branch(&slacks) ^ Node::positive_branch(&jeans);

            let exclusion = Node::negative_branch(&red) | Node::negative_branch(&jeans);
            let root = shirts_branch & pants_branch;

            root & exclusion
        };
        assert_eq!(
            &expected,
            closet.root()
        );
    }

    #[test]
    fn selecting_red_disallows_selecting_jeans() {
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

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&blue)
            .select_item(&red);

        let expected = Node::negative_branch(&jeans) & Node::positive_branch(&slacks);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }

    #[test]
    fn selecting_jeans_does_not_allow_selecting_red() {
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

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&slacks)
            .select_item(&jeans);

        let expected = Node::positive_branch(&blue) & Node::negative_branch(&red);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }

    #[test]
    fn selecting_blue_does_not_exclude_any_selection() {
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

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&red)
            .select_item(&blue);

        let expected = Node::positive_branch(&slacks) ^ Node::positive_branch(&jeans);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }

    #[test]
    fn selecting_slacks_does_not_exclude_any_selection() {
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

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&jeans)
            .select_item(&slacks);

        let expected = Node::positive_branch(&blue) ^ Node::positive_branch(&red);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }
}

#[cfg(test)]
mod include_rules_tests {
    use bdd::node::Node;
    use core::Family;
    use core::Item;
    use super::ClosetBuilder;

    #[test]
    fn inclusion_rule_produces_expected_bdd() {
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
            .add_inclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();

        let expected = {
            let shirts_branch = Node::positive_branch(&red) ^ Node::positive_branch(&blue);
            let pants_branch = Node::positive_branch(&slacks) ^ Node::positive_branch(&jeans);

            let inclusion = Node::negative_branch(&red) | Node::positive_branch(&jeans);
            let root = shirts_branch & pants_branch;

            root & inclusion
        };
        assert_eq!(
            &expected,
            closet.root()
        );
    }

    #[test]
    fn selecting_red_requires_selecting_jeans() {
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
            .add_inclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&blue)
            .select_item(&red);

        let expected = Node::negative_branch(&slacks) & Node::positive_branch(&jeans);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }

    #[test]
    fn selecting_blue_does_not_require_any_selection() {
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
            .add_inclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&red)
            .select_item(&blue);

        let expected = Node::positive_branch(&slacks) ^ Node::positive_branch(&jeans);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }

    #[test]
    fn selecting_jeans_does_not_require_any_selection() {
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
            .add_inclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&slacks)
            .select_item(&jeans);

        let expected = Node::positive_branch(&blue) ^ Node::positive_branch(&red);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }

    #[test]
    fn selecting_slacks_does_not_require_any_selection() {
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
            .add_inclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();

        let red_and_jeans_selected = closet
            .exclude_item(&jeans)
            .select_item(&slacks);

        let expected = Node::positive_branch(&blue) & Node::negative_branch(&red);
        assert_eq!(
            &expected,
            red_and_jeans_selected.root()
        );
    }
}