use std::collections::BTreeMap;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

use hashbrown::HashMap;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use super::Universe;

impl<T> Serialize for Universe<T> where T: Hash + Eq + Clone + Ord + Serialize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let ordered: BTreeMap<_, _> = self.occurrences.iter().collect();
        serializer.serialize_newtype_struct("Universe", &ordered)
    }
}

struct UniverseVisitor<T: Hash + Eq + Clone + Ord> {
    marker: PhantomData<Universe<T>>,
}

impl<'de, T> Visitor<'de> for UniverseVisitor<T> where T: Hash + Eq + Clone + Ord + Deserialize<'de> {
    type Value = Universe<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("newtype struct Universe")
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(self, deserializer: D) -> Result<Universe<T>, D::Error> {
        HashMap::deserialize(deserializer).map(Universe::from_occurrences)
    }
}

impl<'de, T> Deserialize<'de> for Universe<T> where T: Hash + Eq + Clone + Ord + Deserialize<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct("Universe", UniverseVisitor { marker: PhantomData })
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::Universe;

    #[test]
    fn tokenize_empty() {
        let universe = Universe::<&str>::default();

        assert_tokens(&universe, &[
            Token::NewtypeStruct { name: "Universe" },

            Token::Map { len: Some(0) },
            Token::MapEnd,
        ]);
    }

    #[test]
    fn tokenize_from_items() {
        let universe = Universe::from_items(&["11", "22", "33"]);

        assert_tokens(&universe, &[
            Token::NewtypeStruct { name: "Universe" },

            Token::Map { len: Some(3) },
            Token::BorrowedStr("11"),
            Token::U64(1),
            Token::BorrowedStr("22"),
            Token::U64(1),
            Token::BorrowedStr("33"),
            Token::U64(1),
            Token::MapEnd,
        ]);
    }

    #[test]
    fn tokenize_from_matrix() {
        let universe = Universe::from_matrix(&[
            vec!["11", "22", "33"],
            vec!["11", "22"],
            vec!["22", "33"],
            vec!["11", "33"],
            vec!["11"],
        ]);

        assert_tokens(&universe, &[
            Token::NewtypeStruct { name: "Universe" },

            Token::Map { len: Some(3) },
            Token::BorrowedStr("11"),
            Token::U64(4),
            Token::BorrowedStr("22"),
            Token::U64(3),
            Token::BorrowedStr("33"),
            Token::U64(3),
            Token::MapEnd,
        ]);
    }
}
