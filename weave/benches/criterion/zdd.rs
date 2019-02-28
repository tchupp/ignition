use criterion::Criterion;

use ::weave::zdd2::Forest;

use super::forest::*;
use super::forest::bench_trees;

pub fn benches(c: &mut Criterion) {
    bench_trees(c, "zdd/forest_10", forest_10::<Forest<&str>>());
    bench_trees(c, "zdd/forest_20", forest_20::<Forest<&str>>());
    bench_trees(c, "zdd/computer_parts", computer_parts::<Forest<&str>>());
}
