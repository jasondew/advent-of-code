use std::ops::RangeInclusive;

type Range = RangeInclusive<usize>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let range_pairs = parse(input);
    let mut count: usize = 0;

    for (left, right) in range_pairs {
        if proper_subset(&left, &right) || proper_subset(&right, &left) {
            count += 1;
        }
    }

    count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let range_pairs = parse(input);
    let mut count: usize = 0;

    for (left, right) in range_pairs {
        if overlapping(&left, &right) {
            count += 1;
        }
    }

    count
}

fn overlapping(left: &Range, right: &Range) -> bool {
    left.contains(right.start())
        || left.contains(right.end())
        || proper_subset(left, right)
        || proper_subset(right, left)
}

fn proper_subset(left: &Range, right: &Range) -> bool {
    left.contains(right.start()) && left.contains(right.end())
}

fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let (left, right) =
                line.trim().split_once(',').expect("pair of ranges");
            (parse_range(left), parse_range(right))
        })
        .collect()
}

fn parse_range(input: &str) -> Range {
    let (from_string, to_string) = input.split_once('-').expect("range");
    let from: usize = from_string.parse().unwrap();
    let to: usize = to_string.parse().unwrap();

    from..=to
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 2)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 4)
    }
}
