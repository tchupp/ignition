use super::intersect;
use super::Node;
use super::Priority;

type Matching = (Node, bool);

pub fn subset(root: Node, element: Priority) -> Node {
    subset_inner(root, element).0
}

fn subset_inner(root: Node, element: Priority) -> Matching {
    match root {
        Node::Leaf(_) => (root, false),
        Node::Branch(id, _low, high) if element == id => {
            let low = Node::FALSE;

            (Node::branch(id, low, high), true)
        }
        Node::Branch(id, low, high) => {
            let (low, keep_low) = reduce_branch(
                subset_inner(low.into(), element)
            );
            let (high, keep_high) = reduce_branch(
                subset_inner(high.into(), element)
            );

            let keep = keep_low || keep_high;

            (Node::branch(id, low, high), keep)
        }
    }
}

pub fn subset_many(root: Node, elements: &[Priority]) -> Node {
    elements.iter()
        .map(|element| subset(root, element.to_owned()))
        .fold(Node::Leaf(true), intersect::intersect)
}

fn reduce_branch((root, keep): Matching) -> Matching {
    match (root, keep) {
        (_root, false) => (Node::Leaf(false), keep),
        (root, true) => (root, keep)
    }
}

#[cfg(test)]
mod subset_tests {
    use super::super::Forest;

    #[test]
    fn subset_of_empty_returns_empty() {
        let forest: Forest<&str> = Forest::empty();
        let element = "1";

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_disjoint_unit_returns_empty() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let element = "1";

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_disjoint_many_returns_empty() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"]
        ]);
        let element = "4";

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_containing_returns_identity() {
        let forest: Forest<&str> = Forest::unit(&["1", "3"]);
        let element = "1";

        assert_eq!(
            Forest::unit(&["1", "3"]),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_containing_returns_unit_1() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"]
        ]);
        let element = "1";

        assert_eq!(
            Forest::unit(&["1", "3"]),
            Forest::subset(forest, element)
        );
    }

    #[test]
    fn subset_of_containing_returns_unit_2() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"]
        ]);
        let element = "2";

        assert_eq!(
            Forest::unit(&["2", "3"]),
            Forest::subset(forest, element)
        );
    }
}

#[cfg(test)]
mod subset_many_tests {
    use super::super::Forest;

    #[test]
    fn subset_many_of_empty_returns_empty() {
        let forest: Forest<&str> = Forest::empty();
        let elements = &["1"];

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_empty_elements_returns_identity() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let elements = &[];

        assert_eq!(
            Forest::unit(&["2", "3"]),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_one_element_returns_identity() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let elements = &["2"];

        assert_eq!(
            Forest::unit(&["2", "3"]),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_disjoint_elements_returns_empty() {
        let forest: Forest<&str> = Forest::unit(&["2", "3"]);
        let elements = &["1", "2"];

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_one_element_returns_many() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"],
            vec!["1", "2"]
        ]);
        let elements = &["3"];

        assert_eq!(
            Forest::many(&[
                vec!["1", "3"],
                vec!["2", "3"],
            ]),
            Forest::subset_many(forest, elements)
        );
    }

    #[test]
    fn subset_many_with_many_elements_returns_unit() {
        let forest: Forest<&str> = Forest::many(&[
            vec!["1", "3"],
            vec!["2", "3"],
            vec!["1", "2"]
        ]);
        let elements = &["1", "3"];

        assert_eq!(
            Forest::unit(&["1", "3"]),
            Forest::subset_many(forest, elements)
        );
    }
}
