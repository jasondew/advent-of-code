use std::{collections::HashMap, ops::Range};

type Bounds = (Range<usize>, Range<usize>);
type Position = (usize, usize);
type Map = HashMap<Position, char>;
type Antennas = HashMap<char, Vec<Position>>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (bounds, _map, antennas) = parse(input);
    let mut antinodes: HashMap<Position, char> = HashMap::new();

    for (frequency, positions) in antennas {
        for (a, b) in position_pairs(&positions) {
            for antinode_position in antinode_positions(a, b, &bounds) {
                antinodes.insert(antinode_position, frequency);
            }
        }
    }

    antinodes.len()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (bounds, _map, antennas) = parse(input);
    let mut antinodes: HashMap<Position, char> = HashMap::new();

    for (frequency, positions) in antennas {
        for (a, b) in position_pairs(&positions) {
            for antinode_position in repeating_antinode_positions(a, b, &bounds)
            {
                antinodes.insert(antinode_position, frequency);
            }
        }
    }

    antinodes.len()
}

fn position_pairs(positions: &[Position]) -> Vec<(&Position, &Position)> {
    let mut pairs: Vec<(&Position, &Position)> = Vec::new();

    for (index, start_position) in positions.iter().enumerate() {
        for end_position in positions.iter().skip(index + 1) {
            pairs.push((start_position, end_position));
        }
    }

    pairs
}

fn antinode_positions(
    a: &Position,
    b: &Position,
    bounds: &Bounds,
) -> Vec<Position> {
    let mut antinodes: Vec<Position> = Vec::new();
    let mut candidate: (isize, isize);

    let delta_y = b.0 as isize - a.0 as isize;
    let delta_x = b.1 as isize - a.1 as isize;

    candidate = (a.0 as isize - delta_y, a.1 as isize - delta_x);
    if let Some(position) = suitable_candidate(candidate, bounds) {
        antinodes.push(position);
    }

    candidate = (b.0 as isize + delta_y, b.1 as isize + delta_x);
    if let Some(position) = suitable_candidate(candidate, bounds) {
        antinodes.push(position);
    }

    antinodes
}

fn repeating_antinode_positions(
    a: &Position,
    b: &Position,
    bounds: &Bounds,
) -> Vec<Position> {
    let mut antinodes: Vec<Position> = Vec::new();
    let mut candidate: (isize, isize);

    let delta_y = b.0 as isize - a.0 as isize;
    let delta_x = b.1 as isize - a.1 as isize;

    candidate = (a.0 as isize - delta_y, a.1 as isize - delta_x);
    while let Some(position) = suitable_candidate(candidate, bounds) {
        antinodes.push(position);
        candidate = (candidate.0 - delta_y, candidate.1 - delta_x);
    }

    candidate = (b.0 as isize + delta_y, b.1 as isize + delta_x);
    while let Some(position) = suitable_candidate(candidate, bounds) {
        antinodes.push(position);
        candidate = (candidate.0 + delta_y, candidate.1 + delta_x);
    }

    antinodes.push(*a);
    antinodes.push(*b);

    antinodes
}

fn suitable_candidate(
    candidate: (isize, isize),
    bounds: &Bounds,
) -> Option<Position> {
    if candidate.0 >= 0 && candidate.1 >= 0 {
        let position: Position = (candidate.0 as usize, candidate.1 as usize);
        if in_bounds(&position, bounds) {
            Some(position)
        } else {
            None
        }
    } else {
        None
    }
}

fn in_bounds(position: &Position, bounds: &Bounds) -> bool {
    bounds.0.contains(&position.0) && bounds.1.contains(&position.1)
}

fn parse(input: &str) -> (Bounds, Map, Antennas) {
    let bounds: Bounds = (
        0..input.lines().count(),
        0..input.lines().next().unwrap().len(),
    );
    let mut map: Map = HashMap::new();
    let mut antennas: Antennas = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => {}
                ch => {
                    map.insert((row, col), ch);
                    antennas.entry(char).or_default().push((row, col));
                }
            }
        }
    }

    (bounds, map, antennas)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 14)
    }

    #[test]
    fn position_pairs_tests() {
        let simple_input = "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

        let (_bounds, _map, antennas) = parse(simple_input);
        let a_positions = antennas.get(&'a').unwrap();

        assert_eq!(position_pairs(a_positions), vec![(&(3, 4), &(5, 5))]);
    }

    #[test]
    fn antinode_positions_tests() {
        let bounds = ((0..10), (0..10));

        assert_eq!(
            antinode_positions(&(3, 4), &(5, 5), &bounds),
            vec![(1, 3), (7, 6)]
        );
        assert_eq!(antinode_positions(&(3, 4), &(4, 8), &bounds), vec![(2, 0)]);
        assert_eq!(antinode_positions(&(0, 0), &(9, 9), &bounds), vec![]);
        assert_eq!(antinode_positions(&(1, 8), &(2, 5), &bounds), vec![(3, 2)]);
    }

    #[test]
    fn repeating_antinode_positions_tests() {
        let bounds = ((0..10), (0..10));

        assert_eq!(
            repeating_antinode_positions(&(0, 0), &(1, 3), &bounds),
            vec![(2, 6), (3, 9), (0, 0), (1, 3)]
        );
        assert_eq!(
            repeating_antinode_positions(&(0, 0), &(2, 1), &bounds),
            vec![(4, 2), (6, 3), (8, 4), (0, 0), (2, 1)]
        );
    }

    #[test]
    fn part2_example() {
        let simple_input = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

        assert_eq!(part2(simple_input), 9);
        assert_eq!(part2(input()), 34)
    }
}
