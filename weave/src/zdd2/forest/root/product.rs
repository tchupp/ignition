use super::Node;
use super::union;

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
            let low_1_low_2 = product(low_1.into(), low_2.into());
            let low_1_high_2 = product(low_1.into(), high_2.into());

            let new_high = {
                let high_1_low_2 = product(high_1.into(), low_2.into());
                let high_1_high_2 = product(high_1.into(), high_2.into());

                union::union(high_1_low_2, high_1_high_2)
            };

            let high = union::union(low_1_high_2, new_high);

            (id_1, low_1_low_2, high)
        }
    };

    Node::branch(id, low, high)
}

#[cfg(test)]
mod tests {
    use zdd2::forest::root::ForestRoot;

    #[test]
    fn left_empty_right_empty() {
        let forest1: ForestRoot<&str> = ForestRoot::empty();
        let forest2: ForestRoot<&str> = ForestRoot::empty();

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn left_empty_right_unit() {
        let forest1: ForestRoot<&str> = ForestRoot::empty();
        let forest2 = ForestRoot::unit(&["1", "2"]);

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_empty() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2: ForestRoot<&str> = ForestRoot::empty();

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unit_disjoint() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2 = ForestRoot::unit(&["3", "4"]);

        assert_eq!(
            ForestRoot::many(&[
                vec!["1", "2", "3", "4"],
            ]),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unique_disjoint() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2 = ForestRoot::unique(&["3", "4"]);

        assert_eq!(
            ForestRoot::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
            ]),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unit_same() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2 = ForestRoot::unit(&["1", "2"]);

        assert_eq!(
            ForestRoot::unit(&["1", "2"]),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unit_contains() {
        let forest1 = ForestRoot::unit(&["1"]);
        let forest2 = ForestRoot::unit(&["1", "2"]);

        assert_eq!(
            ForestRoot::unit(&["1", "2"]),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn many_forest_and_double_unit_forest_with_overlap_returns_many() {
        let forest1 = ForestRoot::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = ForestRoot::unit(&["3", "4"]);

        assert_eq!(
            ForestRoot::many(&[
                vec!["1", "2", "3", "4"],
                vec!["2", "3", "4"]
            ]),
            ForestRoot::product(&forest1, &forest2)
        );
    }

    #[test]
    fn many_forest_and_unique_forest_with_overlap_returns_many() {
        let forest1 = ForestRoot::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = ForestRoot::unique(&["3", "4"]);

        assert_eq!(
            ForestRoot::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
                vec!["2", "3"],
                vec!["2", "3", "4"]
            ]),
            ForestRoot::product(&forest1, &forest2)
        );
    }
}
