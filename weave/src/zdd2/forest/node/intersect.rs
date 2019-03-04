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

        (Node::Branch(id_1, low_1, _), Node::Branch(id_2, _, _)) if id_1 < id_2 => {
            let low = intersect(low_1.into(), node2);
            let high = Node::Leaf(false);

            (id_1, low, high)
        }
        (Node::Branch(id_1, _, _), Node::Branch(id_2, low_2, _)) if id_1 > id_2 => {
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

    Node::branch(id, low, high)
}
