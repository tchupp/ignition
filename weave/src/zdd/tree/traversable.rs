use std::hash::Hash;

use zdd::node::Node;
use zdd::node::NodeId;
use zdd::tree::Tree;
use zdd::tree::Universe;

#[derive(Debug, Eq, PartialEq)]
pub enum TreeNode<T: Clone + Ord + Hash + Eq> {
    Branch { value: T, low: Box<TreeNode<T>>, high: Box<TreeNode<T>> },
    Leaf { value: bool },
}

impl<T: Clone + Ord + Hash> From<Tree<T>> for TreeNode<T> {
    fn from(tree: Tree<T>) -> Self {
        from_node(&tree.universe, tree.root)
    }
}

impl<T: Clone + Ord + Hash> From<&Tree<T>> for TreeNode<T> {
    fn from(tree: &Tree<T>) -> Self {
        from_node(&tree.universe, tree.root)
    }
}

fn from_node<T: Clone + Ord + Hash>(universe: &Universe<T>, node_id: NodeId) -> TreeNode<T> {
    match Node::from(node_id) {
        Node::Leaf(value) => TreeNode::Leaf { value },
        Node::Branch(id, low, high) => {
            let value = universe.get_item(id).unwrap();

            let low = from_node(universe, low);
            let high = from_node(universe, high);

            TreeNode::Branch { value, low: Box::new(low), high: Box::new(high) }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::Item;

    use super::TreeNode;
    use super::Universe;

    #[test]
    fn test() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.tree(&[item1.clone(), item2.clone()]);

        assert_eq!(
            TreeNode::Branch {
                value: item1,
                low: Box::new(TreeNode::Leaf { value: false }),
                high: Box::new(
                    TreeNode::Branch {
                        value: item2,
                        low: Box::new(TreeNode::Leaf { value: false }),
                        high: Box::new(TreeNode::Leaf { value: true }),
                    }
                ),
            },
            TreeNode::from(tree)
        )
    }
}