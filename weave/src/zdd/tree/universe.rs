use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::Hash;

use itertools::Itertools;
use serde::{Serialize, Serializer};

use zdd::node::{Node, NodeId, Priority};
use zdd::tree::Tree;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Universe<T: Ord + Hash + Eq> {
    pub(crate) items: Vec<T>,
    #[serde(serialize_with = "ordered_map")]
    item_index: HashMap<T, Priority>,
}

fn ordered_map<S: Serializer, T: Ord + Hash + Eq + Serialize>(value: &HashMap<T, Priority>, serializer: S) -> Result<S::Ok, S::Error> {
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl<T: Clone + Ord + Hash> From<Vec<T>> for Universe<T> {
    fn from(items: Vec<T>) -> Self {
        let item_index = items.iter()
            .enumerate()
            .map(|(a, b)| (b.clone(), a))
            .collect::<HashMap<_, _>>();

        Universe { items, item_index }
    }
}

impl<T: Clone + Ord + Hash> Default for Universe<T> {
    fn default() -> Self {
        Universe { items: Vec::new(), item_index: HashMap::new() }
    }
}

impl<T: Clone + Ord + Hash> Universe<T> {
    pub fn empty_tree(&self) -> Tree<T> {
        Tree::empty(self.clone())
    }

    pub fn unit_tree(&self) -> Tree<T> {
        Tree::unit(self.clone())
    }

    pub fn tree(&self, combination: &[T]) -> Tree<T> {
        let root = combination.iter()
            .filter_map(|item| self.item_index.get(&item))
            .cloned()
            .sorted()
            .into_iter()
            .rev()
            .fold(Node::TRUE, |next, id| NodeId::from(Node::required_branch(id, next)));

        Tree::from_root(self.clone(), root)
    }

    pub fn unique_tree(&self, items: &[T]) -> Tree<T> {
        items.iter()
            .map(|item| self.tree(&[item.clone()]))
            .fold(self.empty_tree(), |root, next| root.union(&next))
    }

    pub fn hyper_tree(&self, combinations: &[Vec<T>]) -> Tree<T> {
        combinations.iter()
            .map(|cb| self.tree(cb))
            .fold(self.empty_tree(), |root, next| root.union(&next))
    }

    pub fn get_item(&self, p: Priority) -> Option<T> {
        self.items.get(p).cloned()
    }

    pub fn get_items(&self, p: &[Priority]) -> BTreeSet<T> {
        p.iter()
            .filter_map(|p| self.items.get(*p))
            .cloned()
            .collect()
    }
}
