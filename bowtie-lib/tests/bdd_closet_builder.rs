extern crate bowtie_lib;

#[cfg(test)]
mod tests {
    use bowtie_lib::core::Family;
    use bowtie_lib::core::Item;
    use bowtie_lib::bdd::closet_builder::ClosetBuilder;
    use bowtie_lib::bdd::closet_builder::Error;

    #[test]
    fn adding_item_to_two_families_returns_error() {
        let blue = Item::new("shirts:blue");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&pants, &blue);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected ConflictingFamiliesError, but was");

        assert_eq!(
            Error::ConflictingFamilies(vec![(blue, vec![shirts, pants])]),
            error
        );
    }

    #[test]
    fn include_rule_on_same_family_returns_error() {
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
            .add_inclusion_rule(&blue, &red);

        let closet = closet_builder.build();
        let error = closet.expect_err("expected InclusionError, but was");

        assert_eq!(
            Error::InclusionError(vec![(shirts, vec![blue, red])]),
            error
        );
    }

    #[test]
    fn exclude_rule_on_same_family_returns_error() {
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
        let error = closet.expect_err("expected ExclusionError, but was");

        assert_eq!(
            Error::ExclusionError(vec![(shirts, vec![blue, red])]),
            error
        );
    }
}