use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
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

type Path = Vec<Position>;
type Walls = HashSet<Position>;

#[must_use]
pub fn part1(input: &str) -> usize {
    part1_with_params(input, 100)
}

pub fn part1_with_params(input: &str, minimum_savings: usize) -> usize {
    let (walls, start_position, target_position) = parse(input);
    let path = shortest_path(&start_position, &target_position, &walls);
    let mut count: usize = 0;

    for (index, position) in path.iter().enumerate() {
        for target_index in (index + 1)..path.len() {
            let target = path.get(target_index).unwrap();
            let distance = position.col.abs_diff(target.col)
                + position.row.abs_diff(target.row);

            if distance == 2 {
                let time_saved = target_index - index - 2;

                //            println!(
                //                "saved {} by cheating from {:?} to {:?}",
                //                time_saved, &position, &new_position
                //            );

                if time_saved >= minimum_savings {
                    count += 1;
                }
            }
        }
    }

    count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    part2_with_params(input, 100)
}

fn part2_with_params(input: &str, minimum_savings: usize) -> usize {
    let (walls, start_position, target_position) = parse(input);
    let path = shortest_path(&start_position, &target_position, &walls);
    let mut count: usize = 0;
    let mut count_by_time_saved: HashMap<usize, usize> = HashMap::new();

    for (index, position) in path.iter().enumerate() {
        for target_index in (index + 1)..path.len() {
            let target = path.get(target_index).unwrap();
            let distance = position.col.abs_diff(target.col)
                + position.row.abs_diff(target.row);

            if distance >= 2 && distance <= 20 {
                let time_saved = target_index - index - distance;

                //            println!(
                //                "saved {} by cheating from {:?} to {:?}",
                //                time_saved, &position, &new_position
                //            );

                if time_saved >= minimum_savings {
                    *count_by_time_saved.entry(time_saved).or_insert(0) += 1;
                    count += 1;
                }
            }
        }
    }

    //    dbg!(&count_by_time_saved);

    count
}

fn shortest_path(
    start_position: &Position,
    target_position: &Position,
    walls: &Walls,
) -> Path {
    let mut frontier: PriorityQueue<Position, Reverse<usize>> =
        PriorityQueue::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut cost_so_far: HashMap<Position, usize> = HashMap::new();

    frontier.push(start_position.clone(), Reverse(0));
    cost_so_far.insert(start_position.clone(), 0);

    while !frontier.is_empty() {
        let (current, _priority) = frontier.pop().unwrap();

        if current == *target_position {
            break;
        }

        for next in neighbors(&current, walls, false) {
            let new_cost = cost_so_far.get(&current).unwrap() + 1;
            let maybe_cost = cost_so_far.get(&next);

            if maybe_cost.map_or(true, |cost| new_cost < *cost) {
                cost_so_far.insert(next.clone(), new_cost);
                frontier.push(next.clone(), Reverse(new_cost));
                came_from.insert(next.clone(), current.clone());
            }
        }
    }

    let mut current_position = target_position;
    let mut positions = vec![target_position.clone()];

    while let Some(previous_position) = came_from.get(current_position) {
        positions.push(previous_position.clone());
        current_position = previous_position;
    }

    positions.into_iter().rev().collect()
}

fn neighbors(
    position: &Position,
    walls: &Walls,
    allow_walls: bool,
) -> Vec<Position> {
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
    .filter(|position| allow_walls || !walls.contains(&position))
    .collect()
}

#[allow(dead_code)]
fn print(walls: &Walls, path: &Path) {
    for row in 0..15 {
        for col in 0..15 {
            let position = Position { row, col };
            match walls.get(&position) {
                Some(_) => print!("#"),
                None => {
                    if path.contains(&position) {
                        print!("O")
                    } else {
                        print!(".")
                    }
                }
            }
        }
        println!();
    }
    println!();
}

fn parse(input: &str) -> (HashSet<Position>, Position, Position) {
    let mut walls = HashSet::new();
    let mut start_position: Option<Position> = None;
    let mut end_position: Option<Position> = None;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    walls.insert(Position { row, col });
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

    (
        walls,
        start_position.expect("start position"),
        end_position.expect("end position"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1_with_params(input(), 1), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_with_params(input(), 50), 285);
    }
}
