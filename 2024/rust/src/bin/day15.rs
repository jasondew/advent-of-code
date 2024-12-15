use aoc2024::day15::*;

fn main() -> std::io::Result<()> {
    let input: String = std::fs::read_to_string("../inputs/15")?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}
