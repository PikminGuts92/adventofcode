use aoc2021::day22::{*, data::*};
use criterion::{BenchmarkId, black_box, Criterion, criterion_group, criterion_main};

fn day22_benchmark(c: &mut Criterion) {
    let data_set = [
        TEST_DATA_0,
    ];

    let mut group = c.benchmark_group("day_22");

    for (i, data) in data_set.iter().enumerate() {
        group.bench_function(format!("data_{}", i), |b| b.iter(|| find_on_cube_count(data)));
    }
    group.finish();

    let mut group = c.benchmark_group("day_22_2");

    for (i, data) in data_set.iter().enumerate() {
        group.bench_function(format!("data_{}", i), |b| b.iter(|| find_on_cube_count2(data)));
    }
    group.finish();
}


criterion_group!(benches, day22_benchmark);
criterion_main!(benches);