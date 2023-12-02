#[must_use]
pub fn part1(input: &str) -> usize {
    input.lines().count()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
        foobar
        \n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 2)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 2)
    }
}
