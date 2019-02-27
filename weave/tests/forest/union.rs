use std::fmt::Debug;

use weave::Forest;

pub fn both_forests_are_empty<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::empty();
    let forest2 = F::empty();

    let expected = F::empty();

    (forest1, forest2, expected)
}

pub fn left_is_empty_right_is_unit<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::empty();
    let forest2 = F::unit(&["1", "2"]);

    let expected = F::unit(&["1", "2"]);

    (forest1, forest2, expected)
}

pub fn left_is_empty_right_is_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::empty();
    let forest2 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    (forest1, forest2, expected)
}

pub fn forests_are_equal_unit<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["1", "2"]);
    let forest2 = F::unit(&["1", "2"]);

    let expected = F::unit(&["1", "2"]);

    (forest1, forest2, expected)
}

pub fn forests_are_disjoint_units<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["1", "2"]);
    let forest2 = F::unit(&["2", "3"]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    (forest1, forest2, expected)
}

pub fn left_is_unit_right_is_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["3", "4"]);
    let forest2 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"],
        vec!["3", "4"],
    ]);

    (forest1, forest2, expected)
}

pub fn forests_are_equal_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);
    let forest2 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    (forest1, forest2, expected)
}

pub fn forests_are_disjoint_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);
    let forest2 = F::many(&[
        vec!["1", "3"],
        vec!["2", "4"]
    ]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["1", "3"],
        vec!["2", "3"],
        vec!["2", "4"]
    ]);

    (forest1, forest2, expected)
}

pub fn forests_are_have_commonality<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"],
        vec!["3", "4"],
    ]);
    let forest2 = F::many(&[
        vec!["2", "3"],
        vec!["3", "4"],
        vec!["4", "5"],
    ]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"],
        vec!["3", "4"],
        vec!["4", "5"],
    ]);

    (forest1, forest2, expected)
}

pub fn left_is_unit_right_is_many_overlapping<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let forest1 = F::unit(&["1", "2"]);
    let forest2 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    (forest1, forest2, expected)
}