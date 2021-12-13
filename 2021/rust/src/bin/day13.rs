use aoc2021::day13::*;

fn main() -> std::io::Result<()> {
    let input: String = std::fs::read_to_string("../inputs/13")?;

    println!("part 1: {}", part1(&input));
    println!("part 2:\n{}", part2(&input));

    Ok(())
}
