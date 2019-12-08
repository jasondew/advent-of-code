use std::fs;
use std::io;

mod intcode;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("../inputs/05")?;
    let mut intcode: Vec<i32> = input
        .trim()
        .split(",")
        .map(|str| str.parse().unwrap())
        .collect();

    let part1 = intcode::run(&mut intcode, &mut vec![1]).output.unwrap();
    println!("part 1: {:?}", part1);

    Ok(())
}
