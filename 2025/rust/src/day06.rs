#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> usize {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
        }
    }
}

#[derive(Debug)]
struct Homework {
    problems: Vec<Problem>,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let homework = parse(input);

    homework.problems.iter().map(Problem::solve).sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let homework = parse_right_to_left(input);

    homework.problems.iter().map(Problem::solve).sum()
}

fn parse(input: &str) -> Homework {
    let lines: Vec<&str> = input.trim().lines().collect();

    let number_lines = &lines[..lines.len() - 1];
    let operation_line = &lines[lines.len() - 1];

    let mut problems: Vec<Problem> = operation_line
        .split_whitespace()
        .map(|c| match c {
            "*" => Problem {
                numbers: Vec::new(),
                operation: Operation::Multiply,
            },
            "+" => Problem {
                numbers: Vec::new(),
                operation: Operation::Add,
            },
            _ => panic!("Unexpected operation: {c}"),
        })
        .collect();

    for line in number_lines {
        for (i, number_string) in line.split_whitespace().enumerate() {
            problems[i].numbers.push(number_string.parse().unwrap());
        }
    }

    Homework { problems }
}

fn parse_right_to_left(input: &str) -> Homework {
    let lines: Vec<&str> = input.trim().lines().collect();

    let number_lines = &lines[..lines.len() - 1];
    let operation_line = &lines[lines.len() - 1];

    let mut problems: Vec<Problem> = operation_line
        .split_whitespace()
        .map(|c| match c {
            "*" => Problem {
                numbers: Vec::new(),
                operation: Operation::Multiply,
            },
            "+" => Problem {
                numbers: Vec::new(),
                operation: Operation::Add,
            },
            _ => panic!("Unexpected operation: {c}"),
        })
        .collect();

    let number_chars: Vec<Vec<char>> = number_lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let rows = number_chars.len();
    let cols = number_chars.first().map_or(0, std::vec::Vec::len);

    let mut number_chars_transposed: Vec<Vec<char>> =
        vec![vec![' '; rows]; cols];

    for (row, number_row) in number_chars.iter().enumerate() {
        for (col, number_char) in number_row.iter().enumerate() {
            number_chars_transposed[col][row] = *number_char;
        }
    }

    let mut problem_number = 0;
    let mut numbers: Vec<usize> = Vec::new();

    for number_vec in &number_chars_transposed {
        if number_vec.iter().all(|c| c.is_whitespace()) {
            problems[problem_number].numbers = numbers;
            problem_number += 1;
            numbers = Vec::new();
        } else {
            numbers.push(
                number_vec
                    .iter()
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap(),
            );
        }
    }
    problems.last_mut().unwrap().numbers = numbers;

    Homework { problems }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 4277556);
    }

    #[test]
    fn test_problem_solve() {
        let problem_add = Problem {
            numbers: vec![123, 45, 6],
            operation: Operation::Multiply,
        };
        assert_eq!(problem_add.solve(), 33210);

        let problem_multiply = Problem {
            numbers: vec![328, 64, 98],
            operation: Operation::Add,
        };
        assert_eq!(problem_multiply.solve(), 490);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 3263827);
    }
}
