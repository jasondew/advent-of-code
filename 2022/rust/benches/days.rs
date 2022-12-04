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

criterion_group!(benches, day01, day02, day03);
criterion_main!(benches);
