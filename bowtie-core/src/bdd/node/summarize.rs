use bdd::node;
use bdd::node::Node;
use core::Item;
use itertools::Itertools;
use std::collections::HashSet;

impl Node {
    pub fn summarize(node: &Node) -> Vec<Item> {
        Node::summarize_internal(node, HashSet::new())
            .into_iter()
            .sorted()
    }

    fn summarize_internal(node: &Node, summary: HashSet<Item>) -> HashSet<Item> {
        match node {
            Node::Leaf(_val) => summary,
            Node::Branch(id, low, high) => {
                let mut summary = summary.clone();
                summary.insert(id.clone());

                let high = node::get(*high);
                let high_summary = Node::summarize_internal(&high, summary.clone());
                summary.extend(high_summary);

                let low = node::get(*low);
                let low_summary = Node::summarize_internal(&low, summary.clone());
                summary.extend(low_summary);

                summary
            }
        }
    }
}

#[cfg(test)]
mod summarize_tests {
    use bdd::node::Node;
    use core::Item;

    #[test]
    fn summarize_returns_empty_for_leaves() {
        let summary = Node::summarize(&Node::TRUE_LEAF);
        let expected: Vec<Item> = vec![];
        assert_eq!(expected, summary);

        let summary = Node::summarize(&Node::FALSE_LEAF);
        let expected: Vec<Item> = vec![];
        assert_eq!(expected, summary);
    }

    #[test]
    fn summarize_returns_items_for_depth_1() {
        let red = Item::new("shirts:red");
        let expected: Vec<Item> = vec![red.clone()];

        let red_branch = Node::positive_branch(&red);
        let summary = Node::summarize(&red_branch);
        assert_eq!(expected, summary);

        let red_branch = Node::negative_branch(&red);
        let summary = Node::summarize(&red_branch);
        assert_eq!(expected, summary);
    }

    #[test]
    fn summarize_returns_items_for_depth_2() {
        let red = Item::new("shirts:red");
        let blue = Item::new("shirts:blue");
        let expected: Vec<Item> = vec![blue.clone(), red.clone()];

        let red_branch = Node::positive_branch(&red) ^ Node::positive_branch(&blue);
        let summary = Node::summarize(&red_branch);
        assert_eq!(expected, summary);
    }
}