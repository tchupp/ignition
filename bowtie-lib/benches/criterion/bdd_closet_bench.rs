use criterion::Criterion;
use bowtie_lib::bdd::closet::Closet;
use bowtie_lib::bdd::closet_builder::ClosetBuilder;
use bowtie_lib::core::Family;
use bowtie_lib::core::Item;

fn build_closet() -> Closet {
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
    c.bench_function("bdd reduce(recursive)",
                     move |b| b.iter_with_setup(
                         || build_closet(),
                         |closet| closet.reduce(),
                     ),
    );
}
