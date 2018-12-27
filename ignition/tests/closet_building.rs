extern crate ignition;

#[cfg(test)]
mod error_tests {
    use ignition::ClosetBuilder;
    use ignition::ClosetBuilderError;
    use ignition::Family;
    use ignition::Item;

    #[test]
    fn adding_item_to_two_families_returns_error() {
        let blue = Item::new("shirts:blue");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&pants, &blue);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected MultipleFamiliesRegistered, but was");

        assert_eq!(
            ClosetBuilderError::MultipleFamiliesRegistered(blue, vec![shirts, pants]),
            error
        );
    }

    #[test]
    fn inclusion_rule_on_same_family_returns_error() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_items(&shirts, &[blue.clone(), red.clone()])
            .add_items(&pants, &[jeans.clone(), slacks.clone()])
            .add_inclusion_rule(&blue, &red);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected InclusionFamilyConflict, but was");

        assert_eq!(
            ClosetBuilderError::InclusionFamilyConflict(shirts, vec![blue, red]),
            error
        );
    }

    #[test]
    fn inclusion_rule_with_unknown_item_as_selection_returns_error() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let green = Item::new("shirts:green");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_inclusion_rule(&green, &jeans);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected InclusionMissingFamily, but was");

        assert_eq!(
            ClosetBuilderError::InclusionMissingFamily(green),
            error
        );
    }

    #[test]
    fn inclusion_rule_with_unknown_item_included_returns_error() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let green = Item::new("shirts:green");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_inclusion_rule(&jeans, &green);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected InclusionMissingFamily, but was");

        assert_eq!(
            ClosetBuilderError::InclusionMissingFamily(green),
            error
        );
    }

    #[test]
    fn exclusion_rule_on_same_family_returns_error() {
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
            .add_exclusion_rule(&blue, &red);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected ExclusionFamilyConflict, but was");

        assert_eq!(
            ClosetBuilderError::ExclusionFamilyConflict(shirts, vec![blue, red]),
            error
        );
    }

    #[test]
    fn exclusion_rule_with_unknown_item_as_selection_returns_error() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let green = Item::new("shirts:green");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&green, &jeans);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected ExclusionMissingFamily, but was");

        assert_eq!(
            ClosetBuilderError::ExclusionMissingFamily(green),
            error
        );
    }

    #[test]
    fn exclusion_rule_with_unknown_item_excluded_returns_error() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let green = Item::new("shirts:green");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&slacks, &green);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected ExclusionMissingFamily, but was");

        assert_eq!(
            ClosetBuilderError::ExclusionMissingFamily(green),
            error
        );
    }
}
