pub mod intersect;
pub mod product;
pub mod subset;
pub mod subset_not;
pub mod subset_all;
pub mod subset_none;
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
            intersect!($forest, both_forests_are_empty);

            intersect!($forest, left_is_empty_right_is_unit);

            intersect!($forest, left_is_empty_right_is_many);

            intersect!($forest, left_is_unit_right_is_empty);

            intersect!($forest, forests_are_equal_unit);

            intersect!($forest, forests_are_disjoint_units);

            intersect!($forest, left_is_unit_right_is_many);

            intersect!($forest, forests_are_equal_many);

            intersect!($forest, forests_are_disjoint_many);

            intersect!($forest, forests_are_have_single_commonality);

            intersect!($forest, forests_are_have_multiple_commonality);
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
            union!($forest, both_forests_are_empty);

            union!($forest, left_is_empty_right_is_unit);

            union!($forest, left_is_empty_right_is_many);

            union!($forest, forests_are_equal_unit);

            union!($forest, forests_are_disjoint_units);

            union!($forest, left_is_unit_right_is_many);

            union!($forest, forests_are_equal_many);

            union!($forest, forests_are_disjoint_many);

            union!($forest, forests_are_have_commonality);

            union!($forest, left_is_unit_right_is_many_overlapping);
        }
    };
}

macro_rules! product {
    ($forest:ty, $test_case:ident) => {
        spec!($forest, $test_case, product);
    };
}

macro_rules! product_tests {
    ($forest:ty) => {

        #[cfg(test)]
        mod product_tests {
            product!($forest, left_is_empty_right_is_unit);

            product!($forest, overlapping_unit_forests);

            product!($forest, overlapping_many_forest_and_double_unit_forest);

            product!($forest, overlapping_many_forest_and_unique_forest);

            product!($forest, disjoint_unit_forest_and_single_unit_forest);

            product!($forest, disjoint_unit_forest_and_double_unit_forest);

            product!($forest, disjoint_many_forest_and_single_unit_forest);

            product!($forest, disjoint_many_forest_and_unique_forest);

            product!($forest, disjoint_many_forest_and_double_unit_forest);

            product!($forest, disjoint_unique_forests);

            product!($forest, forests_are_disjoint_many);
        }
    };
}

macro_rules! subset {
    ($forest:ty, $test_case:ident) => {

        #[test]
        fn $test_case() {
            let (forest, element, expected) = $crate::forest::subset::$test_case::<$forest>();

            assert_eq!(
                expected,
                <$forest>::subset(forest, element)
            );
        }
    };
}

macro_rules! subset_not {
    ($forest:ty, $test_case:ident) => {

        #[test]
        fn $test_case() {
            let (forest, element, expected) = $crate::forest::subset_not::$test_case::<$forest>();

            assert_eq!(
                expected,
                <$forest>::subset_not(forest, element)
            );
        }
    };
}

macro_rules! subset_all {
    ($forest:ty, $test_case:ident) => {

        #[test]
        fn $test_case() {
            let (forest, elements, expected) = $crate::forest::subset_all::$test_case::<$forest>();

            assert_eq!(
                expected,
                <$forest>::subset_all(forest, &elements)
            );
        }
    };
}

macro_rules! subset_none {
    ($forest:ty, $test_case:ident) => {

        #[test]
        fn $test_case() {
            let (forest, elements, expected) = $crate::forest::subset_none::$test_case::<$forest>();

            assert_eq!(
                expected,
                <$forest>::subset_none(forest, &elements)
            );
        }
    };
}

macro_rules! subset_tests {
    ($forest:ty) => {

        #[cfg(test)]
        mod subset_tests {
            subset!($forest, empty_forest);

            subset!($forest, unit_forest_with_disjoint_element);

            subset!($forest, many_forest_with_disjoint_element);

            subset!($forest, unit_forest_with_matching_element);

            subset!($forest, many_forest_with_matching_element_1);

            subset!($forest, many_forest_with_matching_element_2);
        }

        #[cfg(test)]
        mod subset_not_tests {
            subset_not!($forest, empty_forest);

            subset_not!($forest, unit_forest_with_disjoint_element);

            subset_not!($forest, many_forest_with_disjoint_element);

            subset_not!($forest, unit_forest_with_matching_element);

            subset_not!($forest, many_forest_with_matching_element_1);

            subset_not!($forest, many_forest_with_matching_element_2);
        }

        #[cfg(test)]
        mod subset_all_tests {
            subset_all!($forest, empty_forest_with_single_element);

            subset_all!($forest, unit_forest_with_empty_elements);

            subset_all!($forest, unit_forest_with_one_element);

            subset_all!($forest, unit_forest_with_disjoint_elements);

            subset_all!($forest, many_forest_with_one_element);

            subset_all!($forest, many_forest_with_many_elements);
        }

        #[cfg(test)]
        mod subset_none_tests {
            subset_none!($forest, empty_forest_with_single_element);

            subset_none!($forest, unit_forest_with_empty_elements);

            subset_none!($forest, unit_forest_with_one_element);

            subset_none!($forest, unit_forest_with_disjoint_elements);

            subset_none!($forest, many_forest_with_one_element);

            subset_none!($forest, many_forest_with_many_elements);
        }
    };
}

macro_rules! spec {
    ($forest:ty, $test_case:ident, $module:ident) => {

        #[test]
        fn $test_case() {
            let (forest1, forest2, expected) = $crate::forest::$module::$test_case::<$forest>();

            assert_eq!(
                expected,
                <$forest>::$module(forest1.clone(), forest2.clone())
            );

            assert_eq!(
                expected,
                <$forest>::$module(forest2.clone(), forest1.clone())
            );
        }
    };
}