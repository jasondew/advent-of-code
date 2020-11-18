fn part1(input: &String) -> i32 {
    input.chars().fold(0, |total, char| match char {
        '(' => total + 1,
        ')' => total - 1,
        _ => total,
    })
}

fn part2(input: &String) -> i32 {
    let mut total: i32 = 0;
    let mut index: i32 = 1;

    for char in input.chars() {
        total = match char {
            '(' => total + 1,
            ')' => total - 1,
            _ => total,
        };

        if total == -1 {
            break;
        } else {
            index += 1;
        }
    }

    index
}

fn main() -> std::io::Result<()> {
    let input: String = std::fs::read_to_string("../inputs/01")?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
