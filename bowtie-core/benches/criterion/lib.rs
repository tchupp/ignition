extern crate bowtie_core;
#[macro_use]
extern crate criterion;

use criterion::Criterion;

mod closet_bench_bdd;

criterion_group!(
    benches,
    closet_bench_bdd::bdd_closet_bench
);
criterion_main!(benches);