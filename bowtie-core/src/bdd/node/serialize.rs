use bdd::node::Node;
use bdd::node::structure::Content;
use bdd::node::structure::Structure;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::fmt;

impl Serialize for Node {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let structure = Structure::from(self.clone());
        let content = Content::from(self.clone());

        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("structure", &structure)?;
        state.serialize_field("content", &content)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Structure, Content };

        struct NodeVisitor;

        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Node")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Node, V::Error> {
                let structure = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let content = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Node::from((structure, content)))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Node, V::Error> {
                let mut structure = None;
                let mut content = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Structure => {
                            if structure.is_some() {
                                return Err(de::Error::duplicate_field("structure"));
                            }
                            structure = Some(map.next_value()?);
                        }
                        Field::Content => {
                            if content.is_some() {
                                return Err(de::Error::duplicate_field("content"));
                            }
                            content = Some(map.next_value()?);
                        }
                    }
                }
                let structure = structure.ok_or_else(|| de::Error::missing_field("structure"))?;
                let content = content.ok_or_else(|| de::Error::missing_field("content"))?;

                Ok(Node::from((structure, content)))
            }
        }

        const FIELDS: &'static [&'static str] = &["structure", "content"];

        deserializer.deserialize_struct("Node", FIELDS, NodeVisitor)
    }
}
