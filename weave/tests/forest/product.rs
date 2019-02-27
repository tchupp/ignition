use std::fmt::Debug;

use weave::Forest;

pub fn left_is_empty_right_is_unit<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["1", "2"]);
    let forest2 = F::empty();

    let expected = F::empty();

    (forest1, forest2, expected)
}

pub fn overlapping_unit_forests<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["1", "2"]);
    let forest2 = F::unit(&["1"]);

    let expected = F::many(&[
        vec!["1", "2"],
    ]);

    (forest1, forest2, expected)
}

pub fn overlapping_many_forest_and_double_unit_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);
    let forest2 = F::unit(&["3", "4"]);

    let expected = F::many(&[
        vec!["1", "2", "3", "4"],
        vec!["2", "3", "4"]
    ]);

    (forest1, forest2, expected)
}

pub fn overlapping_many_forest_and_unique_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);
    let forest2 = F::unique(&["3", "4"]);

    let expected = F::many(&[
        vec!["1", "2", "3"],
        vec!["1", "2", "4"],
        vec!["2", "3"],
        vec!["2", "3", "4"]
    ]);

    (forest1, forest2, expected)
}

pub fn disjoint_unit_forest_and_single_unit_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["1", "2"]);
    let forest2 = F::unit(&["3"]);

    let expected = F::many(&[
        vec!["1", "2", "3"],
    ]);

    (forest1, forest2, expected)
}

pub fn disjoint_unit_forest_and_double_unit_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["1", "2"]);
    let forest2 = F::unit(&["3", "4"]);

    let expected = F::unit(&["1", "2", "3", "4"]);

    (forest1, forest2, expected)
}

pub fn disjoint_many_forest_and_single_unit_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);
    let forest2 = F::unit(&["4"]);

    let expected = F::many(&[
        vec!["1", "2", "4"],
        vec!["2", "3", "4"]
    ]);

    (forest1, forest2, expected)
}

pub fn disjoint_many_forest_and_unique_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["5", "6"]
    ]);
    let forest2 = F::unique(&["3", "4"]);

    let expected = F::many(&[
        vec!["1", "2", "3"],
        vec!["1", "2", "4"],
        vec!["5", "6", "3"],
        vec!["5", "6", "4"],
    ]);

    (forest1, forest2, expected)
}

pub fn disjoint_many_forest_and_double_unit_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["5", "6"]
    ]);
    let forest2 = F::unit(&["3", "4"]);

    let expected = F::many(&[
        vec!["1", "2", "3", "4"],
        vec!["5", "6", "3", "4"],
    ]);

    (forest1, forest2, expected)
}

pub fn disjoint_unique_forests<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unique(&["1", "2"]);
    let forest2 = F::unique(&["3", "4"]);

    let expected = F::many(&[
        vec!["1", "3"],
        vec!["1", "4"],
        vec!["2", "3"],
        vec!["2", "4"],
    ]);

    (forest1, forest2, expected)
}

pub fn forests_are_disjoint_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unique(&["1", "2"]);
    let forest2 = F::many(&[
        vec!["3", "4"],
        vec!["7", "8"]
    ]);

    let expected = F::many(&[
        vec!["1", "3", "4"],
        vec!["1", "7", "8"],
        vec!["2", "3", "4"],
        vec!["2", "7", "8"],
    ]);

    (forest1, forest2, expected)
}