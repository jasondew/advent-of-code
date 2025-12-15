#[must_use]
pub fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut total_joltage = 0usize;

    for line in lines {
        let joltages: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        total_joltage += maximum_two_digit_joltage(&joltages);
    }

    total_joltage
}

fn maximum_two_digit_joltage(joltages: &[usize]) -> usize {
    let mut largest_digit = 0usize;
    let mut largest_digit_position = 0usize;
    let mut second_largest_digit = 0usize;

    for (position, &joltage) in joltages.iter().enumerate() {
        if joltage > largest_digit && position < joltages.len() - 1 {
            largest_digit = joltage;
            largest_digit_position = position;
            second_largest_digit = 0;
        } else if joltage > second_largest_digit
            && position > largest_digit_position
        {
            second_largest_digit = joltage;
        }
    }

    largest_digit * 10 + second_largest_digit
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut total_joltage = 0usize;

    for line in lines {
        let joltages: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        total_joltage += maximum_twelve_digit_joltage(&joltages);
    }

    total_joltage
}

fn maximum_twelve_digit_joltage(joltages: &[usize]) -> usize {
    let joltage_count = joltages.len();
    let mut largest_digits = [0usize].repeat(12);
    let mut largest_digit_positions = [0usize].repeat(12);

    for (position, &joltage) in joltages.iter().enumerate() {
        for i in 0..12 {
            if joltage > largest_digits[i]
                && position < joltage_count - (12 - i - 1)
            {
                largest_digits[i] = joltage;
                largest_digit_positions[i] = position;
                for j in (i + 1)..12 {
                    largest_digits[j] = 0;
                    largest_digit_positions[j] = 0;
                }
                break;
            }
        }
    }

    largest_digits
        .into_iter()
        .reduce(|total, digit| total * 10 + digit)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn test_maximum_two_digit_joltage() {
        assert_eq!(
            maximum_two_digit_joltage(&[
                9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1
            ]),
            98
        );
        assert_eq!(
            maximum_two_digit_joltage(&[
                8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9
            ]),
            89
        );
        assert_eq!(
            maximum_two_digit_joltage(&[
                2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8
            ]),
            78
        );
        assert_eq!(
            maximum_two_digit_joltage(&[
                8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1
            ]),
            92
        );
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 3121910778619);
    }

    #[test]
    fn test_maximum_twelve_digit_joltage() {
        assert_eq!(
            maximum_twelve_digit_joltage(&[
                9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1
            ]),
            987654321111
        );
        assert_eq!(
            maximum_twelve_digit_joltage(&[
                8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9
            ]),
            811111111119
        );
        assert_eq!(
            maximum_twelve_digit_joltage(&[
                2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8
            ]),
            434234234278
        );
        assert_eq!(
            maximum_twelve_digit_joltage(&[
                8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1
            ]),
            888911112111
        );
    }
}
