use aoc2020::day6::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 6");
    let input = std::fs::read_to_string("../inputs/06").unwrap();

    group.warm_up_time(std::time::Duration::from_secs(1));
    group.bench_function("part 1", |b| b.iter(|| part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| part2(black_box(&input))));

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
