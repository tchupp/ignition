#[macro_use]
extern crate criterion;
extern crate bowtie_lib;

mod bdd_closet_bench;
mod iterative_closet_bench;

use criterion::Criterion;
use bdd_closet_bench::bdd_closet_bench;
use iterative_closet_bench::iterative_closet_bench;

criterion_group!(
    benches,
    bdd_closet_bench,
    iterative_closet_bench
);
criterion_main!(benches);