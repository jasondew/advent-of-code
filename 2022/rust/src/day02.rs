#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from_code(code: &str) -> Self {
        match code {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            other => panic!("expected valid shape code, got {other}"),
        }
    }

    fn from_encrypted(code: &str) -> Self {
        match code {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            other => panic!("expected valid encrypted shape code, got {other}"),
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let pairs = parse(input);
    let mut score: usize = 0;

    for (opponent, suggestion) in pairs {
        let opponent_shape = Shape::from_code(opponent);
        let suggested_shape = Shape::from_encrypted(suggestion);
        score += shape_score(&suggested_shape)
            + outcome_score(&suggested_shape, &opponent_shape);
    }

    score
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let pairs = parse(input);
    let mut score: usize = 0;

    for (opponent, desired_outcome) in pairs {
        let opponent_shape = Shape::from_code(opponent);
        let your_shape = match desired_outcome {
            "X" => losing_shape(&opponent_shape),
            "Y" => opponent_shape,
            "Z" => winning_shape(&opponent_shape),
            other => panic!("expected valid desired outcome code, got {other}"),
        };

        score += shape_score(&your_shape)
            + outcome_score(&your_shape, &opponent_shape);
    }

    score
}

fn winning_shape(opponent_shape: &Shape) -> Shape {
    match opponent_shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn losing_shape(opponent_shape: &Shape) -> Shape {
    match opponent_shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn shape_score(shape: &Shape) -> usize {
    *shape as usize
}

fn outcome_score(yours: &Shape, opponents: &Shape) -> usize {
    use Shape::{Paper, Rock, Scissors};

    match (yours, opponents) {
        (Rock, Rock) => 3,
        (Rock, Paper) => 0,
        (Rock, Scissors) => 6,
        (Scissors, Rock) => 0,
        (Scissors, Paper) => 6,
        (Scissors, Scissors) => 3,
        (Paper, Rock) => 6,
        (Paper, Paper) => 3,
        (Paper, Scissors) => 0,
    }
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let mut pair = line.split_whitespace();
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
        A Y
        B X
        C Z\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 15)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 12)
    }
}
