fn part1(input: &String) -> i32 {
    input.chars().count() as i32
}

fn part2(input: &String) -> i32 {
    input.lines().count() as i32
}

fn main() -> std::io::Result<()> {
    let input: String = std::fs::read_to_string("../inputs/01")?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
