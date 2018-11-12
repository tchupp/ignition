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
        let error = closet.expect_err("expected ConflictingFamilies, but was");

        assert_eq!(
            ClosetBuilderError::ConflictingFamilies(blue, vec![shirts, pants]),
            error
        );
    }
}
