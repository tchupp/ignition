extern crate serde_test;
extern crate weave;

#[macro_use]
mod forest;

#[cfg(test)]
mod zdd_tests {
    intersect_tests!(weave::zdd2::Forest<&str>);

    union_tests!(weave::zdd2::Forest<&str>);

    product_tests!(weave::zdd2::Forest<&str>);

    subset_tests!(weave::zdd2::Forest<&str>);

    occurrences_tests!(weave::zdd2::Forest<&str>);
}

#[cfg(test)]
mod serde_tests {
    use serde_test::{assert_tokens, Token};

    use weave::zdd2::Forest;

    #[test]
    fn empty() {
        let forest = Forest::<&str>::empty();

        assert_tokens(&forest, &[
            Token::Struct { name: "Forest", len: 2 },
            Token::Str("root"),

            Token::NewtypeStruct { name: "Node" },
            Token::Str("(N)"),

            Token::Str("universe"),
            Token::NewtypeStruct { name: "Universe" },

            Token::Map { len: Some(0) },
            Token::MapEnd,

            Token::StructEnd,
        ]);
    }

    #[test]
    fn unit() {
        let forest = Forest::unit(&["1", "2"]);

        assert_tokens(&forest, &[
            Token::Struct { name: "Forest", len: 2 },
            Token::Str("root"),

            Token::NewtypeStruct { name: "Node" },
            Token::Str("(0 (N) (1 (N) (A)))"),

            Token::Str("universe"),
            Token::NewtypeStruct { name: "Universe" },

            Token::Map { len: Some(2) },
            Token::BorrowedStr("1"),
            Token::U64(1),
            Token::BorrowedStr("2"),
            Token::U64(1),
            Token::MapEnd,

            Token::StructEnd,
        ]);
    }

    #[test]
    fn many() {
        let forest = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);

        assert_tokens(&forest, &[
            Token::Struct { name: "Forest", len: 2 },
            Token::Str("root"),

            Token::NewtypeStruct { name: "Node" },
            Token::Str("(0 (N) (1 (2 (N) (A)) (A)))"),

            Token::Str("universe"),
            Token::NewtypeStruct { name: "Universe" },

            Token::Map { len: Some(3) },
            Token::BorrowedStr("1"),
            Token::U64(1),
            Token::BorrowedStr("2"),
            Token::U64(2),
            Token::BorrowedStr("3"),
            Token::U64(1),
            Token::MapEnd,

            Token::StructEnd,
        ]);
    }
}