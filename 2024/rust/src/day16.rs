use priority_queue::PriorityQueue;
use std::{
    cmp::{Ordering, Reverse},
    collections::{HashMap, HashSet, VecDeque},
};

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

type Path = Vec<Position>;
type Map = HashMap<Position, Tile>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (map, start_position, target_position) = parse(input);

    best_path_cost(&start_position, &target_position, &map)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (map, start_position, target_position) = parse(input);
    let best_cost = best_path_cost(&start_position, &target_position, &map);

    let best_positions: HashSet<Position> =
        best_paths(&start_position, &target_position, &map, best_cost)
            .into_iter()
            .flat_map(|path| path.into_iter())
            .collect();

    best_positions.len()
}

fn best_paths(
    start_position: &Position,
    target_position: &Position,
    map: &Map,
    best_cost: usize,
) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((vec![start_position.clone()], Direction::Right, 0));
    visited.insert((start_position.clone(), Direction::Right), 0);

    while let Some((current_path, current_direction, current_cost)) =
        queue.pop_front()
    {
        let current_position = current_path.last().unwrap();

        if current_position == target_position {
            paths.push(current_path);
            continue;
        }

        for next in neighbors(current_position, map) {
            let next_direction = direction(current_position, &next);
            let new_cost = current_cost
                + rotation_cost(&current_direction, &next_direction)
                + 1;

            // Skip paths that exceed the best cost
            if new_cost > best_cost {
                continue;
            }

            let state = (next.clone(), next_direction.clone());
            if let Some(&prev_cost) = visited.get(&state) {
                if new_cost > prev_cost {
                    continue;
                }
            }

            visited.insert(state, new_cost);

            let mut new_path = current_path.clone();
            new_path.push(next.clone());

            queue.push_back((new_path, next_direction, new_cost));
        }
    }

    paths
}

fn best_path_cost(
    start_position: &Position,
    target_position: &Position,
    map: &Map,
) -> usize {
    let mut frontier: PriorityQueue<Position, Reverse<usize>> =
        PriorityQueue::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut cost_so_far: HashMap<Position, (Direction, usize)> = HashMap::new();

    frontier.push(start_position.clone(), Reverse(0));
    cost_so_far.insert(start_position.clone(), (Direction::Right, 0));

    while !frontier.is_empty() {
        let (current, _priority) = frontier.pop().unwrap();

        if current == *target_position {
            break;
        }

        for next in neighbors(&current, map) {
            let (current_direction, cost) = cost_so_far.get(&current).unwrap();
            let next_direction = direction(&current, &next);
            let new_cost =
                cost + rotation_cost(&current_direction, &next_direction) + 1;

            //            println!(
            //                "{:?} -> {:?}, {:?} -> {:?}, {}",
            //                &current, &next, &current_direction, &next_direction, new_cost
            //            );

            let maybe_cost = cost_so_far.get(&next);
            if maybe_cost.is_none()
                || maybe_cost.map_or(false, |(_, cost)| new_cost < *cost)
            {
                cost_so_far.insert(next.clone(), (next_direction, new_cost));
                frontier.push(next.clone(), Reverse(new_cost));
                came_from.insert(next.clone(), current.clone());
            }
        }
    }

    let (_, cost) = cost_so_far.get(&target_position).unwrap();
    *cost
}

fn neighbors(position: &Position, map: &Map) -> Vec<Position> {
    vec![
        position.row.checked_sub(1).map(|r| (r, position.col)),
        position.col.checked_sub(1).map(|c| (position.row, c)),
        Some((position.row + 1, position.col)),
        Some((position.row, position.col + 1)),
    ]
    .into_iter()
    .filter_map(|maybe_position| {
        maybe_position.map(|(row, col)| Position { row, col })
    })
    .filter(|position| !map.contains_key(&position))
    .collect()
}

fn direction(from: &Position, to: &Position) -> Direction {
    match from.row.cmp(&to.row) {
        Ordering::Less => Direction::Down,
        Ordering::Equal => match from.col.cmp(&to.col) {
            Ordering::Less => Direction::Right,
            Ordering::Equal => panic!("stationary point found: {:?}", from),
            Ordering::Greater => Direction::Left,
        },
        Ordering::Greater => Direction::Up,
    }
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

    fn tiny_input() -> &'static str {
        "\
######
#...##
#.#.E#
#S..##
######"
    }

    #[allow(dead_code)]
    fn small_input() -> &'static str {
        "\
#########
#......E#
#..#..#.#
#S..#...#
#########"
    }

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
        assert_eq!(part1(tiny_input()), 2004);
        assert_eq!(part1(example_input()), 7036);
        assert_eq!(part1(second_example_input()), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(example_input()), 45);
        assert_eq!(part2(second_example_input()), 64);
    }
}
