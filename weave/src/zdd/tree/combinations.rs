use zdd::node::Node;
use zdd::node::NodeId;
use zdd::node::Priority;

pub fn combinations_recursive(root: NodeId) -> Vec<Vec<Priority>> {
    combinations_inner(root, &[])
        .unwrap_or_else(Vec::new)
}

fn combinations_inner(root: NodeId, path: &[Priority]) -> Option<Vec<Vec<Priority>>> {
    match Node::from(root) {
        Node::Branch(id, low, high) => {
            let low = combinations_inner(low, &path);

            let high = {
                let mut path = path.to_vec();
                path.push(id);
                combinations_inner(high, &path)
            };

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

pub fn combinations_iter(root: NodeId) -> Vec<Vec<Priority>> {
    let mut combinations = vec![];

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
            Node::Leaf(true) => combinations.push(path),
            Node::Leaf(false) => {}
        };
    }

    combinations
}

#[cfg(test)]
mod tests {
    use core::Item;
    use zdd::Universe;

    #[test]
    fn hyper_tree_with_two_sets_with_no_overlap() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);

        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()]
        ]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone())),
            tree.combinations_recursive()
        );
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone())),
            tree.combinations()
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations_with(&[], &[item2])
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations_with(&[item1], &[])
        );
    }

    #[test]
    fn unique_tree_with_two_sets_with_no_overlap() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);

        let tree = universe.unique_tree(&[
            item1.clone(),
            item2.clone(),
            item2.clone()
        ]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone())),
            tree.combinations_recursive()
        );
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone())),
            tree.combinations()
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations_with(&[], &[item2])
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations_with(&[item1], &[])
        );
    }

    #[test]
    fn tree_with_two_sets_with_one_overlap() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");
        let item3 = Item::new("3");

        let universe = Universe::from(vec![item1.clone(), item2.clone(), item3.clone()]);

        let tree = universe.hyper_tree(&[
            vec![item1.clone(), item2.clone()],
            vec![item2.clone(), item3.clone()]
        ]);

        assert_eq!(
            btreeset!(
                btreeset!(item1.clone(), item2.clone()),
                btreeset!(item2.clone(), item3.clone())
            ),
            tree.combinations()
        );

        assert_eq!(
            btreeset!(),
            tree.combinations_with(&[], &[item2.clone()])
        );
        assert_eq!(
            btreeset!(btreeset!(item2.clone(), item3.clone())),
            tree.combinations_with(&[], &[item1.clone()])
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone())),
            tree.combinations_with(&[item1], &[])
        );
    }
}