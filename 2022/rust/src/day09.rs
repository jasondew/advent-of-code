use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Motion {
    direction: Direction,
    amount: usize,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl Position {
    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn go_towards(&mut self, target: &Position) {
        if target.x > self.x {
            self.x += 1;
        }
        if target.x < self.x {
            self.x -= 1;
        }

        if target.y > self.y {
            self.y += 1;
        }
        if target.y < self.y {
            self.y -= 1;
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let motions = parse(input);
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(tail);

    for motion in motions {
        for _ in 0..motion.amount {
            head.go(motion.direction);

            if non_contiguous(&head, &tail) {
                tail.go_towards(&head);
                visited.insert(tail);
            }
        }
    }

    visited.len()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let motions = parse(input);
    let mut head = Position::default();
    let mut knots = Vec::new();
    for _ in 0..9 {
        knots.push(Position::default());
    }

    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(knots[8]);

    for motion in motions {
        for _ in 0..motion.amount {
            head.go(motion.direction);

            let mut previous_knot = &head;

            for knot in &mut knots {
                if non_contiguous(previous_knot, knot) {
                    knot.go_towards(previous_knot);
                }
                previous_knot = knot;
            }
            visited.insert(knots[8]);
        }
    }

    visited.len()
}

fn non_contiguous(a: &Position, b: &Position) -> bool {
    (a.x - b.x).abs() > 1 || (a.y - b.y).abs() > 1
}

fn parse(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let (direction_string, amount_string) =
                line.trim().split_once(' ').unwrap();
            let direction = match direction_string {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("invalid direction"),
            };
            let amount = amount_string.parse::<usize>().unwrap();
            Motion { direction, amount }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2\n"
    }

    fn larger_input() -> &'static str {
        "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 13)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(larger_input()), 36)
    }
}
