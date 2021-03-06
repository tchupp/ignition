extern crate serde_test;
extern crate weave;

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use weave::Universe;

    #[test]
    fn tokenize_small_node() {
        let item1 = "1";
        let item2 = "2";

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
            Token::BorrowedStr("1"),
            Token::BorrowedStr("2"),
            Token::SeqEnd,

            Token::Str("item_index"),

            Token::Map { len: Some(2) },
            Token::BorrowedStr("1"),
            Token::U64(0),
            Token::BorrowedStr("2"),
            Token::U64(1),
            Token::MapEnd,

            Token::StructEnd,

            Token::StructEnd,
        ]);
    }
}
