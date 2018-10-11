use core::Item;
use std::collections::BTreeSet;
use zdd::node::Node;
use zdd::node::NodeId;
use zdd::node::Priority;
use zdd::tree::Tree;

pub fn combinations(tree: &Tree) -> BTreeSet<BTreeSet<Item>> {
    combinations_inner(tree.root, &[])
        .unwrap_or_else(Vec::new)
        .into_iter()
        .map(|set| set.into_iter()
            .filter_map(|p| tree.universe.get_item(p))
            .cloned()
            .collect::<BTreeSet<_>>())
        .collect::<BTreeSet<_>>()
}

fn combinations_inner(root: NodeId, path: &[Priority]) -> Option<Vec<Vec<Priority>>> {
    match Node::from(root) {
        Node::Branch(id, low, high) => {
            let mut path = path.to_vec();
            path.push(id);

            let high = combinations_inner(high, &path);
            let low = combinations_inner(low, &path);

            let vec = vec![low, high]
                .into_iter()
                .filter_map(|f| f)
                .flatten()
                .collect();

            Some(vec)
        }
        Node::Leaf(true) => Some(vec![path.to_vec()]),
        Node::Leaf(false) => None,
    }
}
