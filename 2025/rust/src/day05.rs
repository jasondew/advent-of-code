use std::ops::RangeInclusive;

struct Inventory {
    ranges: Vec<RangeInclusive<usize>>,
    ids: Vec<usize>,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let inventory = parse(input);
    let mut fresh_count = 0;

    for id in inventory.ids {
        if inventory.ranges.iter().any(|range| range.contains(&id)) {
            fresh_count += 1;
        }
    }

    fresh_count
}

fn parse(input: &str) -> Inventory {
    let (ranges_input, ids_input) = input.trim().split_once("\n\n").unwrap();
    let mut ranges = Vec::<RangeInclusive<usize>>::new();
    let mut ids = Vec::<usize>::new();

    for line in ranges_input.lines() {
        let (start, end) = line.trim().split_once('-').unwrap();
        ranges.push(RangeInclusive::new(
            start.parse().unwrap(),
            end.parse().unwrap(),
        ));
    }

    for line in ids_input.lines() {
        ids.push(line.trim().parse().unwrap());
    }

    Inventory { ranges, ids }
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let inventory = parse(input);
    let ranges = unify(inventory.ranges);

    ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn unify(ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by_key(|range| *range.start());
    let mut unified_ranges = Vec::<RangeInclusive<usize>>::new();

    for range in sorted_ranges {
        if let Some(last_range) = unified_ranges.last_mut()
            && *last_range.end() >= *range.start()
        {
            *last_range = RangeInclusive::new(
                *last_range.start(),
                (*last_range.end()).max(*range.end()),
            );
        } else {
            unified_ranges.push(range);
        }
    }

    unified_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn test_unify() {
        let ranges = vec![
            RangeInclusive::new(3, 5),
            RangeInclusive::new(10, 14),
            RangeInclusive::new(16, 20),
            RangeInclusive::new(12, 18),
        ];
        assert_eq!(
            unify(ranges),
            vec![RangeInclusive::new(3, 5), RangeInclusive::new(10, 20),]
        );
    }
}
