#[must_use]
pub fn part1(input: &str) -> usize {
    let reports = parse(input);

    reports.into_iter().filter(|report| is_safe(report)).count()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let reports = parse(input);

    reports
        .into_iter()
        .filter(|report| {
            is_safe(report)
                || permutations_holding_one_out(report)
                    .iter()
                    .any(|r| is_safe(r))
        })
        .count()
}

fn permutations_holding_one_out(report: &Vec<usize>) -> Vec<Vec<usize>> {
    (1..report.len() + 1)
        .map(|pivot| {
            let (left, right) = report.split_at(pivot);
            let left_drop_last = &left[..left.len() - 1];
            [left_drop_last, right].concat()
        })
        .collect()
}

fn is_safe(report: &Vec<usize>) -> bool {
    let derivative: Vec<isize> = derivative(report);
    all_increasing_or_decreasing(&derivative) && within_range(&derivative)
}

fn all_increasing_or_decreasing(values: &Vec<isize>) -> bool {
    let min = *values.iter().min().unwrap();
    let max = *values.iter().max().unwrap();

    (min > 0isize && max > 0isize) || (min < 0isize && max < 0isize)
}

fn within_range(values: &Vec<isize>) -> bool {
    values
        .iter()
        .all(|value| value.abs() >= 1 && value.abs() <= 3)
}

fn derivative(values: &Vec<usize>) -> Vec<isize> {
    values
        .windows(2)
        .map(|window| match window {
            &[first, second] => second as isize - first as isize,
            _ => panic!("invalid window"),
        })
        .collect()
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|ch| ch.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
9 2 3 4 5
1 9 3 4 5
1 2 9 4 5
1 2 3 9 5
1 2 3 4 9"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 2)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 9)
    }
}
