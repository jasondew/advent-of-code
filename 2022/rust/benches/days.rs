use aoc2022::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 01");
    let input = std::fs::read_to_string("../inputs/01").unwrap();

    group.bench_function("part 1", |b| {
        b.iter(|| day01::part1(black_box(&input)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| day01::part2(black_box(&input)))
    });
    group.finish();
}

pub fn day02(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 02");
    let input = std::fs::read_to_string("../inputs/02").unwrap();

    group.bench_function("part 1", |b| {
        b.iter(|| day01::part1(black_box(&input)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| day01::part2(black_box(&input)))
    });
    group.finish();
}

criterion_group!(benches, day01, day02);
criterion_main!(benches);
