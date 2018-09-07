extern crate bowtie_core;
extern crate serde_test;

#[cfg(test)]
mod tests {
    use bowtie_core::bdd::ClosetBuilder;
    use bowtie_core::bdd::Node;
    use bowtie_core::core::Family;
    use bowtie_core::core::Item;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn tokenize_small_node() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let root = Node::positive_branch(&blue) & Node::positive_branch(&red);

        assert_tokens(&root, &[
            Token::Struct { name: "Node", len: 2 },
            Token::Str("structure"),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2 },
            Token::U64(0),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2 },
            Token::U64(1),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::Str("content"),
            Token::Map { len: Some(2) },
            Token::U64(0),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shirts:blue"),
            Token::U64(1),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shirts:red"),
            Token::MapEnd,

            Token::StructEnd,
        ]);
    }

    #[test]
    fn tokenize_large_node() {
        let blue = Item::new("shirts:blue");
        let red = Item::new("shirts:red");
        let grey = Item::new("shirts:grey");

        let jeans = Item::new("pants:jeans");
        let slacks = Item::new("pants:slacks");

        let sneakers = Item::new("shoes:sneakers");
        let birkenstocks = Item::new("shoes:birkenstocks");
        let topsiders = Item::new("shoes:topsiders");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");
        let shoes = Family::new("shoes");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&shirts, &grey)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_item(&shoes, &birkenstocks)
            .add_item(&shoes, &sneakers)
            .add_item(&shoes, &topsiders)
            .add_exclusion_rule(&blue, &jeans)
            .add_exclusion_rule(&red, &slacks)
            .add_exclusion_rule(&grey, &jeans)
            .add_inclusion_rule(&birkenstocks, &slacks)
            .add_inclusion_rule(&birkenstocks, &blue);
        let closet = closet_builder.must_build();
        let root = closet.root();

        assert_tokens(root, &[
            Token::Struct { name: "Node", len: 2 },
            Token::Str("structure"),

            Token::TupleVariant { name: "Structure", variant: "Available", len: 3 },
            Token::U64(0),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2, },
            Token::U64(1),

            Token::TupleVariant { name: "Structure", variant: "Available", len: 3 },
            Token::U64(2),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2, },
            Token::U64(3),

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(4),

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(5),

            Token::TupleVariant { name: "Structure", variant: "Available", len: 3, },
            Token::U64(6),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2, },
            Token::U64(7),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(7),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(3),

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(4),

            Token::TupleVariant { name: "Structure", variant: "Available", len: 3, },
            Token::U64(5),

            Token::TupleVariant { name: "Structure", variant: "Available", len: 3, },
            Token::U64(6),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2, },
            Token::U64(7),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(7),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(6),

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(7),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(1),

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(2),

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(3),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2, },
            Token::U64(4),

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(5),

            Token::TupleVariant { name: "Structure", variant: "Available", len: 3, },
            Token::U64(6),

            Token::TupleVariant { name: "Structure", variant: "Required", len: 2, },
            Token::U64(7),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariant { name: "Structure", variant: "Excluded", len: 2, },
            Token::U64(7),
            Token::NewtypeVariant { name: "Structure", variant: "Outcome" },
            Token::Bool(true),
            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::TupleVariantEnd,

            Token::Str("content"),
            Token::Map { len: Some(8) },
            Token::U64(0),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("pants:jeans"),
            Token::U64(1),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("pants:slacks"),
            Token::U64(2),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shirts:blue"),
            Token::U64(3),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shirts:grey"),
            Token::U64(4),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shirts:red"),
            Token::U64(5),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shoes:birkenstocks"),
            Token::U64(6),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shoes:sneakers"),
            Token::U64(7),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("shoes:topsiders"),
            Token::MapEnd,

            Token::StructEnd,
        ]);
    }
}
