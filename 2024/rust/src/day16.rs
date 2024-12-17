use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

type Path = (Vec<Position>, Vec<Direction>);

#[must_use]
pub fn part1(input: &str) -> usize {
    let (map, start_position, end_position) = parse(input);
    let mut best_path: Path = (Vec::new(), Vec::new());
    let mut best_cost: usize = usize::max_value();

    for path in all_paths(start_position, end_position, map) {
        let cost = path_cost(&path);
        if cost < best_cost {
            best_cost = cost;
            best_path = path;
        }
    }

    dbg!(&best_path);

    best_cost
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.lines().count()
}

fn path_cost((positions, directions): &Path) -> usize {
    let mut cost: usize = positions.len() - 1;
    let mut current_direction = &Direction::Right;

    for direction in directions {
        cost += rotation_cost(current_direction, direction);
        current_direction = direction;
    }

    cost
}

fn rotation_cost(from: &Direction, to: &Direction) -> usize {
    1000 * match from {
        Direction::Up => match to {
            Direction::Up => 0,
            Direction::Down => 2,
            Direction::Left => 1,
            Direction::Right => 1,
        },
        Direction::Down => match to {
            Direction::Up => 2,
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => 1,
        },
        Direction::Left => match to {
            Direction::Up => 1,
            Direction::Down => 1,
            Direction::Left => 0,
            Direction::Right => 2,
        },
        Direction::Right => match to {
            Direction::Up => 1,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        },
    }
}

fn all_paths(
    start_position: Position,
    end_position: Position,
    map: HashMap<Position, Tile>,
) -> Vec<Path> {
    Vec::new()
}

fn parse(input: &str) -> (HashMap<Position, Tile>, Position, Position) {
    let mut map = HashMap::new();
    let mut start_position: Option<Position> = None;
    let mut end_position: Option<Position> = None;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    map.insert(Position { row, col }, Tile::Wall);
                }
                '.' => {}
                'S' => {
                    start_position = Some(Position { row, col });
                }
                'E' => {
                    end_position = Some(Position { row, col });
                }
                _ => panic!("invalid character seen: {}", char),
            }
        }
    }

    (map, start_position.unwrap(), end_position.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
    }

    fn second_example_input() -> &'static str {
        "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(example_input()), 7036);
        assert_eq!(part1(second_example_input()), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(example_input()), 15);
    }
}
