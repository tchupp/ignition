use bdd::node::Node;
use core::Item;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Structure {
    Available(usize, Box<Structure>, Box<Structure>),
    Required(usize, Box<Structure>),
    Excluded(usize, Box<Structure>),
    Outcome(bool),
}

impl Structure {
    pub fn available<L: Into<Structure>, H: Into<Structure>>(depth: usize, low: L, high: H) -> Structure {
        let low = low.into();
        let high = high.into();

        Structure::Available(depth, Box::new(low), Box::new(high))
    }

    pub fn required<H: Into<Structure>>(depth: usize, high: H) -> Structure {
        let high = high.into();

        Structure::Required(depth, Box::new(high))
    }

    pub fn excluded<H: Into<Structure>>(depth: usize, low: H) -> Structure {
        let low = low.into();

        Structure::Excluded(depth, Box::new(low))
    }
}

impl From<Node> for Structure {
    fn from(node: Node) -> Self {
        to_structure(node, 0)
    }
}

impl<'a> From<&'a Node> for Structure {
    fn from(node: &Node) -> Self {
        Structure::from(node.clone())
    }
}

fn to_structure(node: Node, depth: usize) -> Structure {
    match node {
        Node::Leaf(val) => Structure::Outcome(val),
        Node::Branch(_id, low, high) => {
            let low = Node::from(low);
            let low = to_structure(low, depth + 1);

            let high = Node::from(high);
            let high = to_structure(high, depth + 1);

            if Structure::Outcome(false) == low {
                return Structure::required(depth, high);
            }
            if Structure::Outcome(false) == high {
                return Structure::excluded(depth, low);
            }

            Structure::available(depth, low, high)
        }
    }
}

pub type Content = BTreeMap<usize, Item>;

impl From<Node> for Content {
    fn from(node: Node) -> Self {
        let mut content = Content::new();
        to_content(node, &mut content, 0);
        content
    }
}

impl<'a> From<&'a Node> for Content {
    fn from(node: &Node) -> Self {
        Content::from(node.clone())
    }
}

fn to_content(node: Node, content: &mut Content, depth: usize) {
    if let Node::Branch(id, low, high) = node {
        content.insert(depth, id);

        let low = Node::from(low);
        to_content(low, content, depth + 1);

        let high = Node::from(high);
        to_content(high, content, depth + 1);
    }
}

impl From<(Structure, Content)> for Node {
    fn from((structure, content): (Structure, Content)) -> Self {
        to_node(structure, &content)
    }
}

fn to_node(structure: Structure, content: &Content) -> Node {
    match structure {
        Structure::Outcome(true) => Node::TRUE_LEAF,
        Structure::Outcome(false) => Node::FALSE_LEAF,
        Structure::Required(depth, high) => {
            let item = content.get(&depth).expect("expected");
            let low = Node::FALSE_LEAF;
            let high = to_node(*high, content);

            Node::branch(item, low, high)
        }
        Structure::Excluded(depth, low) => {
            let item = content.get(&depth).expect("expected");
            let low = to_node(*low, content);
            let high = Node::FALSE_LEAF;

            Node::branch(item, low, high)
        }
        Structure::Available(depth, low, high) => {
            let item = content.get(&depth).expect("expected");
            let low = to_node(*low, content);
            let high = to_node(*high, content);

            Node::branch(item, low, high)
        }
    }
}

#[cfg(test)]
mod tests {
    use bdd::node::Node;
    use core::Item;
    use super::Structure;

    #[test]
    fn structure_represents_available_items() {
        let blue = Item::new("shirts:blue");
        let jeans = Item::new("pants:jeans");

        let root = Node::branch(
            &blue,
            Node::positive_branch(&jeans),
            Node::negative_branch(&jeans),
        );

        let structure = Structure::from(root);

        assert_eq!(
            Structure::available(
                0,
                Structure::required(1, Structure::Outcome(true)),
                Structure::excluded(1, Structure::Outcome(true)),
            ),
            structure
        );
    }

    #[test]
    fn structure_represents_required_items() {
        let blue = Item::new("shirts:blue");
        let root = Node::positive_branch(&blue);

        let structure = Structure::from(root);

        assert_eq!(
            Structure::required(0, Structure::Outcome(true)),
            structure
        );
    }

    #[test]
    fn structure_represents_excluded_items() {
        let blue = Item::new("shirts:blue");
        let root = Node::negative_branch(&blue);

        let structure = Structure::from(root);

        assert_eq!(
            Structure::excluded(0, Structure::Outcome(true)),
            structure
        );
    }
}
