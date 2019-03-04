use std::fmt;
use std::hash::Hash;

use super::node::Node;
use super::node::NodeId;

use self::universe::Universe;

mod trees;
mod universe;

mod intersect;
mod union;
mod product;
mod subset;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Priority(pub(crate) usize);

#[derive(Clone, Eq, PartialEq)]
pub struct ForestRoot<T: Hash + Eq + Clone + Ord> {
    root: NodeId,
    universe: Universe<T>,
}

impl<T: Hash + Eq + Clone + Ord + fmt::Debug> fmt::Debug for ForestRoot<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.fmt_inner(self.root.into(), 1))
    }
}

impl<T: Hash + Eq + Clone + Ord + fmt::Debug> ForestRoot<T> {
    fn fmt_inner(&self, root: Node, indent: usize) -> String {
        match root {
            Node::Leaf(val) => format!("{}", val),
            Node::Branch(id, low, high) =>
                format!(
                    "{:?}: {:?}\n{}{}\n{}{}",
                    id,
                    self.universe.get_item(id).unwrap(),
                    "| ".repeat(indent),
                    self.fmt_inner(Node::from(low), indent + 1),
                    "| ".repeat(indent),
                    self.fmt_inner(Node::from(high), indent + 1)
                ),
        }
    }
}

impl<T: Hash + Eq + Clone + Ord + Sync + Send> ForestRoot<T> {
    pub fn empty() -> Self {
        let universe = Universe::default();
        let root = Node::FALSE;

        ForestRoot { root, universe }
    }

    pub fn unit(items: &[T]) -> Self {
        let universe = Universe::from_items(items);
        let root = items_as_node(&universe, items).into();

        ForestRoot { root, universe }
    }

    pub fn many(matrix: &[Vec<T>]) -> Self {
        let universe = Universe::from_matrix(matrix);

        let root = matrix.iter()
            .map(|items| items_as_node(&universe, items))
            .fold(Node::Leaf(false), |root, item| union::union(root, item));

        ForestRoot { root: root.into(), universe }
    }

    pub fn unique(set: &[T]) -> Self {
        let universe = Universe::from_items(set);

        let root = universe.get_priorities::<Vec<_>>(set)
            .into_iter()
            .fold(Node::Leaf(false), |root, item| Node::branch(item, root, Node::Leaf(true)));

        ForestRoot { root: root.into(), universe }
    }

    fn canonical(root: impl Into<NodeId>, universe: Universe<T>) -> Self {
        let temp = ForestRoot { root: root.into(), universe };
        let trees = temp.trees();

        Self::many(&trees)
    }

    pub fn trees(&self) -> Vec<Vec<T>> {
        trees::trees(self.root)
            .into_iter()
            .map(|set| self.universe.get_items::<Vec<_>>(&set))
            .collect()
    }

    pub fn len(&self) -> usize {
        trees::trees(self.root).len()
    }

    pub fn is_empty(&self) -> bool {
        trees::trees(self.root).is_empty()
    }

    pub fn contains(&self, set: &[T]) -> bool {
        self.trees()
            .contains(&set.into())
    }

    pub fn union(&self, other: &Self) -> Self {
        let universe = Universe::merge(&self.universe, &other.universe);

        let self_root = translate_root(&self.universe, &universe, self.root.into());
        let other_root = translate_root(&other.universe, &universe, other.root.into());
        let root = union::union(self_root, other_root);

        Self::canonical(root, universe)
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let universe = Universe::merge(&self.universe, &other.universe);

        let self_root = translate_root(&self.universe, &universe, self.root.into());
        let other_root = translate_root(&other.universe, &universe, other.root.into());
        let root = intersect::intersect(self_root, other_root);

        Self::canonical(root, universe)
    }

    pub fn product(&self, other: &Self) -> Self {
        let universe = Universe::merge(&self.universe, &other.universe);

        let self_root = translate_root(&self.universe, &universe, self.root.into());
        let other_root = translate_root(&other.universe, &universe, other.root.into());
        let root = product::product(self_root, other_root);

        Self::canonical(root, universe)
    }

    pub fn extend(&self, set: &[T]) -> Self {
        let trees: Vec<Vec<T>> = self.trees()
            .into_iter()
            .chain(vec![set.to_vec()])
            .collect();

        Self::many(&trees)
    }

    pub fn subset(self, element: T) -> Self {
        let element = match self.universe.get_priority(&element) {
            None => return Self::empty(),
            Some(element) => element,
        };
        let root = subset::subset(self.root.into(), element);

        Self::canonical(root, self.universe)
    }

    pub fn subset_many(self, elements: &[T]) -> Self {
        if elements.is_empty() {
            return self;
        }

        let elements = {
            let known_elements: Vec<_> = self.universe.get_priorities(elements);
            if known_elements.len() != elements.len() {
                return Self::empty();
            }

            known_elements
        };

        let root = subset::subset_many(self.root.into(), &elements);

        Self::canonical(root, self.universe)
    }
}

fn items_as_node<T: Hash + Eq + Clone + Ord>(universe: &Universe<T>, items: &[T]) -> Node {
    universe.get_priorities::<Vec<_>>(items)
        .into_iter()
        .fold(Node::Leaf(true), |root, item| Node::branch(item, Node::Leaf(false), root))
}

fn translate_root<T: Hash + Eq + Clone + Ord>(old_universe: &Universe<T>, new_universe: &Universe<T>, node: Node) -> Node {
    match node {
        Node::Leaf(_) => node,
        Node::Branch(id, low, high) => {
            let low = translate_root(old_universe, new_universe, low.into());
            let high = translate_root(old_universe, new_universe, high.into());

            let item = old_universe.get_item(id).unwrap();
            let id = new_universe.get_priority(item).unwrap();

            Node::branch(id, low, high)
        }
    }
}