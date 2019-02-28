use criterion::Criterion;
use criterion::Fun;

use weave::zdd::Tree;
use zdd_building;

fn zdd_combinations_recursive(tree: &Tree<&str>) {
    tree.combinations_recursive();
}

fn zdd_combinations_iterative(tree: &Tree<&str>) {
    tree.combinations();
}

pub fn bench_combinations(c: &mut Criterion) {
    {
        let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| zdd_combinations_recursive(tree)));
        let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| zdd_combinations_iterative(tree)));
        let functions = vec!(combo_recursive, combo_iterative);

        let tree = zdd_building::setup_tree_10();
        c.bench_functions("Zdd Combinations - 10", functions, tree);
    }
    {
        let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| zdd_combinations_recursive(tree)));
        let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| zdd_combinations_iterative(tree)));
        let functions = vec!(combo_recursive, combo_iterative);

        let tree = zdd_building::setup_tree_20();
        c.bench_functions("Zdd Combinations - 20", functions, tree);
    }
    // Disabled because they take too long to run (lol...benchmarking)
    /*{
        let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| zdd_combinations_recursive(tree)));
        let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| zdd_combinations_iterative(tree)));
        let functions = vec!(combo_recursive, combo_iterative);

        let tree = zdd_building::setup_tree_computer_parts();
        c.bench_functions("Zdd Combinations - ComputerParts", functions, tree);
    }*/
}