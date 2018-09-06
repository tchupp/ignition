use bdd::node::Node;
use core::ItemStatus;
use itertools::Itertools;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Node {
    pub fn summarize(node: &Node) -> Vec<ItemStatus> {
        let mut queue = vec![node.clone()];
        let mut nodes = HashMap::new();

        while let Some(node) = queue.pop() {
            if let Node::Branch(item, low, high) = node {
                let low = Node::from(low);
                let high = Node::from(high);

                let item_status = match high {
                    Node::Leaf(false) => ItemStatus::Excluded(item.clone()),
                    _ => ItemStatus::Available(item.clone())
                };
                nodes.entry(item.clone())
                    .or_insert_with(BinaryHeap::new)
                    .push(item_status);

                queue.push(low);
                queue.push(high);
            };
        }

        nodes.values()
            .map(|s| s.peek())
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .cloned()
            .sorted()
    }
}

#[cfg(test)]
mod summarize_tests {
    use bdd::node::Node;
    use core::Item;
    use core::ItemStatus;

    #[test]
    fn summarize_returns_empty_for_leaves() {
        let summary = Node::summarize(&Node::TRUE_LEAF);
        let expected: Vec<ItemStatus> = vec![];
        assert_eq!(expected, summary);

        let summary = Node::summarize(&Node::FALSE_LEAF);
        let expected: Vec<ItemStatus> = vec![];
        assert_eq!(expected, summary);
    }

    #[test]
    fn summarize_returns_available_items_for_depth_1() {
        let red = Item::new("shirts:red");

        let red_branch = Node::positive_branch(&red);
        let summary = Node::summarize(&red_branch);
        let expected: Vec<ItemStatus> = vec![ItemStatus::Available(red.clone())];
        assert_eq!(expected, summary);
    }

    #[test]
    fn summarize_returns_excluded_items_for_depth_1() {
        let red = Item::new("shirts:red");

        let red_branch = Node::negative_branch(&red);
        let summary = Node::summarize(&red_branch);
        let expected: Vec<ItemStatus> = vec![ItemStatus::Excluded(red.clone())];
        assert_eq!(expected, summary);
    }

    #[test]
    fn summarize_returns_available_items_for_depth_2() {
        let red = Item::new("shirts:red");
        let blue = Item::new("shirts:blue");
        let expected: Vec<ItemStatus> = vec![
            ItemStatus::Available(blue.clone()),
            ItemStatus::Available(red.clone())
        ];

        let red_branch = Node::positive_branch(&red) ^ Node::positive_branch(&blue);
        let summary = Node::summarize(&red_branch);
        assert_eq!(expected, summary);
    }
}