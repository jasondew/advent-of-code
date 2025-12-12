use std::collections::{HashMap, HashSet};

type Position = (usize, usize);

#[derive(Debug)]
struct Manifold {
    start_position: Position,
    splitter_positions: Vec<Position>,
    height: usize,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let manifold = parse(input);
    let mut beam_positions = HashSet::<usize>::new();
    let mut splits = 0usize;

    beam_positions.insert(manifold.start_position.1);

    for row in 1..manifold.height {
        let current_positions: Vec<usize> =
            beam_positions.iter().cloned().collect();
        beam_positions.clear();

        for col in &current_positions {
            if manifold.splitter_positions.contains(&(row, *col)) {
                splits += 1;
                beam_positions.insert(*col - 1);
                beam_positions.insert(*col + 1);
            } else {
                beam_positions.insert(*col);
            }
        }
    }

    splits
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let manifold = parse(input);
    let mut beams: HashMap<usize, usize> = HashMap::new();

    beams.insert(manifold.start_position.1, 1);

    for row in 1..manifold.height {
        let mut new_beams: HashMap<usize, usize> = HashMap::new();
        let splitter_cols: Vec<usize> = manifold
            .splitter_positions
            .iter()
            .filter_map(|&(r, c)| if r == row { Some(c) } else { None })
            .collect();

        if splitter_cols.is_empty() {
            continue;
        } else {
            for (&col, &timelines) in &beams {
                if splitter_cols.contains(&col) {
                    new_beams
                        .entry(col - 1)
                        .and_modify(|t| *t += timelines)
                        .or_insert(timelines);
                    new_beams
                        .entry(col + 1)
                        .and_modify(|t| *t += timelines)
                        .or_insert(timelines);
                } else {
                    new_beams
                        .entry(col)
                        .and_modify(|t| *t += timelines)
                        .or_insert(timelines);
                }
            }

            beams = new_beams;
        }
    }

    beams.iter().map(|(_col, timelines)| timelines).sum()
}

fn parse(input: &str) -> Manifold {
    let mut start_position = (0, 0);
    let mut splitter_positions = Vec::new();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    start_position = (row, col);
                }
                '^' => {
                    splitter_positions.push((row, col));
                }
                _ => {}
            }
        }
    }

    Manifold {
        start_position,
        splitter_positions,
        height: input.trim().lines().count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 40);
    }
}
