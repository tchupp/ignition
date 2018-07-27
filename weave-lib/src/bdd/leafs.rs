use closet::Item;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Node<'a> {
    Terminal(bool),
    Leaf(&'a Item, Box<Node<'a>>, Box<Node<'a>>),
}

impl<'a> Node<'a> {
    fn leaf(id: &'a Item, left: Node<'a>, right: Node<'a>) -> Node<'a> {
        Node::Leaf(id, Box::new(left), Box::new(right))
    }
}

fn reduce<'a>(node: &'a Node<'a>) -> Node<'a> {
    return match node {
        Node::Terminal(_val) => node.clone(),
        Node::Leaf(item, ref l, ref r) => {
            let l = reduce(l);
            let r = reduce(r);

            if l == r {
                return l;
            }

            return Node::leaf(item, l, r);
        }
    };
}

#[cfg(test)]
mod tests {
    use bdd::leafs::reduce;
    use closet::Item;
    use std::collections::HashMap;
    use super::Node;
    use super::Node::*;

    #[test]
    fn or_can_be_reduced_if_left_and_right_are_equal() {
        let false_node = Terminal(false);
        let true_node = Terminal(true);

        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_leaf = Node::leaf(&jeans, false_node.clone(), true_node.clone());
        let right_leaf = Node::leaf(&jeans, false_node.clone(), true_node.clone());
        let parent_leaf = Node::leaf(&blue_shirt, left_leaf, right_leaf);

        let actual = reduce(&parent_leaf);

        let expected = Node::leaf(&jeans, false_node, true_node);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn sibling_relationship_cannot_be_reduced_in_nodes() {
        let false_node = Terminal(false);
        let true_node = Terminal(true);

        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_leaf = Node::leaf(&jeans, true_node.clone(), false_node.clone());
        let right_leaf = Node::leaf(&jeans, false_node.clone(), true_node.clone());
        let parent_leaf = Node::leaf(&blue_shirt, left_leaf.clone(), right_leaf.clone());

        let actual = reduce(&parent_leaf);

        let expected = Node::leaf(&blue_shirt, left_leaf, right_leaf);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn exclusion_rule_can_be_reduced() {
        let false_node = Terminal(false);
        let true_node = Terminal(true);

        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_leaf = Node::leaf(&jeans, true_node.clone(), true_node.clone());
        let right_leaf = Node::leaf(&jeans, true_node.clone(), false_node.clone());
        let parent_leaf = Node::leaf(&blue_shirt, left_leaf.clone(), right_leaf.clone());

        let actual = reduce(&parent_leaf);

        let expected = Node::leaf(&blue_shirt, true_node.clone(), right_leaf);
        assert_eq!(
            expected,
            actual
        );
    }

    #[test]
    fn inclusion_rule_can_be_reduced() {
        let false_node = Terminal(false);
        let true_node = Terminal(true);

        let jeans = Item::new("jeans");
        let blue_shirt = Item::new("blue_shirt");

        let left_leaf = Node::leaf(&jeans, true_node.clone(), true_node.clone());
        let right_leaf = Node::leaf(&jeans, false_node.clone(), true_node.clone());
        let parent_leaf = Node::leaf(&blue_shirt, left_leaf.clone(), right_leaf.clone());

        let actual = reduce(&parent_leaf);

        let expected = Node::leaf(&blue_shirt, true_node.clone(), right_leaf);
        assert_eq!(
            expected,
            actual
        );
    }
}
