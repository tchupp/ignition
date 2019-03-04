use super::Priority;
use super::node::Node;
use super::node::NodeId;

pub fn trees(root: NodeId) -> Vec<Vec<Priority>> {
    let trees: Vec<Vec<Priority>> = {
        let mut trees = vec![];

        let mut queue: Vec<(Node, Vec<Priority>)> = vec![(Node::from(root), vec![])];
        while let Some((node, path)) = queue.pop() {
            match node {
                Node::Branch(id, low, high) => {
                    let low = Node::from(low);
                    let high = Node::from(high);

                    queue.push((low, path.clone()));

                    let mut path = path.clone();
                    path.push(id);
                    queue.push((high, path));
                }
                Node::Leaf(true) => trees.push(path),
                Node::Leaf(false) => {}
            };
        }

        trees
    };

    trees
}

