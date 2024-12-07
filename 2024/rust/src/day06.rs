use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

type Position = (usize, usize);

#[derive(PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (bounds, starting_position, obstacles) = parse(input);

    if let Some(positions) = walk(&bounds, starting_position, &obstacles) {
        //        print_map(&bounds, &obstacles, &positions);
        positions.len()
    } else {
        panic!("loop found")
    }
}

fn walk(
    bounds: &(Range<usize>, Range<usize>),
    starting_position: Position,
    obstacles: &Vec<Position>,
) -> Option<HashSet<Position>> {
    let mut position = starting_position;
    let mut next_position;
    let mut direction: Direction = Direction::North;
    let mut positions: HashSet<Position> = HashSet::new();
    let mut directions: HashMap<Position, Direction> = HashMap::new();

    while bounds.0.contains(&position.0) && bounds.1.contains(&position.1) {
        positions.insert(position);
        directions.insert(position, direction.clone());
        next_position = step(&direction, &position);

        while obstacles.contains(&next_position) {
            direction = turn_right(&direction);
            next_position = step(&direction, &position);
        }

        position = next_position;
        if positions.contains(&position)
            && directions.get(&position) == Some(&direction)
        {
            return None;
        }
    }

    return Some(positions);
}

#[allow(dead_code)]
fn print_map(
    bounds: &(Range<usize>, Range<usize>),
    obstacles: &Vec<Position>,
    positions: &HashSet<Position>,
) {
    for row in bounds.0.start..bounds.0.end {
        for col in bounds.1.start..bounds.1.end {
            if obstacles.contains(&(row, col)) {
                print!("#");
            } else {
                if positions.contains(&(row, col)) {
                    print!("X")
                } else {
                    print!(".")
                }
            }
        }
        println!()
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match *direction {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
    }
}

fn step(direction: &Direction, position: &Position) -> Position {
    match direction {
        Direction::North => (position.0 - 1, position.1),
        Direction::South => (position.0 + 1, position.1),
        Direction::East => (position.0, position.1 + 1),
        Direction::West => (position.0, position.1 - 1),
    }
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (bounds, starting_position, obstacles) = parse(input);
    let mut updated_obstacles: Vec<Position>;
    let mut loops: usize = 0;

    for row in bounds.0.start..bounds.0.end {
        for col in bounds.1.start..bounds.1.end {
            if !obstacles.contains(&(row, col)) {
                updated_obstacles = obstacles.clone();
                updated_obstacles.push((row, col));

                if walk(&bounds, starting_position, &updated_obstacles)
                    .is_none()
                {
                    loops += 1;
                }
            }
        }
    }

    loops
}

fn parse(
    input: &str,
) -> ((Range<usize>, Range<usize>), Position, Vec<Position>) {
    let mut obstacles: Vec<Position> = Vec::new();
    let mut starting_position: Position = (1, 1);
    let rows = input.lines().count();
    let mut cols: usize = 1;

    for (row, line) in input.lines().enumerate() {
        cols = line.chars().count();
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => obstacles.push((row + 1, col + 1)),
                '^' => starting_position = (row + 1, col + 1),
                _ => {}
            }
        }
    }

    ((1..rows + 1, 1..cols + 1), starting_position, obstacles)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 41)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 6)
    }
}
