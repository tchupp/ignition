use itertools::Itertools;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::hash::Hash;
use zdd::node::Node;
use zdd::node::NodeId;
use zdd::node::Priority;
use zdd::tree::Tree;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Universe<T> {
    items: Vec<T>
}

impl<T> From<Vec<T>> for Universe<T> {
    fn from(items: Vec<T>) -> Self {
        Universe { items }
    }
}

impl<T> Default for Universe<T> {
    fn default() -> Self {
        Universe { items: Vec::new() }
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
        let item_map = self.items.iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<HashMap<_, _>>();

        let root = combination.into_iter()
            .filter_map(|item| item_map.get(&item))
            .cloned()
            .sorted()
            .into_iter()
            .rev()
            .fold(Node::TRUE, |next, id| NodeId::from(Node::required_branch(id, next)));

        Tree::from_root(self.clone(), root)
    }

    pub fn unique_tree(&self, items: &[T]) -> Tree<T> {
        items.into_iter()
            .map(|item| self.tree(&[item.clone()]))
            .fold(self.empty_tree(), |root, next| root.union(&next))
    }

    pub fn hyper_tree(&self, combinations: &[Vec<T>]) -> Tree<T> {
        combinations.into_iter()
            .map(|cb| self.tree(cb))
            .fold(self.empty_tree(), |root, next| root.union(&next))
    }

    pub fn get_item(&self, p: Priority) -> Option<T> {
        self.items.get(p).cloned()
    }

    pub fn get_items(&self, p: &[Priority]) -> BTreeSet<T> {
        p.into_iter()
            .filter_map(|p| self.items.get(*p))
            .cloned()
            .collect()
    }
}
