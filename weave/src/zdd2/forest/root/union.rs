use super::Node;

pub fn union(node1: Node, node2: Node) -> Node {
    if node1 == node2 {
        return node1;
    }

    let (id, low, high) = match (node1, node2) {
        (_, Node::Leaf(false)) => return node1,
        (Node::Leaf(false), _) => return node2,

        (Node::Leaf(true), Node::Leaf(true)) => return Node::Leaf(true),

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
    use zdd2::forest::root::ForestRoot;

    #[test]
    fn left_empty_right_empty() {
        let forest1: ForestRoot<&str> = ForestRoot::empty();
        let forest2: ForestRoot<&str> = ForestRoot::empty();

        assert_eq!(
            ForestRoot::empty(),
            ForestRoot::union(&forest1, &forest2)
        );
    }

    #[test]
    fn left_empty_right_unit() {
        let forest1: ForestRoot<&str> = ForestRoot::empty();
        let forest2 = ForestRoot::unit(&["1", "2"]);

        assert_eq!(
            ForestRoot::unit(&["1", "2"]),
            ForestRoot::union(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_empty() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2: ForestRoot<&str> = ForestRoot::empty();

        assert_eq!(
            ForestRoot::unit(&["1", "2"]),
            ForestRoot::union(&forest1, &forest2)
        );
    }

    #[test]
    fn left_unit_right_unit() {
        let forest1 = ForestRoot::unit(&["1", "2"]);
        let forest2 = ForestRoot::unit(&["3", "4"]);

        assert_eq!(
            ForestRoot::many(&[
                vec!["1", "2"],
                vec!["3", "4"],
            ]),
            ForestRoot::union(&forest1, &forest2)
        );
    }
}
