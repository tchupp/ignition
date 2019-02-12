use super::Node;

pub fn intersect(node1: Node, node2: Node) -> Node {
    if node1 == node2 {
        return node1;
    }

    let (id, low, high) = match (node1, node2) {
        (_, Node::Leaf(true)) => return node1,
        (Node::Leaf(true), _) => return node2,

        (_, Node::Leaf(false)) => return Node::Leaf(false),
        (Node::Leaf(false), _) => return Node::Leaf(false),

        (Node::Branch(id_1, low_1, high_1), Node::Branch(id_2, _, _)) if id_1 < id_2 => {
            let low = intersect(low_1.into(), node2);
            let high = Node::Leaf(false);

            (id_1, low, high)
        }
        (Node::Branch(id_1, _, _), Node::Branch(id_2, low_2, high_2)) if id_1 > id_2 => {
            let low = intersect(node1, low_2.into());
            let high = Node::Leaf(false);

            (id_2, low, high)
        }
        (Node::Branch(id_1, low_1, high_1), Node::Branch(_, low_2, high_2)) => {
            let low = intersect(low_1.into(), low_2.into());
            let high = intersect(high_1.into(), high_2.into());

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
    use zdd2::forest::root::ForestRoot;

    #[test]
    fn left_empty_right_empty() {
        let forest1: ForestRoot<&str> = ForestRoot::empty();
        let forest2: ForestRoot<&str> = ForestRoot::empty();

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::intersect(&forest1, &forest2)
        );
    }

    #[test]
    fn left_empty_right_unit() {
        let forest1: ForestRoot<&str> = ForestRoot::empty();
        let forest2 = ForestRoot::unit(&["1", "2"]);

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::intersect(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_empty() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2: ForestRoot<&str> = ForestRoot::empty();

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::intersect(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unit_disjoint() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2 = ForestRoot::unit(&["3", "4"]);

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::intersect(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unit_same() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2 = ForestRoot::unit(&["1", "2"]);

        assert_eq!(
            ForestRoot::unit(&["1", "2"]),
            ForestRoot::intersect(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unit_contains() {
        let forest1 = ForestRoot::unit(&["1"]);
        let forest2 = ForestRoot::unit(&["1", "2"]);

        assert_eq!(
            ForestRoot::unit(&["1", "2"]),
            ForestRoot::intersect(&forest1, &forest2)
        );
    }
}
