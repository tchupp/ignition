use bdd::node::Node;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl Node {
    pub fn hash_structure(node: &Node) -> String {
        match node {
            Node::Leaf(_) => {
                let mut hasher = DefaultHasher::new();
                node.hash(&mut hasher);

                format!("{:x}", hasher.finish())
            }
            Node::Branch(_id, low, high) => {
                let mut hasher = DefaultHasher::new();

                let low = Node::from(low);
                let high = Node::from(high);

                Node::hash_structure(&low).hash(&mut hasher);
                Node::hash_structure(&high).hash(&mut hasher);

                format!("{:x}", hasher.finish())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bdd::node::Node;
    use core::Item;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn hash_structure_of_true_leaf() {
        let root = Node::TRUE_LEAF;

        assert_eq!(
            hash(&root),
            Node::hash_structure(&root)
        );
    }

    #[test]
    fn hash_structure_of_false_leaf() {
        let root = Node::FALSE_LEAF;

        assert_eq!(
            hash(&root),
            Node::hash_structure(&root)
        );
    }

    #[test]
    fn hash_structure_of_branch_with_depth_1() {
        let red = Item::new("shirts:red");
        let root = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);

        assert_eq!(
            hash_two(
                &hash(&Node::FALSE_LEAF),
                &hash(&Node::TRUE_LEAF),
            ),
            Node::hash_structure(&root)
        );
    }

    #[test]
    fn hash_structure_of_branch_with_unbalanced_depth_2() {
        let red = Item::new("shirts:red");
        let blue = Item::new("shirts:blue");

        let red_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let root = Node::branch(&blue, Node::FALSE_LEAF, red_branch);

        assert_eq!(
            hash_two(
                &hash(&Node::FALSE_LEAF),
                &hash_two(
                    &hash(&Node::FALSE_LEAF),
                    &hash(&Node::TRUE_LEAF),
                ),
            ),
            Node::hash_structure(&root)
        );
    }

    #[test]
    fn hash_structure_of_branch_with_balanced_depth_2() {
        let red = Item::new("shirts:red");
        let blue = Item::new("shirts:blue");
        let green = Item::new("shirts:green");

        let red_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let blue_branch = Node::branch(&blue, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let root = Node::branch(&green, blue_branch, red_branch);

        assert_eq!(
            hash_two(
                &hash_two(
                    &hash(&Node::TRUE_LEAF),
                    &hash(&Node::FALSE_LEAF),
                ),
                &hash_two(
                    &hash(&Node::FALSE_LEAF),
                    &hash(&Node::TRUE_LEAF),
                ),
            ),
            Node::hash_structure(&root)
        );
    }

    fn hash<V: Hash>(value: &V) -> String {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    fn hash_two<V: Hash>(value1: &V, value2: &V) -> String {
        let mut hasher = DefaultHasher::new();
        value1.hash(&mut hasher);
        value2.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }
}