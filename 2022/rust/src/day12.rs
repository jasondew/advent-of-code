use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Height(usize);

impl Debug for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Position(usize, usize);

impl Position {
    fn distance(&self, position: Position) -> usize {
        self.0.abs_diff(position.0) + self.1.abs_diff(position.1)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

type HeightMap = HashMap<Position, Height>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (height_map, start_position, target_position) = parse(input);

    shortest_path(&height_map, start_position, target_position)
        .unwrap()
        .len()
        - 1
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (height_map, _start_position, target_position) = parse(input);

    height_map
        .iter()
        .filter_map(
            |(position, height)| {
                if height.0 == 0 {
                    Some(position)
                } else {
                    None
                }
            },
        )
        .filter_map(|&start_position| {
            shortest_path(&height_map, start_position, target_position)
        })
        .map(|path| path.len() - 1)
        .min()
        .unwrap()
}

fn shortest_path(
    height_map: &HeightMap,
    start_position: Position,
    target_position: Position,
) -> Option<Vec<Position>> {
    let mut cost_so_far: HashMap<Position, usize> = HashMap::new();
    let mut came_from: HashMap<Position, Option<Position>> = HashMap::new();
    let mut frontier = BinaryHeap::new();

    cost_so_far.insert(start_position, 0);
    came_from.insert(start_position, None);
    frontier.push(Reverse((0, start_position)));

    while let Some(Reverse((_priority, position))) = frontier.pop() {
        if position == target_position {
            let mut path = vec![position];
            while let Some(Some(from)) = came_from.get(path.last().unwrap()) {
                path.push(*from);
            }

            return Some(path);
        }

        for next_position in possible_next_positions(height_map, &position) {
            let new_cost = cost_so_far.get(&position).unwrap() + 1;
            let maybe_next_cost = cost_so_far.get(&next_position);

            if maybe_next_cost.is_none() || new_cost < *maybe_next_cost.unwrap()
            {
                let priority =
                    new_cost + next_position.distance(target_position);
                frontier.push(Reverse((priority, next_position)));
                cost_so_far.insert(next_position, new_cost);
                came_from.insert(next_position, Some(position));
            }
        }
    }

    None
}

fn possible_next_positions(
    height_map: &HeightMap,
    position: &Position,
) -> Vec<Position> {
    let current_height = height_map.get(position).unwrap();
    let mut positions: Vec<Position> = vec![];

    positions.push(Position(position.0, position.1 + 1));

    if position.1 > 1 {
        positions.push(Position(position.0, position.1 - 1));
    }

    positions.push(Position(position.0 + 1, position.1));
    if position.0 > 0 {
        positions.push(Position(position.0 - 1, position.1));
    }

    positions
        .into_iter()
        .filter(|position| match height_map.get(position) {
            Some(height) => height.0 <= current_height.0 + 1,
            None => false,
        })
        .collect()
}

fn parse(input: &str) -> (HeightMap, Position, Position) {
    let mut start_position: Position = Position(0, 0);
    let mut target_position: Position = Position(0, 0);

    let height_map: HeightMap = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    'S' => {
                        start_position = Position(row, col);
                        (Position(row, col), Height(0))
                    }
                    'E' => {
                        target_position = Position(row, col);
                        (Position(row, col), Height(25))
                    }
                    _ => (Position(row, col), Height(ch as usize - 97)),
                })
                .collect::<Vec<(Position, Height)>>()
        })
        .collect();

    (height_map, start_position, target_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 31)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 29)
    }
}
