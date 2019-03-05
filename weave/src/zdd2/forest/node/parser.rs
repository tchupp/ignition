use std::str;

use nom::digit;
use nom::Err;

use super::Node;
use super::Priority;

const ALWAYS_NODE: &str = "(A)";
const NEVER_NODE: &str = "(N)";

pub fn build_node_string(node: impl Into<Node>) -> String {
    match node.into() {
        Node::Always => String::from(ALWAYS_NODE),
        Node::Never => String::from(NEVER_NODE),
        Node::Branch(id, low, high) => format!(
            "({:?} {} {})",
            id.0,
            build_node_string(low),
            build_node_string(high)
        ),
    }
}

pub fn parse_node_string(node_str: &str) -> Result<Node, Err<&[u8], u32>> {
    node(node_str.trim().as_bytes())
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
        map!(tag!(ALWAYS_NODE), |_: &[u8]| Node::Always) |
        map!(tag!(NEVER_NODE), |_: &[u8]| Node::Never) |
        do_parse!(
            tag!("(")        >>
            id: ws!(node_id) >>
            low: ws!(node)   >>
            high: ws!(node)  >>
            tag!(")")        >>
            (Node::branch(Priority(id), low, high))
        )
    )
);

#[cfg(test)]
mod tests {
    use super::Node;
    use super::parse_node_string;

    #[test]
    fn parse_leafs() {
        assert_eq!(
            Node::Always,
            parse_node_string("(A)").unwrap());

        assert_eq!(
            Node::Never,
            parse_node_string("(N)").unwrap());
    }

    #[test]
    fn parse_branch_no_whitespace() {
        assert_eq!(
            node! {id: 0, low: node!(Always), high: node!(Never)},
            parse_node_string("(0(A)(N))").unwrap());

        assert_eq!(
            node!(id: 0),
            parse_node_string("(0(N)(A))").unwrap());
    }

    #[test]
    fn parse_branch_with_whitespace() {
        assert_eq!(
            node!(id: 0),
            parse_node_string(" (  0 (N)   \n             (A) \t )").unwrap());
    }

    #[test]
    fn parse_branches_recursively() {
        assert_eq!(
            node! {
                id: 0,
                low: node!(Always),
                high: node!(id: 1)
            },
            parse_node_string("(0 (A)(1 (N)(A)))").unwrap());

        assert_eq!(
            node! {
                id: 0,
                low: node!(id: 1),
                high: node!(Always)
            },
            parse_node_string("(0 (1 (N) (A)) (A))").unwrap());
    }
}