use criterion::Criterion;
use criterion::Fun;

use tree_building;
use weave::core::Item;
use weave::Tree;

fn combinations_recursive(tree: &Tree<Item>) {
    tree.combinations_recursive();
}

fn combinations_iterative(tree: &Tree<Item>) {
    tree.combinations();
}

pub fn bench_combinations(c: &mut Criterion) {
    {
        let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| combinations_recursive(tree)));
        let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| combinations_iterative(tree)));
        let functions = vec!(combo_recursive, combo_iterative);

        let tree = tree_building::setup_tree_10();
        c.bench_functions("Combinations_10", functions, tree);
    }
    {
        let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| combinations_recursive(tree)));
        let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| combinations_iterative(tree)));
        let functions = vec!(combo_recursive, combo_iterative);

        let tree = tree_building::setup_tree_20();
        c.bench_functions("Combinations_20", functions, tree);
    }
    {
        let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| combinations_recursive(tree)));
        let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| combinations_iterative(tree)));
        let functions = vec!(combo_recursive, combo_iterative);

        let tree = tree_building::setup_tree_computer_parts();
        c.bench_functions("Combinations_Computer_Parts", functions, tree);
    }
}