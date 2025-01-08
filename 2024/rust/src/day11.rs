use std::collections::HashMap;

#[must_use]
pub fn part1(input: &str) -> usize {
    let stones = parse(input);
    stone_count_after_blinks(stones, 25)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let stones = parse(input);
    stone_count_after_blinks(stones, 75)
}

fn stone_count_after_blinks(stones: Vec<usize>, blinks: usize) -> usize {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    stones
        .into_iter()
        .map(|stone| blink(stone, blinks, &mut cache))
        .sum()
}

fn blink(
    stone: usize,
    blinks_left: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(count) = cache.get(&(stone, blinks_left)) {
        *count
    } else if blinks_left == 0 {
        1
    } else {
        let count = if stone == 0 {
            blink(1, blinks_left - 1, cache)
        } else if let Some((left, right)) = split(stone) {
            blink(left, blinks_left - 1, cache)
                + blink(right, blinks_left - 1, cache)
        } else {
            blink(stone * 2024, blinks_left - 1, cache)
        };
        cache.insert((stone, blinks_left), count);
        count
    }
}

fn digits(value: usize) -> Vec<usize> {
    if value < 10 {
        vec![value]
    } else {
        let mut digits = digits(value / 10);
        digits.push(value % 10);
        digits
    }
}

fn combine(digits: &[usize]) -> usize {
    let mut combined: usize = 0;

    for digit in digits {
        combined = combined * 10 + digit;
    }

    combined
}

fn split(stone: usize) -> Option<(usize, usize)> {
    let digits: Vec<usize> = digits(stone);
    let length: usize = digits.len();

    if length % 2 == 0 {
        let midpoint = length / 2;
        Some((
            combine(&digits[0..midpoint]),
            combine(&digits[midpoint..length]),
        ))
    } else {
        None
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "125 17"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 55312)
    }

    #[test]
    fn digits_example() {
        assert_eq!(digits(12345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn combine_example() {
        assert_eq!(combine(&vec![1, 2, 3, 4, 5]), 12345);
    }

    #[test]
    fn stone_count_after_blinks_example() {
        assert_eq!(stone_count_after_blinks(vec![0], 75), 22938365706844)
    }
}
