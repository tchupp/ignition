use zdd::node::Node;
use zdd::node::NodeId;
use zdd::node::Priority;

pub fn combinations(root: NodeId) -> Vec<Vec<Priority>> {
    combinations_inner(root, &[])
        .unwrap_or_else(Vec::new)
}

fn combinations_inner(root: NodeId, path: &[Priority]) -> Option<Vec<Vec<Priority>>> {
    match Node::from(root) {
        Node::Branch(id, low, high) => {
            let low = combinations_inner(low, &path);

            let path = {
                let mut path = path.to_vec();
                path.push(id);
                path
            };

            let high = combinations_inner(high, &path);

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
