use aoc2021::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 01");
    let input = std::fs::read_to_string("../inputs/01").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day01::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day01::part2(black_box(&input))));
    group.finish();
}

pub fn day02(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 02");
    let input = std::fs::read_to_string("../inputs/02").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day02::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day02::part2(black_box(&input))));
    group.finish();
}

pub fn day03(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 03");
    let input = std::fs::read_to_string("../inputs/03").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day03::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day03::part2(black_box(&input))));
    group.finish();
}

pub fn day04(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 04");
    let input = std::fs::read_to_string("../inputs/04").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day04::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day04::part2(black_box(&input))));
    group.finish();
}

pub fn day05(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 05");
    let input = std::fs::read_to_string("../inputs/05").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day05::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day05::part2(black_box(&input))));
    group.finish();
}

pub fn day06(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 06");
    let input = std::fs::read_to_string("../inputs/06").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day06::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day06::part2(black_box(&input))));
    group.finish();
}

pub fn day07(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 07");
    let input = std::fs::read_to_string("../inputs/07").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day07::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day07::part2(black_box(&input))));
    group.finish();
}

pub fn day08(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 08");
    let input = std::fs::read_to_string("../inputs/08").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day08::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day08::part2(black_box(&input))));
    group.finish();
}

pub fn day09(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 09");
    let input = std::fs::read_to_string("../inputs/09").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day09::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day09::part2(black_box(&input))));
    group.finish();
}

pub fn day10(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 10");
    let input = std::fs::read_to_string("../inputs/10").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day10::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day10::part2(black_box(&input))));
    group.finish();
}

pub fn day11(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 11");
    let input = std::fs::read_to_string("../inputs/11").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day11::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day11::part2(black_box(&input))));
    group.finish();
}

criterion_group!(
    benches, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11
);
criterion_main!(benches);
