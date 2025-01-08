use std::collections::HashMap;

#[must_use]
pub fn part1(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(parse_ints)
        .collect::<Vec<(usize, usize)>>();
    let (mut lefts, mut rights): (Vec<usize>, Vec<usize>) =
        pairs.into_iter().unzip();

    lefts.sort_unstable();
    rights.sort_unstable();

    let sorted_pairs = lefts.into_iter().zip(rights);

    sorted_pairs.into_iter().map(|(a, b)| a.abs_diff(b)).sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(parse_ints)
        .collect::<Vec<(usize, usize)>>();
    let (lefts, rights): (Vec<usize>, Vec<usize>) = pairs.into_iter().unzip();

    let lefts_with_counts = unique_values_with_counts(lefts);
    let rights_with_counts = unique_values_with_counts(rights);

    lefts_with_counts
        .into_iter()
        .map(|(value, left_count)| match rights_with_counts.get(&value) {
            Some(right_count) => value * left_count * right_count,
            None => 0,
        })
        .sum()
}

fn unique_values_with_counts(values: Vec<usize>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for value in values {
        *map.entry(value).or_insert(0) += 1;
    }

    map
}

fn parse_ints(line: &str) -> (usize, usize) {
    let ints = line
        .splitn(2, ' ')
        .map(|part| part.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    match ints.as_slice() {
        [a, b] => (*a, *b),
        _ => panic!("invalid input: {line}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
3   4
4   3
2   5
1   3
3   9
3   3"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 11)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 31)
    }
}
