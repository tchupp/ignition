use super::Node;

pub fn union(node1: Node, node2: Node) -> Node {
    if node1 == node2 {
        return node1;
    }

    let (id, low, high) = match (node1, node2) {
        (_, Node::Never) => return node1,
        (Node::Never, _) => return node2,

        (Node::Always, Node::Always) => return Node::Always,

        (Node::Branch(id, low, high), Node::Always) => {
            let low = union(low.into(), node2);
            let high = Node::from(high);

            (id, low, high)
        }
        (Node::Always, Node::Branch(id, low, high)) => {
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

    Node::branch(id, low, high)
}
