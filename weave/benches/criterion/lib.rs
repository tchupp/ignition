extern crate bincode;
#[macro_use]
extern crate criterion;
extern crate serde_json;
extern crate weave;

mod zdd_building;

mod combinations;
mod serialize;

mod forest;

mod matrix;
mod zdd;

criterion_group!(
    benches,
    combinations::bench_combinations,
    serialize::bench_serialize,
    serialize::bench_deserialize,
    matrix::benches,
    zdd::benches,
);
criterion_main!(benches);
