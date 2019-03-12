use std::fmt::Debug;

use weave::Forest;

pub fn empty<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<(&'a str, usize)>) {
    let forest = F::empty();

    let expected = vec![];

    (forest, expected)
}

pub fn unit_with_one<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<(&'a str, usize)>) {
    let forest = F::unit(&["1", "1"]);

    let expected = vec![
        ("1", 1),
    ];

    (forest, expected)
}

pub fn unit_with_two<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<(&'a str, usize)>) {
    let forest = F::unit(&["1", "2"]);

    let expected = vec![
        ("1", 1),
        ("2", 1)
    ];

    (forest, expected)
}

pub fn overlapping_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<(&'a str, usize)>) {
    let forest = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = vec![
        ("1", 1),
        ("2", 2),
        ("3", 1),
    ];

    (forest, expected)
}

pub fn disjoint_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, Vec<(&'a str, usize)>) {
    let forest = F::many(&[
        vec!["1", "2"],
        vec!["3", "4"],
        vec!["1", "2"]
    ]);

    let expected = vec![
        ("1", 1),
        ("2", 1),
        ("3", 1),
        ("4", 1),
    ];

    (forest, expected)
}
