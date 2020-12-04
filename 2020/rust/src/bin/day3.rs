use aoc2020::day3::*;

fn main() -> std::io::Result<()> {
    let map: Map = std::fs::read_to_string("../inputs/03")?.parse().unwrap();

    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));

    Ok(())
}
