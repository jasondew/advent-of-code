#[must_use]
pub fn part1(input: &str) -> usize {
    let rucksacks: Vec<&str> = parse(input);
    let split_rucksacks: Vec<(&str, &str)> = rucksacks
        .into_iter()
        .map(|line| line.split_at(line.len() / 2))
        .collect();
    let mut common_chars: Vec<char> = Vec::new();

    for (left, right) in split_rucksacks {
        for left_char in left.chars() {
            if right.contains(left_char) {
                common_chars.push(left_char);
                break;
            }
        }
    }

    common_chars.iter().map(|ch| priority(ch)).sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let rucksacks: Vec<&str> = parse(input);
    let mut common_chars: Vec<char> = Vec::new();

    for group in rucksacks.chunks(3) {
        let first_rucksack: &str = group.iter().next().unwrap();
        let other_rucksacks: Vec<&str> =
            group.iter().skip(1).cloned().collect();

        for ch in first_rucksack.chars() {
            if other_rucksacks.iter().all(|rucksack| rucksack.contains(ch)) {
                common_chars.push(ch);
                break;
            }
        }
    }

    common_chars.iter().map(|ch| priority(ch)).sum()
}

fn priority(ch: &char) -> usize {
    let code: usize = *ch as usize;

    if code > 96 {
        code - 96
    } else {
        code - 38
    }
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 157)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 70)
    }
}
