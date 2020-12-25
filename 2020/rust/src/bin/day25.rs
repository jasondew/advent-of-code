use aoc2020::day25::*;

fn main() -> std::io::Result<()> {
    let input: String = std::fs::read_to_string("../inputs/25")?;

    println!("part 1: {}", part1(&input));

    Ok(())
}
