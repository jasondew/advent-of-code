use aoc2019::intcode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sum_of_primes(n: u64) -> u64 {
    let machine = &mut intcode::Machine::new_with_tape(&vec![
        3, 100, 1007, 100, 2, 7, 1105, -1, 87, 1007, 100, 1, 14, 1105, -1, 27, 101, -2, 100, 100,
        101, 1, 101, 101, 1105, 1, 9, 101, 105, 101, 105, 101, 2, 104, 104, 101, 1, 102, 102, 1,
        102, 102, 103, 101, 1, 103, 103, 7, 102, 101, 52, 1106, -1, 87, 101, 105, 102, 59, 1005,
        -1, 65, 1, 103, 104, 104, 101, 105, 102, 83, 1, 103, 83, 83, 7, 83, 105, 78, 1106, -1, 35,
        1101, 0, 1, -1, 1105, 1, 69, 4, 104, 99,
    ]);

    machine.input.push(n as intcode::Word);
    machine.run();

    machine.output.pop().unwrap() as u64
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum-of-primes 10000", |b| {
        b.iter(|| sum_of_primes(black_box(10000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
