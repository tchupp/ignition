use zdd::node::Node;
use zdd::node::Priority;

pub fn union(node1: Node, node2: Node) -> Node {
    let (first_id, node1_low, node1_high, node2_low, node2_high): (Priority, Node, Node, Node, Node) =
        match (node1, node2) {
            (_, Node::Leaf(false)) => return node1,
            (Node::Leaf(false), _) => return node2,
            (Node::Leaf(true), Node::Leaf(true)) => return node1,

            (Node::Branch(id, low, high), Node::Leaf(_)) =>
                (id,
                 low.into(), high.into(),
                 node2, node2),
            (Node::Leaf(_), Node::Branch(id, low, high)) =>
                (id,
                 node1, node1,
                 low.into(), high.into()),

            (Node::Branch(id_1, low_1, high_1), Node::Branch(id_2, _, _)) if id_1 < id_2 =>
                (id_1,
                 low_1.into(), high_1.into(),
                 node2, node2),
            (Node::Branch(id_1, _, _), Node::Branch(id_2, low_2, high_2)) if id_1 > id_2 =>
                (id_2,
                 node1, node1,
                 low_2.into(), high_2.into()),
            (Node::Branch(id_1, low_1, high_1), Node::Branch(_, low_2, high_2)) =>
                (id_1,
                 low_1.into(), high_1.into(),
                 low_2.into(), high_2.into()),
        };

    let low = union(node1_low, node2_low);
    let high = union(node1_high, node2_high);

    Node::branch(first_id, low, high)
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
        let tree1 = universe.tree(vec![item1.clone()]);
        let tree2 = universe.tree(vec![item2.clone()]);

        let tree = Tree::union(&tree1, &tree2);
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone()), btreeset!(item1.clone(), item2.clone())),
            tree.combinations()
        );

        let tree = Tree::union(&tree2, &tree1);
        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone()), btreeset!(item1.clone(), item2.clone())),
            tree.combinations()
        );
    }

    #[test]
    fn union_returns_identity_when_trees_are_equal() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.tree(vec![item1.clone()]);
        let tree2 = universe.tree(vec![item1.clone()]);

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
    }

    #[test]
    fn union_returns_identity_when_one_tree_is_empty() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.empty_tree();
        let tree2 = universe.tree(vec![item1.clone()]);

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
    }

    #[test]
    fn union_returns_correct_when_one_tree_is_unit() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree1 = universe.unit_tree();
        let tree2 = universe.tree(vec![item1.clone()]);

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
    }
}