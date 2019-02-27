pub mod union;

macro_rules! union {
    ($forest:ty, $name:ident) => {

        #[test]
        fn $name() {
            let (tree1, tree2, expected) = $crate::forest::union::$name::<$forest>();

            assert_eq!(
                expected,
                <$forest>::union(tree1.clone(), tree2.clone())
            );

            assert_eq!(
                expected,
                <$forest>::union(tree2.clone(), tree1.clone())
            );
        }
    };
}

macro_rules! union_tests {
    ($forest:ty) => {

        #[cfg(test)]
        mod union_tests {
            union!($forest, both_trees_are_empty);

            union!($forest, left_is_empty_right_is_unit);

            union!($forest, left_is_empty_right_is_many);

            union!($forest, trees_are_equal_unit);

            union!($forest, trees_are_disjoint_units);

            union!($forest, left_is_unit_right_is_many);

            union!($forest, trees_are_equal_many);

            union!($forest, trees_are_disjoint_many);

            union!($forest, trees_are_have_commonality);

            union!($forest, left_is_unit_right_is_many_overlapping);
        }
    };
}