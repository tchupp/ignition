use criterion::{Benchmark, Criterion, Throughput};

use weave::Forest;

pub fn bench_trees<'a, 'b, 'c, F: Forest<&'a str> + 'static>(
    c: &mut Criterion,
    group_name: &'b str,
    forest: F,
) {
    let throughput = Throughput::Elements(forest.len() as u32);
    let benchmark = Benchmark::new(
        "trees",
        move |b| b.iter(|| forest.trees()),
    )
        .throughput(throughput);

    c.bench(group_name, benchmark);
}
