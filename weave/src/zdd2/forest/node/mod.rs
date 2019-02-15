use std::fmt;
use std::sync::RwLock;

use super::root::Priority;

use self::arena::*;

mod arena;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NodeId(usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", Node::from(self))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Node {
    Branch(Priority, NodeId, NodeId),
    Leaf(bool),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.fmt_inner(1))
    }
}

impl Node {
    fn fmt_inner(&self, indent: usize) -> String {
        match self {
            Node::Leaf(val) => format!("{}", val),
            Node::Branch(id, low, high) =>
                format!(
                    "{:?}:\n{}{}\n{}{}",
                    id,
                    "| ".repeat(indent),
                    Node::from(low).fmt_inner(indent + 1),
                    "| ".repeat(indent),
                    Node::from(high).fmt_inner(indent + 1)
                ),
        }
    }
}

impl Node {
    pub const FALSE: NodeId = NodeId(0);
    pub const TRUE: NodeId = NodeId(1);

    pub fn branch<L, H>(id: Priority, low: L, high: H) -> Self where L: Into<NodeId>, H: Into<NodeId> {
        let low = Node::from(low.into());
        let high = Node::from(high.into());

        match (high, low) {
            (Node::Leaf(false), _) => {
                return low;
            }
            (Node::Branch(h_id, h_low, h_high), _) if h_id < id => {
                let high = Node::branch(id, h_low, h_high);
                return Node::branch(h_id, low, high);
            }
            (_, Node::Branch(l_id, l_low, l_high)) if l_id < id => {
                let low = Node::branch(id, l_low, high);
                return Node::branch(l_id, low, l_high);
            }
            (Node::Branch(h_id, _, h_high), _) if h_id == id => {
                return Node::branch(id, low, h_high);
            }
            (_, Node::Branch(l_id, l_low, _)) if l_id == id => {
                return Node::branch(id, l_low, high);
            }
            _ => {}
        }

        Node::Branch(id, low.into(), high.into())
    }
}

impl<'a> From<Node> for NodeId {
    fn from(node: Node) -> Self {
        let node_id = {
            let guard = ARENA.read().unwrap();
            guard.get_by_node(&node).cloned()
        };
        match node_id {
            Some(node_id) => return node_id,
            None => ARENA.write().unwrap().add(node),
        }
    }
}

impl<'a> From<&'a Node> for NodeId {
    fn from(node: &Node) -> Self {
        NodeId::from(*node)
    }
}

impl From<NodeId> for Node {
    fn from(node_id: NodeId) -> Self {
        *ARENA.read().unwrap()
            .get_by_id(node_id)
            .unwrap_or_else(|| panic!("Expected node to exist for: {:?}", node_id))
    }
}

impl<'a> From<&'a NodeId> for Node {
    fn from(node_id: &NodeId) -> Self {
        Node::from(*node_id)
    }
}

impl<'a> From<&'a Node> for Node {
    fn from(node: &Node) -> Self {
        *node
    }
}

lazy_static! {
    pub static ref ARENA: RwLock<NodeArena> = {
        let a = NodeArena::new();
        RwLock::new(a)
    };
}

#[cfg(test)]
mod tests {
    use super::Node;
    use super::Priority;

    #[test]
    fn zdd_nodes_reduce_when_high_is_false() {
        {
            let node = Node::branch(
                Priority(0),
                Node::TRUE,
                Node::TRUE,
            );

            assert_eq!(
                Node::Branch(Priority(0), Node::TRUE, Node::TRUE),
                node
            );
        }
        {
            let node = Node::branch(
                Priority(0),
                Node::TRUE,
                Node::FALSE,
            );

            assert_eq!(
                Node::Leaf(true),
                node
            );
        }
        {
            let node = Node::branch(
                Priority(0),
                Node::FALSE,
                Node::FALSE,
            );

            assert_eq!(
                Node::Leaf(false),
                node
            );
        }
        {
            let node = Node::branch(
                Priority(0),
                Node::FALSE,
                Node::TRUE,
            );

            assert_eq!(
                Node::Branch(Priority(0), Node::FALSE, Node::TRUE),
                node
            );
        }
    }

    #[test]
    fn zdd_nodes_reorder_when_low_has_higher_priority() {
        let initial = {
            let low = Node::branch(
                Priority(0),
                Node::FALSE,
                Node::TRUE,
            );
            let high = Node::branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );
            let initial = Node::branch(
                Priority(1),
                low,
                high,
            );

            initial
        };

        let expected = {
            let expected_high = Node::Branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );
            let expected_low = Node::Branch(
                Priority(1),
                Node::FALSE,
                expected_high.into(),
            );
            let expected = Node::Branch(
                Priority(0),
                expected_low.into(),
                Node::TRUE,
            );

            expected
        };


        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_reorder_when_high_has_higher_priority() {
        let initial = {
            let low = Node::branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );
            let high = Node::branch(
                Priority(0),
                Node::FALSE,
                Node::TRUE,
            );
            let initial = Node::branch(
                Priority(1),
                low,
                high,
            );

            initial
        };

        let expected = {
            let expected_low = Node::Branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );
            let expected_high = Node::Branch(
                Priority(1),
                Node::FALSE,
                Node::TRUE,
            );
            let expected = Node::Branch(
                Priority(0),
                expected_low.into(),
                expected_high.into(),
            );

            expected
        };


        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_reorder_when_empty_set_is_allowed() {
        let initial = {
            let high = Node::branch(
                Priority(0),
                Node::FALSE,
                Node::TRUE,
            );
            let initial = Node::branch(
                Priority(1),
                Node::TRUE,
                high,
            );

            initial
        };

        let expected = {
            let high = Node::Branch(
                Priority(1),
                Node::FALSE,
                Node::TRUE,
            );

            Node::Branch(
                Priority(0),
                Node::TRUE,
                high.into(),
            )
        };

        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_dedup_when_low_has_same_priority() {
        let initial = {
            let low = Node::branch(
                Priority(1),
                Node::FALSE,
                Node::TRUE,
            );
            let high = Node::branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );
            let initial = Node::branch(
                Priority(1),
                low,
                high,
            );

            initial
        };

        let expected = {
            let high = Node::Branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );

            Node::Branch(
                Priority(1),
                Node::FALSE,
                high.into(),
            )
        };

        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_dedup_when_high_has_same_priority() {
        let initial = {
            let high2 = Node::branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );
            let high1 = Node::branch(
                Priority(1),
                Node::FALSE,
                high2,
            );
            let initial = Node::branch(
                Priority(1),
                Node::FALSE,
                high1,
            );

            initial
        };

        let expected = {
            let high = Node::Branch(
                Priority(2),
                Node::FALSE,
                Node::TRUE,
            );

            Node::Branch(
                Priority(1),
                Node::FALSE,
                high.into(),
            )
        };

        assert_eq!(
            expected,
            initial
        );
    }
}