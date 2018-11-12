extern crate serde_test;
extern crate weave;

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use weave::bdd::Node;
    use weave::core::Item;

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
}
