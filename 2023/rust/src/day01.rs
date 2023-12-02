#[must_use]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<usize> = numbers(line);
            numbers[numbers.len() - 1] + 10 * numbers[0]
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<usize> = numbers_enhanced(line);
            numbers[numbers.len() - 1] + 10 * numbers[0]
        })
        .sum()
}

fn numbers(line: &str) -> Vec<usize> {
    line.chars()
        .filter_map(|ch| ch.to_digit(10).map(|d| d as usize))
        .collect()
}

fn numbers_enhanced(line: &str) -> Vec<usize> {
    let mut digits_with_indices: Vec<(usize, usize)> = line
        .chars()
        .enumerate()
        .filter_map(|(index, ch)| ch.to_digit(10).map(|d| (index, d as usize)))
        .collect();

    if line.len() > 2 {
        for offset in 0..(line.len() - 2) {
            match &line[offset..(offset + 3)] {
                "one" => digits_with_indices.push((offset, 1)),
                "two" => digits_with_indices.push((offset, 2)),
                "six" => digits_with_indices.push((offset, 6)),
                _ => (),
            }
        }
    }

    if line.len() > 3 {
        for offset in 0..(line.len() - 3) {
            match &line[offset..(offset + 4)] {
                "four" => digits_with_indices.push((offset, 4)),
                "five" => digits_with_indices.push((offset, 5)),
                "nine" => digits_with_indices.push((offset, 9)),
                _ => (),
            }
        }
    }

    if line.len() > 4 {
        for offset in 0..(line.len() - 4) {
            match &line[offset..(offset + 5)] {
                "three" => digits_with_indices.push((offset, 3)),
                "seven" => digits_with_indices.push((offset, 7)),
                "eight" => digits_with_indices.push((offset, 8)),
                _ => (),
            }
        }
    }

    digits_with_indices.sort();

    digits_with_indices
        .into_iter()
        .map(|(_index, digit)| digit)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input1() -> &'static str {
        "1abc2
         pqr3stu8vwx
         a1b2c3d4e5f
         treb7uchet"
    }

    fn input2() -> &'static str {
        "two1nine
         eightwothree
         abcone2threexyz
         xtwone3four
         4nineeightseven2
         zoneight234
         7pqrstsixteen"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input1()), 142)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input2()), 281)
    }
}
