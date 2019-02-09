use std::hash::Hash;

use super::node::Node;
use super::node::NodeId;
use super::Tree;

use self::universe::Universe;

mod trees;
mod universe;

mod intersect;
mod union;
mod product;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Priority(pub(crate) usize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForestRoot<T: Hash + Eq + Clone> {
    root: NodeId,
    universe: Universe<T>,
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> ForestRoot<T> {
    pub fn empty() -> Self {
        let universe = Universe::default();
        let root = Node::FALSE;

        ForestRoot { root, universe }
    }

    pub fn unit(items: &[T]) -> Self {
        let universe = Universe::from_items(items);
        let root = items_as_node(&universe, items);

        ForestRoot { root, universe }
    }

    pub fn many(matrix: &[Vec<T>]) -> Self {
        let universe = Universe::from_matrix(matrix);

        let root = matrix.iter()
            .map(|items| items_as_node(&universe, items))
            .fold(Node::FALSE, |root, item| union::union(root.into(), item.into()).into());

        ForestRoot { root, universe }
    }

    pub fn trees(&self) -> Vec<Tree<T>> {
        let trees = trees::trees(self.root);
        trees.into_iter()
            .map(|set| self.universe.get_items::<Vec<_>>(&set))
            .map(|set| Tree::many(&set))
            .collect()
    }

    pub fn len(&self) -> usize {
        trees::trees(self.root).len()
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn union(&self, other: &Self) -> Self {
        let universe = Universe::merge(&self.universe, &other.universe);

        let self_root = translate_root(&self.universe, &universe, self.root.into());
        let other_root = translate_root(&other.universe, &universe, other.root.into());
        let root = union::union(self_root, other_root);

        ForestRoot { root: root.into(), universe }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let universe = Universe::merge(&self.universe, &other.universe);

        let self_root = translate_root(&self.universe, &universe, self.root.into());
        let other_root = translate_root(&other.universe, &universe, other.root.into());
        let root = intersect::intersect(self_root, other_root);

        ForestRoot { root: root.into(), universe }
    }

    pub fn product(&self, other: &Self) -> Self {
        let universe = Universe::merge(&self.universe, &other.universe);

        let self_root = translate_root(&self.universe, &universe, self.root.into());
        let other_root = translate_root(&other.universe, &universe, other.root.into());
        let root = product::product(self_root, other_root);

        ForestRoot { root: root.into(), universe }
    }

    pub fn extend(&self, set: &[T]) -> Self {
        let trees: Vec<Vec<T>> = self.trees()
            .into_iter()
            .map(|tree| tree.into())
            .chain(vec![set.to_vec()])
            .collect();

        Self::many(&trees)
    }
}

fn items_as_node<T: Hash + Eq + Clone>(universe: &Universe<T>, items: &[T]) -> NodeId {
    universe.get_priorities::<Vec<_>>(items)
        .into_iter()
        .fold(Node::TRUE, |root, item| Node::branch(item, Node::FALSE, root).into())
}

fn translate_root<T: Hash + Eq + Clone>(old_universe: &Universe<T>, new_universe: &Universe<T>, node: Node) -> Node {
    match node {
        Node::Branch(id, low, high) => {
            let low = translate_root(old_universe, new_universe, low.into());
            let high = translate_root(old_universe, new_universe, high.into());

            let item = old_universe.get_item(id).unwrap();
            let id = new_universe.get_priority(item).unwrap();

            Node::branch(id, low, high)
        }
        Node::Leaf(_) => node,
    }
}