use std::hash::Hash;

use rayon::iter::ParallelIterator;

use super::Tree;

pub fn union<T: Hash + Clone + Ord + Sync + Send>(tree1: Tree<T>, tree2: Tree<T>) -> Tree<T> {
    if tree1 == tree2 {
        return tree1;
    }

    match (&tree1, &tree2) {
        (_, Tree::Empty) => tree1.clone(),
        (Tree::Empty, _) => tree2.clone(),

        (Tree::One(element1), Tree::One(element2)) =>
            Tree::many(&[element1.clone(), element2.clone()]),

        (Tree::Many(set), Tree::One(element)) if set.contains(element) => tree1.clone(),
        (Tree::One(element), Tree::Many(set)) if set.contains(element) => tree2.clone(),

        (Tree::Many(set1), Tree::Many(set2)) if set1.par_is_subset(set2) => tree2.clone(),
        (Tree::Many(set1), Tree::Many(set2)) if set1.par_is_superset(set2) => tree1.clone(),

        (Tree::Many(set1), Tree::Many(set2)) =>
            Tree::many(&set1.par_union(set2).cloned().collect::<Vec<_>>()),

        (_, _) => Tree::Empty,
    }
}

#[cfg(test)]
mod subset_tests {
    use super::Tree;

    #[test]
    fn union_returns_right_when_left_is_one_subset() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::many(&["1", "2"]);

        assert_eq!(
            Tree::many(&["1", "2"]),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_left_when_right_is_one_subset() {
        let tree1 = Tree::many(&["1", "2"]);
        let tree2 = Tree::one("2");

        assert_eq!(
            Tree::many(&["1", "2"]),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_left_when_left_is_many_subset() {
        let tree1 = Tree::many(&["1", "2"]);
        let tree2 = Tree::many(&["1", "2", "3"]);

        assert_eq!(
            Tree::many(&["1", "2", "3"]),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_right_when_right_is_many_subset() {
        let tree1 = Tree::many(&["1", "2", "3"]);
        let tree2 = Tree::many(&["1", "2"]);

        assert_eq!(
            Tree::many(&["1", "2", "3"]),
            Tree::union(tree1, tree2)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn left_side_empty() {
        let tree1 = Tree::<&str>::empty();
        let tree2 = Tree::one("1");

        assert_eq!(
            Tree::one("1"),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn right_side_empty() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::<&str>::empty();

        assert_eq!(
            Tree::one("1"),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn returns_all_when_trees_are_disjoint() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::one("2");

        assert_eq!(
            Tree::many(&["1", "2"]),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_identity_when_trees_are_equal() {
        let tree1 = Tree::<&str>::empty();
        let tree2 = Tree::<&str>::empty();

        assert_eq!(
            Tree::<&str>::empty(),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_identity_when_trees_are_equal_one() {
        let tree1 = Tree::one("1");
        let tree2 = Tree::one("1");

        assert_eq!(
            Tree::one("1"),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_identity_when_trees_are_equal_many() {
        let tree1 = Tree::many(&["1", "2"]);
        let tree2 = Tree::many(&["1", "2"]);

        assert_eq!(
            Tree::many(&["1", "2"]),
            Tree::union(tree1, tree2)
        );
    }

    #[test]
    fn union_returns_many() {
        let tree1 = Tree::many(&["2", "3"]);
        let tree2 = Tree::many(&["1", "2"]);

        assert_eq!(
            Tree::many(&["1", "2", "3"]),
            Tree::union(tree1, tree2)
        );
    }
}

