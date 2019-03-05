use std::fmt;
use std::sync::RwLock;

use self::arena::*;

mod arena;

#[macro_use]
mod macros;

mod intersect;
mod union;
mod product;
mod subset;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Priority(pub(crate) usize);

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
    Always,
    Never,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.fmt_inner(1))
    }
}

impl Node {
    fn fmt_inner(self, indent: usize) -> String {
        match self {
            Node::Branch(id, low, high) =>
                format!(
                    "{:?}:\n{}{}\n{}{}",
                    id,
                    "| ".repeat(indent),
                    Node::from(low).fmt_inner(indent + 1),
                    "| ".repeat(indent),
                    Node::from(high).fmt_inner(indent + 1)
                ),
            Node::Always => String::from("Always"),
            Node::Never => String::from("Never"),
        }
    }
}

impl Node {
    pub const NEVER: NodeId = NodeId(0);
    pub const ALWAYS: NodeId = NodeId(1);

    pub fn branch<L, H>(id: Priority, low: L, high: H) -> Self where L: Into<NodeId>, H: Into<NodeId> {
        let low = Node::from(low.into());
        let high = Node::from(high.into());

        match (high, low) {
            (Node::Never, _) => low,
            (Node::Branch(h_id, h_low, h_high), Node::Branch(l_id, l_low, l_high)) if h_id < id && l_id == h_id => {
                let low = Node::branch(id, l_low, h_low);
                let high = Node::branch(id, l_high, h_high);

                Node::branch(h_id, low, high)
            }

            (Node::Branch(h_id, h_low, h_high), _) if h_id < id => {
                let high = Node::branch(id, h_low, h_high);

                Node::branch(h_id, low, high)
            }
            (_, Node::Branch(l_id, l_low, l_high)) if l_id < id => {
                let low = Node::branch(id, l_low, high);

                Node::branch(l_id, low, l_high)
            }

            (Node::Branch(h_id, _, h_high), _) if h_id == id => Node::branch(id, low, h_high),
            (_, Node::Branch(l_id, l_low, _)) if l_id == id => Node::branch(id, l_low, high),

            _ => Node::Branch(id, low.into(), high.into())
        }
    }

    pub fn intersect(self, other: Self) -> Self {
        intersect::intersect(self, other)
    }

    pub fn union(self, other: Self) -> Self {
        union::union(self, other)
    }

    pub fn product(self, other: Self) -> Self {
        product::product(self, other)
    }

    pub fn subset(self, element: Priority) -> Self {
        subset::subset(self, element)
    }

    pub fn subset_all(self, elements: &[Priority]) -> Self {
        subset::subset_all(self, elements)
    }
}

impl<'a> From<Node> for NodeId {
    fn from(node: Node) -> Self {
        let node_id = {
            let guard = ARENA.read().unwrap();
            guard.get_by_node(&node).cloned()
        };

        node_id.unwrap_or_else(
            || ARENA.write().unwrap().add(node)
        )
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
    fn zdd_nodes_reduce_when_high_is_never_node() {
        {
            let node = node! {
                id: 0,
                low: node!(Always),
                high: node!(Always)
            };

            assert_eq!(
                Node::Branch(Priority(0), Node::ALWAYS, Node::ALWAYS),
                node
            );
        }
        {
            let node = node! {
                id: 0,
                low: node!(Always),
                high: node!(Never)
            };

            assert_eq!(
                Node::Always,
                node
            );
        }
        {
            let node = node! {
                id: 0,
                low: node!(Never),
                high: node!(Never)
            };

            assert_eq!(
                Node::Never,
                node
            );
        }
        {
            let node = node!(id: 0);

            assert_eq!(
                Node::Branch(Priority(0), Node::NEVER, Node::ALWAYS),
                node
            );
        }
    }

    #[test]
    fn zdd_nodes_reorder_when_low_has_higher_priority() {
        let initial = node! {
            id: 1,
            low: node!(id: 0),
            high: node!(id: 2)
        };

        let expected = node! {
            id: 0,
            low: node! {
                id: 1,
                low: node!(Never),
                high: node!(id: 2)
            },
            high: node!(Always)
        };

        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_reorder_when_high_has_higher_priority() {
        let initial = node! {
            id: 1,
            low: node!(id: 2),
            high: node!(id: 0)
        };

        let expected = node! {
            id: 0,
            low: node!(id: 2),
            high: node!(id: 1)
        };

        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_reorder_when_high_and_low_have_higher_priority() {
        let initial = node! {
            id: 1,
            low: node! {
                id: 0,
                low: node!(Never),
                high: node!(id: 2)
            },
            high: node! {
                id: 0,
                low: node!(id: 3),
                high: node!(Always)
            }
        };

        let expected = node! {
            id: 0,
            low: node! {
                id: 1,
                low: node!(Never),
                high: node!(id: 3)
            },
            high: node! {
                id: 1,
                low: node!(id: 2),
                high: node!(Always)
            }
        };


        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_reorder_when_empty_set_is_allowed() {
        let initial = node! {
            id: 1,
            low: node!(Always),
            high: node! {
                id: 0,
                low: node!(Never),
                high: node!(Always)
            }
        };

        let expected = node! {
            id: 0,
            low: node!(Always),
            high: node!(id: 1)
        };

        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_dedup_when_low_has_same_priority() {
        let initial = node! {
            id: 1,
            low: node! {
                id: 1,
                low: node!(Never),
                high: node!(Always)
            },
            high: node! {
                id: 2,
                low: node!(Never),
                high: node!(Always)
            }
        };

        let expected = node! {
            id: 1,
            low: node!(Never),
            high: node!(id: 2)
        };

        assert_eq!(
            expected,
            initial
        );
    }

    #[test]
    fn zdd_nodes_dedup_when_high_has_same_priority() {
        let initial = node! {
            id: 1,
            low: node!(Never),
            high: node! {
                id: 1,
                low: node!(Never),
                high: node!(id: 2)
            }
        };

        let expected = node! {
            id: 1,
            low: node!(Never),
            high: node!(id: 2)
        };

        assert_eq!(
            expected,
            initial
        );
    }
}