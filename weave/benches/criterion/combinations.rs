use criterion::Criterion;
use criterion::Fun;

use weave::zdd2::Forest;
use weave::zdd::Tree;
use zdd2_building;
use zdd_building;

fn zdd_combinations_recursive(tree: &Tree<&str>) {
    tree.combinations_recursive();
}

fn zdd_combinations_iterative(tree: &Tree<&str>) {
    tree.combinations();
}

fn zdd2_trees(forest: &Forest<&str>) {
    forest.trees();
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
    {
        let combo_iterative = Fun::new("Iterative", |b, forest| b.iter(|| zdd2_trees(forest)));
        let functions = vec!(combo_iterative);

        let forest = zdd2_building::setup_forest_10();
        c.bench_functions("Zdd2 Combinations - 10", functions, forest);
    }
    {
        let combo_iterative = Fun::new("Iterative", |b, forest| b.iter(|| zdd2_trees(forest)));
        let functions = vec!(combo_iterative);

        let forest = zdd2_building::setup_forest_20();
        c.bench_functions("Zdd2 Combinations - 20", functions, forest);
    }
    // Disabled because they take too long to run (lol...benchmarking)
    /*{
        let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| zdd_combinations_recursive(tree)));
        let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| zdd_combinations_iterative(tree)));
        let functions = vec!(combo_recursive, combo_iterative);

        let tree = zdd_building::setup_tree_computer_parts();
        c.bench_functions("Zdd Combinations - ComputerParts", functions, tree);
    }*/
    /*{
        let combo_iterative = Fun::new("Iterative", |b, forest| b.iter(|| zdd2_trees(forest)));
        let functions = vec!(combo_iterative);

        let forest = zdd2_building::setup_forest_computer_parts();
        c.bench_functions("Zdd2 Combinations - ComputerParts", functions, forest);
    }*/
}