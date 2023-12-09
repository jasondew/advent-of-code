type Sequence = Vec<isize>;

#[must_use]
pub fn part1(input: &str) -> isize {
    let sequences = parse(input);

    let extrapolates: Vec<isize> = sequences
        .iter()
        .map(|sequence| extrapolate(sequence))
        .collect();

    extrapolates.iter().sum()
}

#[must_use]
pub fn part2(input: &str) -> isize {
    let sequences: Vec<Sequence> = parse(input)
        .into_iter()
        .map(|sequence| sequence.into_iter().rev().collect())
        .collect();

    let extrapolates: Vec<isize> = sequences
        .iter()
        .map(|sequence| extrapolate(sequence))
        .collect();

    extrapolates.iter().sum()
}

fn derivative(sequence: &Sequence) -> Sequence {
    sequence
        .windows(2)
        .map(|window| match &window[..] {
            &[x, y] => y - x,
            _ => panic!("got shorted on a window"),
        })
        .collect()
}

fn tail(sequence: &Sequence) -> isize {
    sequence[sequence.len() - 1]
}

fn extrapolate(initial_sequence: &Sequence) -> isize {
    let mut sequence = initial_sequence.clone();
    let mut tails = vec![tail(initial_sequence)];

    loop {
        let derivative = derivative(&sequence);
        tails.push(tail(&derivative));

        let first_element = derivative[0];
        if derivative.iter().all(|element| element == &first_element) {
            break;
        }

        sequence = derivative;
    }

    tails.iter().sum()
}

fn parse(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 114)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 2)
    }
}
