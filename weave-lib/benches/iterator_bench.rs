#[macro_use]
extern crate criterion;
extern crate weave_lib;

use criterion::Bencher;
use criterion::Criterion;
use weave_lib::core::Family;
use weave_lib::core::Item;
use weave_lib::iterative::closet::Closet;
use weave_lib::iterative::closet_builder::ClosetBuilder;
use weave_lib::iterative::outfits::complete_outfit;

fn families_2_items_4_no_selections() -> Closet {
    let shirt1 = Item::new("shirt1");
    let shirt2 = Item::new("shirt2");

    let pants1 = Item::new("pants1");
    let pants2 = Item::new("pants2");

    let shirts = Family::new("shirts");
    let pants = Family::new("pants");

    let closet_builder = ClosetBuilder::new()
        .add_item(&shirts, &shirt1)
        .add_item(&shirts, &shirt2)
        .add_item(&pants, &pants1)
        .add_item(&pants, &pants2);

    closet_builder.must_build()
}

fn families_2_items_16_no_selections() -> Closet {
    let shirt1 = Item::new("shirt1");
    let shirt2 = Item::new("shirt2");
    let shirt3 = Item::new("shirt3");
    let shirt4 = Item::new("shirt4");
    let shirt5 = Item::new("shirt5");
    let shirt6 = Item::new("shirt6");
    let shirt7 = Item::new("shirt7");
    let shirt8 = Item::new("shirt8");

    let pants1 = Item::new("pants1");
    let pants2 = Item::new("pants2");
    let pants3 = Item::new("pants3");
    let pants4 = Item::new("pants4");
    let pants5 = Item::new("pants5");
    let pants6 = Item::new("pants6");
    let pants7 = Item::new("pants7");
    let pants8 = Item::new("pants8");

    let shirts = Family::new("shirts");
    let pants = Family::new("pants");

    let closet_builder = ClosetBuilder::new()
        .add_item(&shirts, &shirt1)
        .add_item(&shirts, &shirt2)
        .add_item(&shirts, &shirt3)
        .add_item(&shirts, &shirt4)
        .add_item(&shirts, &shirt5)
        .add_item(&shirts, &shirt6)
        .add_item(&shirts, &shirt7)
        .add_item(&shirts, &shirt8)
        .add_item(&pants, &pants1)
        .add_item(&pants, &pants2)
        .add_item(&pants, &pants3)
        .add_item(&pants, &pants4)
        .add_item(&pants, &pants5)
        .add_item(&pants, &pants6)
        .add_item(&pants, &pants7)
        .add_item(&pants, &pants8);

    closet_builder.must_build()
}

fn families_4_items_16_no_selections() -> Closet {
    let shirt1 = Item::new("shirt1");
    let shirt2 = Item::new("shirt2");
    let shirt3 = Item::new("shirt3");
    let shirt4 = Item::new("shirt4");

    let pants1 = Item::new("pants1");
    let pants2 = Item::new("pants2");
    let pants3 = Item::new("pants3");
    let pants4 = Item::new("pants4");

    let shoes1 = Item::new("shoes1");
    let shoes2 = Item::new("shoes2");
    let shoes3 = Item::new("shoes3");
    let shoes4 = Item::new("shoes4");

    let socks1 = Item::new("socks1");
    let socks2 = Item::new("socks2");
    let socks3 = Item::new("socks3");
    let socks4 = Item::new("socks4");

    let shirts = Family::new("shirts");
    let pants = Family::new("pants");
    let socks = Family::new("socks");
    let shoes = Family::new("shoes");

    let closet_builder = ClosetBuilder::new()
        .add_item(&shirts, &shirt1)
        .add_item(&shirts, &shirt2)
        .add_item(&shirts, &shirt3)
        .add_item(&shirts, &shirt4)
        .add_item(&pants, &pants1)
        .add_item(&pants, &pants2)
        .add_item(&pants, &pants3)
        .add_item(&pants, &pants4)
        .add_item(&socks, &socks1)
        .add_item(&socks, &socks2)
        .add_item(&socks, &socks3)
        .add_item(&socks, &socks4)
        .add_item(&shoes, &shoes1)
        .add_item(&shoes, &shoes2)
        .add_item(&shoes, &shoes3)
        .add_item(&shoes, &shoes4);

    closet_builder.must_build()
}

fn iterative_bench(c: &mut Criterion) {
    c.bench_function("iterative complete_outfit(2 families, 4 items)", move |b| b.iter_with_setup(|| families_2_items_4_no_selections(), |closet| complete_outfit(closet.clone(), vec![])));
    c.bench_function("iterative complete_outfit(2 families, 16 items)", move |b| b.iter_with_setup(|| families_2_items_16_no_selections(), |closet| complete_outfit(closet.clone(), vec![])));
    c.bench_function("iterative complete_outfit(4 families, 16 items)", move |b| b.iter_with_setup(|| families_4_items_16_no_selections(), |closet| complete_outfit(closet.clone(), vec![])));
}

criterion_group!(benches, iterative_bench);
criterion_main!(benches);
