extern crate bowtie_core;

#[cfg(test)]
mod no_rules_tests {
    use bowtie_core::core::Family;
    use bowtie_core::core::Item;
    use bowtie_core::core::Outfit;
    use bowtie_core::core::OutfitError::Validation;
    use bowtie_core::core::ValidationError::MultipleItemsPerFamily;
    use bowtie_core::core::ValidationError::UnknownItems;
    use bowtie_core::iterative::ClosetBuilder;
    use std::collections::BTreeMap;

    #[test]
    fn no_rules_no_selections() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans, blue]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![])
        );
    }

    #[test]
    fn no_rules_one_selection() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans, red.clone()]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![red])
        );
    }

    #[test]
    fn no_rules_selection_for_each_family() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![slacks.clone(), blue.clone()]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![slacks, blue])
        );
    }

    #[test]
    fn no_rules_unknown_selection() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let black = Item::new("shirts:black");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Err(Validation(UnknownItems(vec![black.clone()])));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![jeans, black])
        );
    }

    #[test]
    fn no_rules_more_selections_than_families() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = {
            let mut duplicates = BTreeMap::new();
            duplicates.insert(pants, vec![jeans.clone(), slacks.clone()]);

            Err(Validation(MultipleItemsPerFamily(duplicates)))
        };

        assert_eq!(
            expected,
            closet.complete_outfit(vec![jeans, blue, slacks])
        );
    }
}

#[cfg(test)]
mod exclusion_rules_tests {
    use bowtie_core::core::Family;
    use bowtie_core::core::Item;
    use bowtie_core::core::Outfit;
    use bowtie_core::core::OutfitError::Validation;
    use bowtie_core::core::ValidationError::IncompatibleSelections;
    use bowtie_core::iterative::ClosetBuilder;

    #[test]
    fn exclusion_rule_with_one_selection() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&blue, &jeans);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![slacks, blue.clone()]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![blue])
        );

        let expected = Ok(Outfit::new(vec![jeans.clone(), red]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![jeans])
        );
    }

    #[test]
    fn exclusion_rule_with_conflicting_selection() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&blue, &jeans);
        let closet = closet_builder.must_build();

        let expected = Err(Validation(IncompatibleSelections(vec![jeans.clone(), blue.clone()])));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![blue, jeans])
        );
    }

    #[test]
    #[should_panic]
    fn exclusion_rules_with_impossible_selection() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&blue, &jeans)
            .add_exclusion_rule(&blue, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![blue.clone()]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![blue])
        );
    }
}

#[cfg(test)]
mod inclusion_rules_tests {
    use bowtie_core::core::Family;
    use bowtie_core::core::Item;
    use bowtie_core::core::Outfit;
    use bowtie_core::iterative::ClosetBuilder;

    #[test]
    fn inclusion_rule_with_one_selection() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans.clone(), blue.clone()]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![])
        );


        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_inclusion_rule(&jeans, &red);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans, red]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![])
        );
    }

    #[test]
    fn inclusion_rule_is_one_way() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_inclusion_rule(&red, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![slacks.clone(), blue]));
        assert_eq!(
            expected,
            closet.complete_outfit(vec![slacks])
        );
    }
}