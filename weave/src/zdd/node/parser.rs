use nom::digit;
use nom::Err;
use std::str;
use zdd::node::Node;

const TRUE_NODE: &str = "(T)";
const FALSE_NODE: &str = "(F)";

pub(crate) fn build_node_string(node: Node) -> String {
    match node {
        Node::Leaf(true) => String::from(TRUE_NODE),
        Node::Leaf(false) => String::from(FALSE_NODE),
        Node::Branch(id, low, high) => format!(
            "({:?} {} {})",
            id,
            build_node_string(Node::from(low)),
            build_node_string(Node::from(high))
        ),
    }
}

pub(crate) fn parse_node_string(node_str: &str) -> Result<Node, Err<&[u8], u32>> {
    node(node_str.as_bytes())
        .map(|(_, node)| node)
}

named!(node_id<usize>,
    complete!(
        map_res!(
            map_res!(digit, str::from_utf8),
            |s: &str| s.parse::<usize>()
        )
    )
);

named!(node<Node>,
    alt!(
        map!(tag!(TRUE_NODE), |_: &[u8]| Node::Leaf(true)) |
        map!(tag!(FALSE_NODE), |_: &[u8]| Node::Leaf(false)) |
        do_parse!(
            tag!("(")        >>
            id: ws!(node_id) >>
            low: ws!(node)   >>
            high: ws!(node)  >>
            tag!(")")        >>
            (Node::branch(id, low, high))
        )
    )
);

#[cfg(test)]
mod tests {
    use super::parse_node_string;
    use zdd::node::Node;

    #[test]
    fn test_parse_leafs() {
        assert_eq!(
            Ok(Node::Leaf(true)),
            parse_node_string("(T)"));
        assert_eq!(
            Ok(Node::Leaf(false)),
            parse_node_string("(F)"));
    }

    #[test]
    fn test_parse_branch_no_whitespace() {
        assert_eq!(
            Ok(
                Node::branch(0,
                             Node::Leaf(true),
                             Node::Leaf(false))),
            parse_node_string("(0(T)(F))"));
    }

    #[test]
    fn test_parse_branch_with_whitespace() {
        assert_eq!(
            Ok(
                Node::branch(0,
                             Node::Leaf(true),
                             Node::Leaf(false))),
            parse_node_string("(  0 (T)   \n             (F) \t )"));
    }

    #[test]
    fn test_parse_branches_recursively() {
        assert_eq!(
            Ok(
                Node::branch(0,
                             Node::Leaf(true),
                             Node::branch(1,
                                          Node::Leaf(true),
                                          Node::Leaf(false)))),
            parse_node_string("(0 (T)(1 (T)(F)))"));
        assert_eq!(
            Ok(
                Node::branch(0,
                             Node::branch(1,
                                          Node::Leaf(true),
                                          Node::Leaf(false)),
                             Node::Leaf(true))),
            parse_node_string("(0 (1 (T) (F)) (T))"));
    }
}