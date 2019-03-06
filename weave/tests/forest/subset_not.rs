use std::fmt::Debug;

use weave::Forest;

pub fn empty_forest<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, &'a str, F) {
    let forest = F::empty();
    let element = "1";

    let expected = F::empty();

    (forest, element, expected)
}

pub fn unit_forest_with_disjoint_element<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, &'a str, F) {
    let forest = F::unit(&["2", "3"]);
    let element = "1";

    let expected = F::unit(&["2", "3"]);

    (forest, element, expected)
}

pub fn many_forest_with_disjoint_element<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, &'a str, F) {
    let forest = F::many(&[
        vec!["1", "3"],
        vec!["2", "3"]
    ]);
    let element = "4";

    let expected = F::many(&[
        vec!["1", "3"],
        vec!["2", "3"]
    ]);

    (forest, element, expected)
}

pub fn unit_forest_with_matching_element<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, &'a str, F) {
    let forest = F::unit(&["1", "3"]);
    let element = "1";

    let expected = F::empty();

    (forest, element, expected)
}

pub fn many_forest_with_matching_element_1<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, &'a str, F) {
    let forest = F::many(&[
        vec!["1", "3"],
        vec!["2", "3"]
    ]);
    let element = "1";

    let expected = F::unit(&["2", "3"]);

    (forest, element, expected)
}

pub fn many_forest_with_matching_element_2<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, &'a str, F) {
    let forest = F::many(&[
        vec!["1", "3"],
        vec!["2", "3"]
    ]);
    let element = "2";

    let expected = F::unit(&["1", "3"]);

    (forest, element, expected)
}