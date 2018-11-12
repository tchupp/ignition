use zdd::node::Node;

pub fn product(node1: Node, node2: Node) -> Node {
    if node1 == node2 {
        return node1;
    }

    let (id, low, high) = match (node1, node2) {
        (_, Node::Leaf(true)) => return node1,
        (Node::Leaf(true), _) => return node2,

        (_, Node::Leaf(false)) => return Node::Leaf(false),
        (Node::Leaf(false), _) => return Node::Leaf(false),

        (Node::Branch(id_1, low_1, high_1), Node::Branch(id_2, _, _)) if id_1 < id_2 => {
            let low = product(low_1.into(), node2);
            let high = product(high_1.into(), node2);

            (id_1, low, high)
        }
        (Node::Branch(id_1, _, _), Node::Branch(id_2, low_2, high_2)) if id_1 > id_2 => {
            let low = product(node1, low_2.into());
            let high = product(node1, high_2.into());

            (id_2, low, high)
        }
        (Node::Branch(id_1, low_1, high_1), Node::Branch(_, low_2, high_2)) => {
            let low = product(low_1.into(), low_2.into());
            let high = product(high_1.into(), high_2.into());

            (id_1, low, high)
        }
    };

    if high == Node::Leaf(false) {
        return low;
    }

    Node::branch(id, low, high)
}

#[cfg(test)]
mod tests {
    use core::Item;
    use zdd::tree::Tree;
    use zdd::tree::Universe;

    #[test]
    fn product_returns_tree_that_has_correct_combination() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.tree(&[item1.clone()]);
        let tree2 = universe.tree(&[item2.clone()]);

        let tree = Tree::product(&tree1, &tree2);
        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone())),
            tree.combinations()
        );

        let tree = Tree::product(&tree2, &tree1);
        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone())),
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[vec![item1, item2]]);
        assert_eq!(
            expected,
            tree
        );
    }

    #[test]
    fn product_returns_tree_that_has_correct_combinations_one() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");
        let item3 = Item::new("3");
        let item4 = Item::new("4");

        let universe = Universe::from(vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()]);
        let tree1 = universe.unique_tree(&[item1.clone(), item2.clone(), item3.clone()]);
        let tree2 = universe.unique_tree(&[item4.clone()]);

        let expected = btreeset!(
            btreeset!(item1.clone(), item4.clone()),
            btreeset!(item2.clone(), item4.clone()),
            btreeset!(item3.clone(), item4.clone())
        );

        let tree = Tree::product(&tree1, &tree2);
        assert_eq!(
            expected,
            tree.combinations()
        );

        let tree = Tree::product(&tree2, &tree1);
        assert_eq!(
            expected,
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[
            vec![item1.clone(), item4.clone()],
            vec![item2.clone(), item4.clone()],
            vec![item3.clone(), item4.clone()]
        ]);
        assert_eq!(
            expected,
            tree
        );
    }

    #[test]
    fn product_returns_tree_that_has_correct_combinations_many() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");
        let item3 = Item::new("3");
        let item4 = Item::new("4");

        let universe = Universe::from(vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()]);
        let tree1 = universe.unique_tree(&[item1.clone(), item2.clone()]);
        let tree2 = universe.unique_tree(&[item3.clone(), item4.clone()]);

        let expected = btreeset!(
            btreeset!(item1.clone(), item3.clone()),
            btreeset!(item1.clone(), item4.clone()),
            btreeset!(item2.clone(), item3.clone()),
            btreeset!(item2.clone(), item4.clone())
        );

        let tree = Tree::product(&tree1, &tree2);
        assert_eq!(
            expected,
            tree.combinations()
        );

        let tree = Tree::product(&tree2, &tree1);
        assert_eq!(
            expected,
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[
            vec![item1.clone(), item3.clone()],
            vec![item1.clone(), item4.clone()],
            vec![item2.clone(), item3.clone()],
            vec![item2.clone(), item4.clone()]
        ]);
        assert_eq!(
            expected,
            tree
        );
    }
}