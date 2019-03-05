use std::fmt;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

use super::Node;
use super::NodeId;
use super::parser;

impl Serialize for NodeId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let node = Node::from(self);
        node.serialize(serializer)
    }
}

impl Serialize for Node {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let node_str = parser::build_node_string(*self);
        serializer.serialize_newtype_struct("Node", &node_str)
    }
}

struct NodeVisitor;

impl<'de> Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("newtype struct Node")
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(self, deserializer: D) -> Result<Node, D::Error> {
        deserializer.deserialize_str(NodeStringVisitor)
    }
}

struct NodeStringVisitor;

impl<'de> Visitor<'de> for NodeStringVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string, formatted as a Node")
    }

    fn visit_str<E: Error>(self, node_str: &str) -> Result<Node, E> {
        parser::parse_node_string(node_str)
            .map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct("Node", NodeVisitor)
            .map(NodeId::from)
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct("Node", NodeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    #[test]
    fn tokenize_leaves() {
        {
            let node = node!(Always);

            assert_tokens(&node, &[
                Token::NewtypeStruct { name: "Node" },
                Token::Str("(A)"),
            ]);
        }

        {
            let node = node!(Never);

            assert_tokens(&node, &[
                Token::NewtypeStruct { name: "Node" },
                Token::Str("(N)"),
            ]);
        }
    }

    #[test]
    fn tokenize_branch() {
        let node = node!(id: 1);

        assert_tokens(&node, &[
            Token::NewtypeStruct { name: "Node" },
            Token::Str("(1 (N) (A))"),
        ]);
    }

    #[test]
    fn tokenize_nested_branches() {
        let node = node! {
            id: 1,
            low: node!(Always),
            high: node!(id: 2)
        };

        assert_tokens(&node, &[
            Token::NewtypeStruct { name: "Node" },
            Token::Str("(1 (A) (2 (N) (A)))"),
        ]);
    }
}