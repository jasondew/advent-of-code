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
enum Command {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (direction, amount_string) = input.split_once(" ").ok_or("invalid command")?;
        let amount = amount_string.parse().map_err(|_| "invalid amount")?;

        match direction {
            "up" => Ok(Self::Up(amount)),
            "down" => Ok(Self::Down(amount)),
            "forward" => Ok(Self::Forward(amount)),
            _ => Err("invalid direction"),
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let commands: Vec<Command> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut position = Position::default();

    for command in commands {
        match command {
            Command::Forward(amount) => position.horizontal += amount,
            Command::Up(amount) => position.depth -= amount,
            Command::Down(amount) => position.depth += amount,
        }
    }

    position.horizontal * position.depth
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let commands: Vec<Command> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut position = Position::default();

    for command in commands {
        match command {
            Command::Forward(amount) => {
                position.horizontal += amount;
                position.depth += position.aim * amount;
            }
            Command::Up(amount) => position.aim -= amount,
            Command::Down(amount) => position.aim += amount,
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
