use std::collections::HashSet;

use core::ItemStatus;
use zdd::node::Node;
use zdd::node::Priority;

pub fn summarize(root: Node) -> HashSet<ItemStatus<Priority>> {
    let mut nodes = HashSet::new();

    let mut queue = vec![(root, vec![])];
    while let Some((node, path)) = queue.pop() {
        if let Node::Branch(id, low, high) = node {
            println!("checking node: {:?}\n", node);
            let low = Node::from(low);
            let high = Node::from(high);

            let item_status = match (low, high) {
                (_, Node::Leaf(false)) => ItemStatus::Excluded(id),
                (Node::Leaf(false), _) => ItemStatus::Required(id),
                _ => ItemStatus::Available(id),
            };
            println!("status: {:?}\n\n", &item_status);
            nodes.insert(item_status);

            {
                let mut path = path.clone();
                path.push(ItemStatus::Excluded(id));
                queue.push((low, path));
            }
            {
                let mut path = path.clone();
                path.push(ItemStatus::Selected(id));
                queue.push((high, path));
            }
        };
    }

    nodes
}

#[cfg(test)]
mod summarize_tests {
    use core::Item;
    use core::ItemStatus;
    use zdd::Universe;

    #[test]
    fn summarize_returns_empty_for_empty_tree() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);

        let tree = universe.empty_tree();
        assert_eq!(
            vec![
                ItemStatus::Excluded(item1.clone()),
                ItemStatus::Excluded(item2.clone()),
            ],
            tree.summarize()
        );
    }

    #[test]
    fn summarize_returns_simple_item_statuses() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);

        let tree = universe.hyper_tree(&[
            vec![item1.clone(), item2.clone()],
            vec![item1.clone()]
        ]);
        assert_eq!(
            vec![
                ItemStatus::Required(item1.clone()),
                ItemStatus::Available(item2.clone()),
            ],
            tree.summarize()
        );
    }

    #[test]
    fn summarize_returns_excluded_items() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");
        let item3 = Item::new("3");
        let item4 = Item::new("4");

        let universe = Universe::from(vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()]);

        let tree = universe.hyper_tree(&[
            vec![item1.clone(), item3.clone()],
            vec![item1.clone(), item4.clone()],
            vec![item2.clone(), item3.clone()],
            vec![item2.clone(), item4.clone()],
        ]);

        assert_eq!(
            vec![
                ItemStatus::Available(item1.clone()),
                ItemStatus::Available(item2.clone()),
                ItemStatus::Available(item3.clone()),
                ItemStatus::Available(item4.clone()),
            ],
            tree.summarize()
        );
    }
}
