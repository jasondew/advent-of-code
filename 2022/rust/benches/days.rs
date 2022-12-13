use aoc2022::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! day {
    ($name:ident, $day:tt) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(format!("Day {}", $day));
            let input =
                std::fs::read_to_string(format!("../inputs/{}", $day)).unwrap();

            group.bench_function("part 1", |b| {
                b.iter(|| day01::part1(black_box(&input)))
            });
            group.bench_function("part 2", |b| {
                b.iter(|| day01::part2(black_box(&input)))
            });
            group.finish();
        }
    };
}

day!(day01, "01");
day!(day02, "02");
day!(day03, "03");
day!(day04, "04");
day!(day05, "05");
day!(day06, "06");
day!(day07, "07");
day!(day08, "08");
day!(day09, "09");
day!(day10, "10");
day!(day11, "11");
day!(day12, "12");

criterion_group!(
    benches, day01, day02, day03, day04, day05, day06, day07, day08, day09,
    day10, day11, day12
);
criterion_main!(benches);
