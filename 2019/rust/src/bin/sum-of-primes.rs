use aoc2019::intcode;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let n: u64 = args[1].parse().unwrap();
    let machine = &mut intcode::Machine::new_with_tape(&vec![
        3, 100, 1007, 100, 2, 7, 1105, -1, 87, 1007, 100, 1, 14, 1105, -1, 27, 101, -2, 100, 100,
        101, 1, 101, 101, 1105, 1, 9, 101, 105, 101, 105, 101, 2, 104, 104, 101, 1, 102, 102, 1,
        102, 102, 103, 101, 1, 103, 103, 7, 102, 101, 52, 1106, -1, 87, 101, 105, 102, 59, 1005,
        -1, 65, 1, 103, 104, 104, 101, 105, 102, 83, 1, 103, 83, 83, 7, 83, 105, 78, 1106, -1, 35,
        1101, 0, 1, -1, 1105, 1, 69, 4, 104, 99,
    ]);

    machine.input.push(n as intcode::Word);
    machine.run();

    println!("{:?}", machine.output.pop());

    Ok(())
}
