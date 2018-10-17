#[macro_use]
extern crate criterion;
extern crate weave;

use criterion::Criterion;
use criterion::Fun;
use std::collections::BTreeSet;
use weave::core::Item;
use weave::zdd::Tree;
use weave::zdd::Universe;

criterion_group!(
    benches,
    bench_combo_10,
    bench_combo_20
);
criterion_main!(benches);


fn combinations_recursive(tree: &Tree<Item>) -> BTreeSet<BTreeSet<Item>> {
    tree.combinations()
}

fn combinations_iterative(tree: &Tree<Item>) -> BTreeSet<BTreeSet<Item>> {
    tree.combinations_iter()
}

fn setup_tree_20() -> Tree<Item> {
    let item0 = Item::new("0");
    let item1 = Item::new("1");
    let item2 = Item::new("2");
    let item3 = Item::new("3");
    let item4 = Item::new("4");
    let item5 = Item::new("5");
    let item6 = Item::new("6");
    let item7 = Item::new("7");
    let item8 = Item::new("8");
    let item9 = Item::new("9");
    let item10 = Item::new("10");
    let item11 = Item::new("11");
    let item12 = Item::new("12");
    let item13 = Item::new("13");
    let item14 = Item::new("14");
    let item15 = Item::new("15");
    let item16 = Item::new("16");
    let item17 = Item::new("17");
    let item18 = Item::new("18");
    let item19 = Item::new("19");

    let universe = Universe::from(vec![
        item0.clone(), item1.clone(), item2.clone(), item3.clone(), item4.clone(), item5.clone(), item6.clone(), item7.clone(), item8.clone(), item9.clone(),
        item10.clone(), item11.clone(), item12.clone(), item13.clone(), item14.clone(), item15.clone(), item16.clone(), item17.clone(), item18.clone(), item19.clone()
    ]);
    universe.hyper_tree(&[
        vec![item0.clone(), item1.clone(), item2.clone(), item3.clone()],
        vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()],
        vec![item2.clone(), item3.clone(), item4.clone(), item5.clone()],
        vec![item6.clone(), item7.clone(), item8.clone(), item9.clone()],
        vec![item1.clone(), item3.clone(), item5.clone(), item7.clone(), item9.clone()],
        vec![item0.clone(), item0.clone(), item1.clone(), item2.clone(), item3.clone(), item5.clone(), item8.clone()],
        vec![item10.clone(), item11.clone(), item12.clone(), item13.clone()],
        vec![item11.clone(), item12.clone(), item13.clone(), item14.clone()],
        vec![item12.clone(), item13.clone(), item14.clone(), item15.clone()],
        vec![item16.clone(), item17.clone(), item18.clone(), item19.clone()],
        vec![item11.clone(), item13.clone(), item15.clone(), item17.clone(), item19.clone()],
        vec![item10.clone(), item10.clone(), item11.clone(), item12.clone(), item13.clone(), item15.clone(), item18.clone()],
        vec![item0.clone(), item1.clone(), item2.clone(), item13.clone()],
        vec![item1.clone(), item2.clone(), item3.clone(), item14.clone()],
        vec![item12.clone(), item13.clone(), item4.clone(), item5.clone()],
    ])
}

fn setup_tree_10() -> Tree<Item> {
    let item0 = Item::new("0");
    let item1 = Item::new("1");
    let item2 = Item::new("2");
    let item3 = Item::new("3");
    let item4 = Item::new("4");
    let item5 = Item::new("5");
    let item6 = Item::new("6");
    let item7 = Item::new("7");
    let item8 = Item::new("8");
    let item9 = Item::new("9");

    let universe = Universe::from(vec![item0.clone(), item1.clone(), item2.clone(), item3.clone(), item4.clone(), item5.clone(), item6.clone(), item7.clone(), item8.clone(), item9.clone()]);
    universe.hyper_tree(&[
        vec![item0.clone(), item1.clone(), item2.clone(), item3.clone()],
        vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()],
        vec![item2.clone(), item3.clone(), item4.clone(), item5.clone()],
        vec![item6.clone(), item7.clone(), item8.clone(), item9.clone()],
        vec![item1.clone(), item3.clone(), item5.clone(), item7.clone(), item9.clone()],
        vec![item0.clone(), item0.clone(), item1.clone(), item2.clone(), item3.clone(), item5.clone(), item8.clone()],
    ])
}

fn bench_combo_10(c: &mut Criterion) {
    let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| combinations_recursive(tree)));
    let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| combinations_iterative(tree)));
    let functions = vec!(combo_recursive, combo_iterative);

    let tree = setup_tree_10();
    c.bench_functions("Combinations_10", functions, tree);
}

fn bench_combo_20(c: &mut Criterion) {
    let combo_recursive = Fun::new("Recursive", |b, tree| b.iter(|| combinations_recursive(tree)));
    let combo_iterative = Fun::new("Iterative", |b, tree| b.iter(|| combinations_iterative(tree)));
    let functions = vec!(combo_recursive, combo_iterative);

    let tree = setup_tree_20();
    c.bench_functions("Combinations_20", functions, tree);
}
