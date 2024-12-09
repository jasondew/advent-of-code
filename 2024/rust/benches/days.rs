use aoc2024::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! day {
    ($name:ident, $day:tt) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(format!("Day {}", $day));
            let input =
                std::fs::read_to_string(format!("../inputs/{}", $day)).unwrap();

            group.bench_function("part 1", |b| {
                b.iter(|| $name::part1(black_box(&input)))
            });
            group.bench_function("part 2", |b| {
                b.iter(|| $name::part2(black_box(&input)))
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

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(25);
    targets = day01, day02, day03, day04, day05, day06, day07, day08, day09
}
criterion_main!(benches);
