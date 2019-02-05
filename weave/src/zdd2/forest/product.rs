use std::hash::Hash;

use hashbrown::HashSet;

use zdd2::Forest;
use zdd2::Tree;

pub fn product<T: Hash + Eq + Clone + Ord + Sync + Send>(forest: Forest<T>, tree: Tree<T>) -> Forest<T> {
    match (forest, tree) {
        (_, Tree::Empty) => Forest::empty(),
        (Forest::Empty, _) => Forest::empty(),

        (Forest::Unit(set), Tree::One(element)) => one_to_set(set, element),

        (Forest::Unit(set1), Tree::Many(set2)) => Forest::many(&many_to_many(set1, &set2)),

        (Forest::Many(matrix), Tree::One(element)) => one_to_matrix(matrix, element),

        (Forest::Many(matrix), Tree::Many(set)) => many_to_matrix(matrix, set),

        (_, _) => Forest::empty(),
    }
}

fn one_to_set<T: Hash + Clone + Ord + Sync + Send>(set: Vec<T>, element: T) -> Forest<T> {
    let set: Vec<T> = set.into_iter()
        .chain(vec![element.clone()])
        .collect();

    Forest::unit(&set)
}

fn one_to_matrix<T: Hash + Clone + Ord + Sync + Send>(matrix: HashSet<Vec<T>>, element: T) -> Forest<T> {
    let matrix: Vec<Vec<T>> = matrix.into_iter()
        .map(|set| set.into_iter()
            .chain(vec![element.clone()])
            .collect::<Vec<_>>()
        )
        .collect();

    Forest::many(&matrix)
}

fn many_to_many<T: Hash + Clone + Ord + Sync + Send>(set1: Vec<T>, set2: &HashSet<T>) -> Vec<Vec<T>> {
    set2.iter()
        .map(|element| set1.iter()
            .cloned()
            .chain(vec![element.clone()])
            .collect::<Vec<_>>()
        )
        .collect()
}

fn many_to_matrix<T: Hash + Clone + Ord + Sync + Send>(matrix: HashSet<Vec<T>>, set2: HashSet<T>) -> Forest<T> {
    let matrix: Vec<Vec<T>> = matrix.into_iter()
        .flat_map(|set1| many_to_many(set1, &set2))
        .collect();

    Forest::many(&matrix)
}

#[cfg(test)]
mod tests {
    use zdd2::Forest;
    use zdd2::Tree;

    #[test]
    fn empty_tree_returns_empty() {
        let forest = Forest::unit(&["1", "2"]);
        let tree = Tree::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::product(forest, tree)
        );
    }

    #[test]
    fn empty_forest_returns_empty() {
        let forest = Forest::<&str>::empty();
        let tree = Tree::one("1");

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::product(forest, tree)
        );
    }

    #[test]
    fn unit_forest_and_unit_tree_with_overlap_returns_many() {
        let forest = Forest::unit(&["1", "2"]);
        let tree = Tree::one("1");

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
            ]),
            Forest::product(forest, tree)
        );
    }

    #[test]
    fn unit_forest_and_unit_tree_disjoint_returns_many() {
        let forest = Forest::unit(&["1", "2"]);
        let tree = Tree::one("3");

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
            ]),
            Forest::product(forest, tree)
        );
    }

    #[test]
    fn unit_forest_and_many_tree_returns_many() {
        let forest = Forest::unit(&["1", "2"]);
        let tree = Tree::many(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
            ]),
            Forest::product(forest, tree)
        );
    }

    #[test]
    fn many_forest_and_unit_tree_returns_many() {
        let forest = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let tree = Tree::one("4");

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "4"],
                vec!["2", "3", "4"]
            ]),
            Forest::product(forest, tree)
        );
    }

    #[test]
    fn many_forest_and_many_tree_returns_many() {
        let forest = Forest::many(&[
            vec!["1", "2"],
            vec!["5", "6"]
        ]);
        let tree = Tree::many(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
                vec!["5", "6", "3"],
                vec!["5", "6", "4"],
            ]),
            Forest::product(forest, tree)
        );
    }
}