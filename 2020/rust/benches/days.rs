use aoc2020::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day3(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 3");
    let input = std::fs::read_to_string("../inputs/03").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day3::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day3::part2(black_box(&input))));
    group.finish();
}

pub fn day4(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 4");
    let input = std::fs::read_to_string("../inputs/04").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day4::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day4::part2(black_box(&input))));
    group.finish();
}

pub fn day5(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 5");
    let input = std::fs::read_to_string("../inputs/05").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day5::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day5::part2(black_box(&input))));
    group.finish();
}

pub fn day6(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 6");
    let input = std::fs::read_to_string("../inputs/06").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day6::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day6::part2(black_box(&input))));
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

pub fn day12(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 12");
    let input = std::fs::read_to_string("../inputs/12").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day12::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day12::part2(black_box(&input))));
    group.finish();
}

pub fn day13(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 13");
    let input = std::fs::read_to_string("../inputs/13").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day13::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day13::part2(black_box(&input))));
    group.finish();
}

pub fn day14(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 14");
    let input = std::fs::read_to_string("../inputs/14").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day14::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day14::part2(black_box(&input))));
    group.finish();
}

pub fn day15(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 15");
    let input = std::fs::read_to_string("../inputs/15").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day15::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day15::part2(black_box(&input))));
    group.finish();
}

pub fn day16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 16");
    let input = std::fs::read_to_string("../inputs/16").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day16::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day16::part2(black_box(&input))));
    group.finish();
}

pub fn day17(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 17");
    let input = std::fs::read_to_string("../inputs/17").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day17::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day17::part2(black_box(&input))));
    group.finish();
}

pub fn day18(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 18");
    let input = std::fs::read_to_string("../inputs/18").unwrap();

    group.bench_function("part 1", |b| b.iter(|| day18::part1(black_box(&input))));
    group.bench_function("part 2", |b| b.iter(|| day18::part2(black_box(&input))));
    group.finish();
}

criterion_group!(
    benches, day3, day4, day5, day6, day10, day11, day12, day13, day14, day15, day16, day17, day18
);
criterion_main!(benches);
