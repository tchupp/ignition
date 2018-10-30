extern crate bincode;
#[macro_use]
extern crate criterion;
extern crate serde_json;
extern crate weave;

use criterion::Criterion;

mod tree_building;

mod combinations;
mod serialize;

criterion_group!(
    benches,
    combinations::bench_combinations,
    serialize::bench_serialize,
    serialize::bench_deserialize
);
criterion_main!(benches);
