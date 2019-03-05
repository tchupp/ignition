#[macro_export]
macro_rules! node {
    (Always) => {Node::Always};
    (Never) => {Node::Never};

    (id: $id:expr,) => {Node::branch(Priority($id), Node::Never, Node::Always)};
    (id: $id:expr) => {Node::branch(Priority($id), Node::Never, Node::Always)};

    (id: $id:expr, low: $low:expr,) => {Node::branch(Priority($id), $low, Node::Always)};
    (id: $id:expr, low: $low:expr) => {Node::branch(Priority($id), $low, Node::Always)};

    (id: $id:expr, low: $low:expr, high: $high:expr,) => {Node::branch(Priority($id), $low, $high)};
    (id: $id:expr, low: $low:expr, high: $high:expr) => {Node::branch(Priority($id), $low, $high)};
}

#[cfg(test)]
mod tests {
    use super::super::Node;
    use super::super::Priority;

    #[test]
    fn leaves() {
        {
            let node = node!(Always);

            assert_eq!(
                Node::Always,
                node
            );
        }
        {
            let node = node!(Never);

            assert_eq!(
                Node::Never,
                node
            );
        }
    }

    #[test]
    fn branches() {
        {
            let node = node! {
                id: 0,
                low: node!(Always)
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
            let node = node! {
                id: 0,
            };

            assert_eq!(
                Node::Branch(Priority(0), Node::NEVER, Node::ALWAYS),
                node
            );
        }
    }

    #[test]
    fn nested_branches() {
        let node = node! {
            id: 1,
            low: node!(Never),
            high: node!(id: 2)
        };

        let expected = {
            let high = Node::Branch(
                Priority(2),
                Node::NEVER,
                Node::ALWAYS,
            );

            Node::Branch(
                Priority(1),
                Node::NEVER,
                high.into(),
            )
        };

        assert_eq!(
            expected,
            node
        );
    }
}