use aoc2020::day4::*;

fn main() -> std::io::Result<()> {
    let passports: Vec<Passport> = Passport::load(std::fs::read_to_string("../inputs/04")?);

    println!("part 1: {}", part1(&passports));
    println!("part 2: {}", part2(&passports));

    Ok(())
}
