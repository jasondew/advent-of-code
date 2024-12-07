#[must_use]
pub fn part1(input: &str) -> usize {
    let equations = parse(input);

    equations
        .iter()
        .filter(|(total, values)| {
            let (init, rest) = values.split_at(1);

            is_possible(*total, init[0], &rest)
        })
        .map(|(total, _)| total)
        .sum()
}

fn is_possible(desired: usize, current: usize, values: &[usize]) -> bool {
    if values.is_empty() {
        return desired == current;
    }
    if current > desired {
        return false;
    }
    let (next, rest) = values.split_at(1);
    let next_value = next[0];

    is_possible(desired, current + next_value, rest)
        || is_possible(desired, current * next_value, rest)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let equations = parse(input);

    equations
        .iter()
        .filter(|(total, values)| {
            let (init, rest) = values.split_at(1);

            is_possible_with_concatenation(*total, init[0], &rest)
        })
        .map(|(total, _)| total)
        .sum()
}

fn is_possible_with_concatenation(
    desired: usize,
    current: usize,
    values: &[usize],
) -> bool {
    if values.is_empty() {
        return desired == current;
    }
    if current > desired {
        return false;
    }
    let (next, rest) = values.split_at(1);
    let next_value = next[0];

    is_possible_with_concatenation(desired, current + next_value, rest)
        || is_possible_with_concatenation(desired, current * next_value, rest)
        || is_possible_with_concatenation(
            desired,
            concatenate(current, next_value),
            rest,
        )
}

fn concatenate(a: usize, b: usize) -> usize {
    a * 10usize.pow(digits(b)) + b
}

fn digits(value: usize) -> u32 {
    if value < 10 {
        return 1;
    }
    1 + digits(value / 10)
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let total = parse_usize(left);
            let values =
                right.trim().split(" ").map(|v| parse_usize(v)).collect();

            (total, values)
        })
        .collect()
}

fn parse_usize(value: &str) -> usize {
    value.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 3749)
    }

    #[test]
    fn part2_example() {
        assert!(is_possible_with_concatenation(156, 15, &[6]));
        assert!(is_possible_with_concatenation(7290, 6, &[8, 6, 15]));
        assert_eq!(part2(input()), 11387);
    }

    #[test]
    fn concatenate_test() {
        assert_eq!(digits(1), 1);
        assert_eq!(digits(42), 2);
        assert_eq!(digits(582), 3);
        assert_eq!(concatenate(11, 22), 1122);
    }
}
