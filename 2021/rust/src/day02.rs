use std::str::FromStr;

#[derive(Debug)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: usize,
}

impl FromStr for Command {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.split(" ").collect::<Vec<&str>>()[..] {
            ["up", amount_string] => Ok(Self {
                direction: Direction::Up,
                amount: amount_string.parse().unwrap(),
            }),
            ["down", amount_string] => Ok(Self {
                direction: Direction::Down,
                amount: amount_string.parse().unwrap(),
            }),
            ["forward", amount_string] => Ok(Self {
                direction: Direction::Forward,
                amount: amount_string.parse().unwrap(),
            }),
            _ => Err("invalid input"),
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let commands: Vec<Command> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut position = Position::default();

    for command in commands {
        match command.direction {
            Direction::Forward => position.horizontal += command.amount,
            Direction::Up => position.depth -= command.amount,
            Direction::Down => position.depth += command.amount,
        }
    }

    position.horizontal * position.depth
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let commands: Vec<Command> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut position = Position::default();

    for command in commands {
        match command.direction {
            Direction::Forward => {
                position.horizontal += command.amount;
                position.depth += position.aim * command.amount;
            }
            Direction::Up => position.aim -= command.amount,
            Direction::Down => position.aim += command.amount,
        }
    }

    position.horizontal * position.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n"),
            150
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n"),
            900
        )
    }
}
