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
    criterion_benchmark
);
criterion_main!(benches);


fn combinations_recursive(tree: &Tree) -> BTreeSet<BTreeSet<Item>> {
    tree.combinations()
}

fn setup_tree() -> Tree {
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

fn criterion_benchmark(c: &mut Criterion) {
    let combo_recursive = Fun::new("Recursive", |b, i| b.iter(|| combinations_recursive(i)));
    let functions = vec!(combo_recursive);

    let tree = setup_tree();

    c.bench_functions("Combinations", functions, tree);
}
