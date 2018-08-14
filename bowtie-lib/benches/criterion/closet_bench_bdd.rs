use criterion::Criterion;
use bowtie_lib::core::Family;
use bowtie_lib::core::Item;
use bowtie_lib::bdd::closet::Closet;
use bowtie_lib::bdd::closet_builder::ClosetBuilder;

fn families_2_items_4_no_selections() -> Closet {
    let shirt1 = Item::new("shirts:1");
    let shirt2 = Item::new("shirts:2");

    let pants1 = Item::new("pants:1");
    let pants2 = Item::new("pants:2");

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
    let shirt1 = Item::new("shirts:1");
    let shirt2 = Item::new("shirts:2");
    let shirt3 = Item::new("shirts:3");
    let shirt4 = Item::new("shirts:4");
    let shirt5 = Item::new("shirts:5");
    let shirt6 = Item::new("shirts:6");
    let shirt7 = Item::new("shirts:7");
    let shirt8 = Item::new("shirts:8");

    let pants1 = Item::new("pants:1");
    let pants2 = Item::new("pants:2");
    let pants3 = Item::new("pants:3");
    let pants4 = Item::new("pants:4");
    let pants5 = Item::new("pants:5");
    let pants6 = Item::new("pants:6");
    let pants7 = Item::new("pants:7");
    let pants8 = Item::new("pants:8");

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
    let shirt1 = Item::new("shirts:1");
    let shirt2 = Item::new("shirts:2");
    let shirt3 = Item::new("shirts:3");
    let shirt4 = Item::new("shirts:4");

    let pants1 = Item::new("pants:1");
    let pants2 = Item::new("pants:2");
    let pants3 = Item::new("pants:3");
    let pants4 = Item::new("pants:4");

    let shoes1 = Item::new("shoes:1");
    let shoes2 = Item::new("shoes:2");
    let shoes3 = Item::new("shoes:3");
    let shoes4 = Item::new("shoes:4");

    let socks1 = Item::new("socks:1");
    let socks2 = Item::new("socks:2");
    let socks3 = Item::new("socks:3");
    let socks4 = Item::new("socks:4");

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

pub fn bdd_closet_bench(c: &mut Criterion) {
    c.bench_function("bdd complete_outfit(2 families, 4 items)",
                     move |b| b.iter_with_setup(
                         || families_2_items_4_no_selections(),
                         |closet| closet.complete_outfit(vec![]),
                     ),
    );
    c.bench_function("bdd complete_outfit(2 families, 16 items)",
                     move |b| b.iter_with_setup(
                         || families_2_items_16_no_selections(),
                         |closet| closet.complete_outfit(vec![]),
                     ),
    );
    c.bench_function("bdd complete_outfit(4 families, 16 items)",
                     move |b| b.iter_with_setup(
                         || families_4_items_16_no_selections(),
                         |closet| closet.complete_outfit(vec![]),
                     ),
    );
}
