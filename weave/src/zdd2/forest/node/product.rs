use super::Node;

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

                Node::union(high_1_low_2, high_1_high_2)
            };

            let high = Node::union(low_1_high_2, new_high);

            (id_1, low_1_low_2, high)
        }
    };

    Node::branch(id, low, high)
}