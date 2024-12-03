use regex::RegexBuilder;

#[derive(Debug)]
enum Operation {
    Enable,
    Disable,
    Multiply(usize, usize),
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let operations: Vec<Operation> = parse(input);
    evaluate(operations)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let operations: Vec<Operation> = parse(input);
    evaluate(operations)
}

fn evaluate(operations: Vec<Operation>) -> usize {
    let mut enabled: bool = true;
    let mut total: usize = 0;

    for operation in operations.into_iter() {
        match operation {
            Operation::Multiply(first, second) => {
                if enabled {
                    total += first * second
                }
            }
            Operation::Enable => enabled = true,
            Operation::Disable => enabled = false,
        }
    }

    total
}

fn parse(input: &str) -> Vec<Operation> {
    let regex = RegexBuilder::new(r"(do[n't]*)(\(\))|mul\((\d+),(\d+)\)")
        .multi_line(true)
        .build()
        .unwrap();
    regex
        .captures_iter(input)
        .map(|capture| match capture.extract() {
            (_, ["do", _]) => Operation::Enable,
            (_, ["don't", _]) => Operation::Disable,
            (_, [first, second]) => {
                Operation::Multiply(parse_int(first), parse_int(second))
            }
        })
        .collect()
}

fn parse_int(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)
+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 161)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
        assert_eq!(part2("don't()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)\nundo()?mul(8,5))"), 40);
    }
}
