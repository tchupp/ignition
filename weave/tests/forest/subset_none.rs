use std::fmt::Debug;

use weave::Forest;

pub fn empty_forest_with_single_element<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<&'a str>, F) {
    let forest = F::empty();
    let elements = vec!["1"];

    let expected = F::empty();

    (forest, elements, expected)
}

pub fn unit_forest_with_empty_elements<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<&'a str>, F) {
    let forest = F::unit(&["2", "3"]);
    let elements = vec![];

    let expected = F::unit(&["2", "3"]);

    (forest, elements, expected)
}

pub fn unit_forest_with_one_element<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<&'a str>, F) {
    let forest = F::unit(&["2", "3"]);
    let elements = vec!["2"];

    let expected = F::empty();

    (forest, elements, expected)
}

pub fn unit_forest_with_disjoint_elements<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<&'a str>, F) {
    let forest = F::unit(&["2", "3"]);
    let elements = vec!["1"];

    let expected = F::unit(&["2", "3"]);

    (forest, elements, expected)
}

pub fn many_forest_with_one_element<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<&'a str>, F) {
    let forest = F::many(&[
        vec!["1", "3"],
        vec!["2", "3"],
        vec!["1", "2"]
    ]);
    let elements = vec!["3"];

    let expected = F::unit(&["1", "2"]);

    (forest, elements, expected)
}

pub fn many_forest_with_many_elements<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<&'a str>, F) {
    let forest = F::many(&[
        vec!["1", "3"],
        vec!["2", "3"],
        vec!["1", "2"]
    ]);
    let elements = vec!["1", "3"];

    let expected = F::empty();

    (forest, elements, expected)
}