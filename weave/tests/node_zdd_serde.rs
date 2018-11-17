extern crate serde_test;
extern crate weave;

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use weave::core::Item;
    use weave::Universe;

    #[test]
    fn tokenize_small_node() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);

        let tree = universe.hyper_tree(&[vec![item1.clone(), item2.clone()]]);

        assert_tokens(&tree, &[
            Token::Struct { name: "Tree", len: 2 },
            Token::Str("root"),

            Token::NewtypeStruct { name: "Node" },
            Token::Str("(0 (F) (1 (F) (T)))"),

            Token::Str("universe"),
            Token::Struct { name: "Universe", len: 2 },
            Token::Str("items"),

            Token::Seq { len: Some(2) },
            Token::NewtypeStruct { name: "Item" },
            Token::Str("1"),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("2"),
            Token::SeqEnd,

            Token::Str("item_index"),

            Token::Map { len: Some(2) },
            Token::NewtypeStruct { name: "Item" },
            Token::Str("2"),
            Token::U64(1),
            Token::NewtypeStruct { name: "Item" },
            Token::Str("1"),
            Token::U64(0),
            Token::MapEnd,

            Token::StructEnd,

            Token::StructEnd,
        ]);
    }
}
