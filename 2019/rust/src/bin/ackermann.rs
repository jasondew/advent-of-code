use aoc2019::intcode;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let m: u64 = args[1].parse().unwrap();
    let n: u64 = args[2].parse().unwrap();
    let machine = &mut intcode::Machine::new_with_tape(&vec![
        109, 99, 21101, 0, 13, 0, 203, 1, 203, 2, 1105, 1, 16, 204, 1, 99, 1205, 1, 26, 22101, 1,
        2, 1, 2105, 1, 0, 1205, 2, 40, 22101, -1, 1, 1, 21101, 0, 1, 2, 1105, 1, 16, 21101, 0, 57,
        3, 22101, 0, 1, 4, 22101, -1, 2, 5, 109, 3, 1105, 1, 16, 109, -3, 22101, 0, 4, 2, 22101,
        -1, 1, 1, 1105, 1, 16,
    ]);

    machine.input.push(n as intcode::Word);
    machine.input.push(m as intcode::Word);
    machine.run();

    println!("{:?}", machine.output.pop());

    Ok(())
}
