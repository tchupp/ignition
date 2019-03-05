use super::Node;

pub fn intersect(node1: Node, node2: Node) -> Node {
    if node1 == node2 {
        return node1;
    }

    match (node1, node2) {
        (_, Node::Always) => node1,
        (Node::Always, _) => node2,

        (_, Node::Never) => Node::Never,
        (Node::Never, _) => Node::Never,

        (Node::Branch(id_1, low_1, _), Node::Branch(id_2, _, _)) if id_1 < id_2 =>
            intersect(low_1.into(), node2),

        (Node::Branch(id_1, _, _), Node::Branch(id_2, low_2, _)) if id_1 > id_2 =>
            intersect(node1, low_2.into()),

        (Node::Branch(id_1, low_1, high_1), Node::Branch(_, low_2, high_2)) => {
            let low = intersect(low_1.into(), low_2.into());
            let high = intersect(high_1.into(), high_2.into());

            Node::branch(id_1, low, high)
        }
    }
}
