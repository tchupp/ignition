pub mod intersect;
pub mod union;

macro_rules! intersect {
    ($forest:ty, $test_case:ident) => {
        spec!($forest, $test_case, intersect);
    };
}

macro_rules! intersect_tests {
    ($forest:ty) => {

        #[cfg(test)]
        mod intersect_tests {
            intersect!($forest, both_trees_are_empty);

            intersect!($forest, left_is_empty_right_is_unit);

            intersect!($forest, left_is_empty_right_is_many);

            intersect!($forest, left_is_unit_right_is_empty);

            intersect!($forest, trees_are_equal_unit);

            intersect!($forest, trees_are_disjoint_units);

            intersect!($forest, left_is_unit_right_is_many);

            intersect!($forest, trees_are_equal_many);

            intersect!($forest, trees_are_disjoint_many);

            intersect!($forest, trees_are_have_single_commonality);

            intersect!($forest, trees_are_have_multiple_commonality);
        }
    };
}

macro_rules! union {
    ($forest:ty, $test_case:ident) => {
        spec!($forest, $test_case, union);
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

macro_rules! spec {
    ($forest:ty, $test_case:ident, $module:ident) => {

        #[test]
        fn $test_case() {
            let (tree1, tree2, expected) = $crate::forest::$module::$test_case::<$forest>();

            assert_eq!(
                expected,
                <$forest>::$module(tree1.clone(), tree2.clone())
            );

            assert_eq!(
                expected,
                <$forest>::$module(tree2.clone(), tree1.clone())
            );
        }
    };
}