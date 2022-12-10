#[must_use]
pub fn part1(input: &str) -> usize {
    *group_sum(input).iter().max().unwrap()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut elf_calories = group_sum(input);
    elf_calories.sort_unstable();
    elf_calories.reverse();

    elf_calories.iter().take(3).sum()
}

fn group_sum(input: &str) -> Vec<usize> {
    let mut totals: Vec<usize> = Vec::new();
    let mut current_total: usize = 0;

    for line in input.lines() {
        if let Ok(value) = line.trim().parse::<usize>() {
            current_total += value;
        } else {
            totals.push(current_total);
            current_total = 0;
        }
    }

    totals
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
        1
        2
        3

        2
        3

        1
        3
        1

        5

        1
        6
        \n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 7)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 18)
    }
}
