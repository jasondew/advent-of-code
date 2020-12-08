use aoc2019::intcode;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let n: u64 = args[1].parse().unwrap();
    let machine = &mut intcode::Machine::new_with_tape(&vec![
        3, 1, 109, 583, 108, 0, 1, 9, 1106, -1, 14, 4, 1, 99, 107, 0, 1, 19, 1105, -1, 27, 104, -1,
        102, -1, 1, 1, 21101, 0, 38, 0, 20101, 0, 1, 1, 1105, 1, 138, 2101, 1, 1, 41, 101, 596, 41,
        45, 1101, 1, 596, 77, 1101, 0, 1, 53, 101, 1, 77, 77, 101, 1, 53, 53, 7, 45, 77, 67, 1105,
        -1, 128, 108, 1, 1, 74, 1105, -1, 128, 1005, -1, 54, 1, 53, 77, 93, 7, 45, 93, 88, 1105,
        -1, 101, 1101, 0, 1, -1, 1, 53, 93, 93, 1105, 1, 83, 21101, 0, 116, 0, 20101, 0, 1, 1,
        20101, 0, 53, 2, 1105, 1, 235, 1205, 2, 54, 4, 53, 2101, 0, 1, 1, 1105, 1, 101, 108, 1, 1,
        133, 1105, -1, 137, 4, 1, 99, 22101, 0, 1, 2, 22101, 0, 1, 1, 21101, 0, 163, 3, 22101, 0,
        1, 4, 22101, 0, 2, 5, 109, 3, 1105, 1, 198, 109, -3, 22102, -1, 1, 1, 22201, 1, 4, 3,
        22102, -1, 1, 1, 1208, 3, 0, 182, 2105, -1, 0, 1208, 3, 1, 189, 2105, -1, 0, 22101, 0, 4,
        1, 1105, 1, 146, 1207, 1, 1, 203, 2105, -1, 0, 21101, 0, 222, 3, 22101, 0, 2, 4, 22101, 0,
        1, 5, 109, 3, 1105, 1, 235, 109, -3, 22201, 1, 4, 1, 21101, 0, 2, 2, 1105, 1, 235, 1105, 0,
        280, 101, 383, 236, 243, 1107, -1, 583, 247, 1106, -1, 276, 101, 383, 236, 256, 102, 1,
        275, -1, 102, 2, 275, 275, 1007, 275, 0, 266, 1105, -1, 280, 101, 1, 236, 236, 1105, 1,
        238, 1, 101, -1, 236, 236, 101, 383, 236, 286, 207, 1, -1, 289, 1106, -1, -1, 22101, 0, 1,
        3, 2102, 1, 2, 363, 2102, -1, 2, 369, 22102, 0, 1, 1, 22102, 0, 2, 2, 101, 1, 236, 320,
        101, -1, 320, 320, 1107, -1, 0, 324, 2105, -1, 0, 22102, 2, 2, 2, 101, 383, 320, 336, 207,
        3, -1, 339, 1105, -1, 361, 22101, 1, 2, 2, 22102, -1, 3, 3, 101, 383, 320, 354, 22001, -1,
        3, 3, 22102, -1, 3, 3, 1207, 2, -1, 366, 1105, -1, 315, 22101, -1, 2, 2, 101, 383, 320,
        377, 22001, -1, 1, 1, 1105, 1, 315,
    ]);

    machine.input.push(n as intcode::Word);
    machine.run();

    println!("{:?}", machine.output);

    Ok(())
}