use zdd::node::Node;

pub fn union(node1: Node, node2: Node) -> Node {
    if node1 == node2 {
        return node1;
    }

    let (id, low, high) = match (node1, node2) {
        (_, Node::Leaf(false)) => return node1,
        (Node::Leaf(false), _) => return node2,

        (Node::Leaf(true), Node::Leaf(true)) => return node1,

        (Node::Branch(id, low, high), Node::Leaf(true)) => {
            let low = union(low.into(), node2);
            let high = Node::from(high);

            (id, low, high)
        }
        (Node::Leaf(true), Node::Branch(id, low, high)) => {
            let low = union(node1, low.into());
            let high = Node::from(high);

            (id, low, high)
        }

        (Node::Branch(id_1, low_1, high_1), Node::Branch(id_2, _, _)) if id_1 < id_2 => {
            let low = union(low_1.into(), node2);
            let high = Node::from(high_1);

            (id_1, low, high)
        }
        (Node::Branch(id_1, _, _), Node::Branch(id_2, low_2, high_2)) if id_1 > id_2 => {
            let low = union(node1, low_2.into());
            let high = Node::from(high_2);

            (id_2, low, high)
        }
        (Node::Branch(id_1, low_1, high_1), Node::Branch(_, low_2, high_2)) => {
            let low = union(low_1.into(), low_2.into());
            let high = union(high_1.into(), high_2.into());

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
    fn union_returns_tree_that_has_both_combinations() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.tree(&[item1.clone()]);
        let tree2 = universe.tree(&[item2.clone()]);

        let tree = Tree::union(&tree1, &tree2);
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone())),
            tree.combinations()
        );

        let tree = Tree::union(&tree2, &tree1);
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone())),
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[vec![item1.clone()], vec![item2.clone()]]);
        assert_eq!(
            expected,
            tree
        );
    }

    #[test]
    fn union_returns_identity_when_trees_are_equal() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.tree(&[item1.clone()]);
        let tree2 = universe.tree(&[item1.clone()]);

        let tree = Tree::union(&tree1, &tree2);
        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        );

        let tree = Tree::union(&tree2, &tree1);
        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[vec![item1.clone()]]);
        assert_eq!(
            expected,
            tree
        );
    }

    #[test]
    fn union_returns_identity_when_one_tree_is_empty() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.empty_tree();
        let tree2 = universe.tree(&[item1.clone()]);

        let tree = Tree::union(&tree1, &tree2);
        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        );

        let tree = Tree::union(&tree2, &tree1);
        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[vec![item1.clone()]]);
        assert_eq!(
            expected,
            tree
        );
    }

    #[test]
    fn union_returns_correct_when_one_tree_is_unit() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.unit_tree();
        let tree2 = universe.tree(&[item1.clone()]);

        let tree = Tree::union(&tree1, &tree2);
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!()),
            tree.combinations()
        );

        let tree = Tree::union(&tree2, &tree1);
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!()),
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[vec![item1.clone()], vec![]]);
        assert_eq!(
            expected,
            tree
        );
    }

    #[test]
    fn union_returns_unit_when_both_trees_are_unit() {
        let universe = Universe::default();
        let tree1 = universe.unit_tree();
        let tree2 = universe.unit_tree();

        let tree = Tree::union(&tree1, &tree2);
        assert_eq!(
            btreeset!(btreeset!()),
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[vec![]]);
        assert_eq!(
            expected,
            tree
        );
    }

    #[test]
    fn union_returns_empty_when_both_trees_are_empty() {
        let universe = Universe::default();
        let tree1 = universe.empty_tree();
        let tree2 = universe.empty_tree();

        let tree = Tree::union(&tree1, &tree2);
        assert_eq!(
            btreeset!(),
            tree.combinations()
        );

        let expected = universe.hyper_tree(&[]);
        assert_eq!(
            expected,
            tree
        );
    }
}