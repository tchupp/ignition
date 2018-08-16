#[macro_use]
extern crate criterion;
extern crate bowtie_core;

mod closet_bench_bdd;
mod closet_bench_iterative;

use criterion::Criterion;
use closet_bench_bdd::bdd_closet_bench;
use closet_bench_iterative::iterative_closet_bench;

criterion_group!(
    benches,
    bdd_closet_bench,
    iterative_closet_bench
);
criterion_main!(benches);