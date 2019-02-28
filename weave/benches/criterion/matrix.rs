use criterion::Criterion;

use ::weave::matrix::Forest;

use super::forest::*;

pub fn benches(c: &mut Criterion) {
    bench_trees(c, "matrix/forest_10", forest_10::<Forest<&str>>());
    bench_trees(c, "matrix/forest_20", forest_20::<Forest<&str>>());
    bench_trees(c, "matrix/computer_parts", computer_parts::<Forest<&str>>());
}
