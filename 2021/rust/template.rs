#[must_use]
pub fn part1(input: &str) -> usize {
    input.len()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("foobar"), 6)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("quux"), 4)
    }
}
