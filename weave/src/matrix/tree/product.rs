use std::hash::Hash;

use hashbrown::HashSet;

use super::Forest;
use super::Tree;

pub fn product<T: Hash + Eq + Clone + Ord + Sync + Send>(tree1: Tree<T>, tree2: Tree<T>) -> Forest<T> {
    match (tree1, tree2) {
        (_, Tree::Empty) => Forest::empty(),
        (Tree::Empty, _) => Forest::empty(),

        (Tree::One(element1), Tree::One(element2)) => Forest::unit(&[element1, element2]),

        (Tree::One(element), Tree::Many(set)) => one_to_many(element, set),
        (Tree::Many(set), Tree::One(element)) => one_to_many(element, set),

        (Tree::Many(set1), Tree::Many(set2)) => many_to_many(set1, set2),
    }
}

fn one_to_many<T: Hash + Clone + Ord + Sync + Send>(a: T, set: HashSet<T>) -> Forest<T> {
    let forest: Vec<Vec<T>> = set.into_iter()
        .map(|b| vec![a.clone(), b])
        .collect();

    Forest::many(&forest)
}

fn many_to_many<T: Hash + Clone + Ord + Sync + Send>(set1: HashSet<T>, set2: HashSet<T>) -> Forest<T> {
    let forest: Vec<Vec<T>> = set1.into_iter()
        .flat_map(|element1|
            set2.iter()
                .map(|element2| vec![element1.clone(), element2.clone()])
                .collect::<Vec<_>>()
        )
        .collect();

    Forest::many(&forest)
}

#[cfg(test)]
mod tests {
    use super::Forest;
    use super::Tree;

    #[test]
    fn left_side_empty() {
        let tree1 = Tree::<&str>::empty();
        let tree2 = Tree::one("1");

        assert_eq!(
            Forest::<&str>::empty(),
            Tree::product(tree1, tree2)
        );
    }

    #[test]
    fn right_side_empty() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Tree::product(tree1, tree2)
        );
    }

    #[test]
    fn returns_unit_when_trees_are_disjoint() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::one("2");

        assert_eq!(
            Forest::unit(&["1", "2"]),
            Tree::product(tree1, tree2)
        );
    }

    #[test]
    fn product_returns_identity_when_both_trees_are_empty() {
        let tree1 = Tree::<&str>::empty();
        let tree2 = Tree::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Tree::product(tree1, tree2)
        );
    }

    #[test]
    fn product_returns_identity_when_trees_are_equal_one() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::one("1");

        assert_eq!(
            Forest::unit(&["1"]),
            Tree::product(tree1, tree2)
        );
    }

    #[test]
    fn product_returns_many_when_trees_are_equal_many() {
        let tree1 = Tree::many(&["1", "2"]);
        let tree2 = Tree::many(&["1", "2"]);

        assert_eq!(
            Forest::many(&[
                vec!["1"],
                vec!["1", "2"],
                vec!["2"],
            ]),
            Tree::product(tree1, tree2)
        );
    }

    #[test]
    fn product_returns_many_trees_of_2() {
        let tree1 = Tree::many(&["2", "3"]);
        let tree2 = Tree::many(&["1", "2"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["1", "3"],
                vec!["2", "3"],
                vec!["2"],
            ]),
            Tree::product(tree1, tree2)
        );
    }

    #[test]
    fn product_returns_many_trees_of_3() {
        let tree1 = Tree::many(&["1", "2", "3"]);
        let tree2 = Tree::many(&["2", "3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
                vec!["1", "3"],
                vec!["1", "4"],
                vec!["2"],
                vec!["2", "3"],
                vec!["2", "4"],
                vec!["3"],
                vec!["3", "4"],
            ]),
            Tree::product(tree1, tree2)
        );
    }
}