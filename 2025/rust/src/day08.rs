use std::{
    cmp::Reverse,
    collections::HashSet,
    fmt::{self, Debug, Formatter},
};

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:3}, {:3}, {:3})", self.x, self.y, self.z)
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    part1_with_count(input, 1000)
}

pub fn part1_with_count(input: &str, count: usize) -> usize {
    let box_positions = parse(input);
    let distances = all_pairs_distances(&box_positions);
    let mut circuits: Vec<HashSet<&Position>> = Vec::new();

    for (from, to, _distance) in distances.iter().take(count) {
        let mut matching_circuits: Vec<&mut HashSet<&Position>> = circuits
            .iter_mut()
            .filter(|nodes| nodes.contains(from) || nodes.contains(to))
            .collect();

        match &mut matching_circuits[..] {
            [] => {
                let mut hash_set = HashSet::<&Position>::new();
                hash_set.insert(from);
                hash_set.insert(to);
                circuits.push(hash_set);
            }
            [circuit1, circuit2] => {
                circuit1.insert(from);
                circuit1.insert(to);
                for position in circuit2.drain() {
                    circuit1.insert(position);
                }
            }
            [circuit] => {
                circuit.insert(from);
                circuit.insert(to);
            }
            _ => panic!("More than two matching circuits found"),
        }
    }

    circuits.sort_unstable_by_key(|nodes| Reverse(nodes.len()));
    circuits.iter().take(3).map(|nodes| nodes.len()).product()
}

fn all_pairs_distances(
    positions: &[Position],
) -> Vec<(&Position, &Position, usize)> {
    let mut pair_distances = Vec::new();

    for (i, a) in positions.iter().enumerate() {
        for b in positions.iter().skip(i + 1) {
            let distance = ((a.x as isize - b.x as isize).pow(2)
                + (a.y as isize - b.y as isize).pow(2)
                + (a.z as isize - b.z as isize).pow(2))
                as usize;
            pair_distances.push((a, b, distance));
        }
    }

    pair_distances.sort_unstable_by_key(|&(_, _, dist)| dist);
    pair_distances
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let box_positions = parse(input);
    let distances = all_pairs_distances(&box_positions);
    let mut circuits: Vec<HashSet<&Position>> = Vec::new();
    let mut max_circuit_size = 0usize;
    let mut answer = 0usize;

    for (from, to, _distance) in distances.iter() {
        let mut matching_circuits: Vec<&mut HashSet<&Position>> = circuits
            .iter_mut()
            .filter(|nodes| nodes.contains(from) || nodes.contains(to))
            .collect();

        match &mut matching_circuits[..] {
            [] => {
                let mut hash_set = HashSet::<&Position>::new();
                hash_set.insert(from);
                hash_set.insert(to);
                circuits.push(hash_set);
            }
            [circuit1, circuit2] => {
                circuit1.insert(from);
                circuit1.insert(to);
                for position in circuit2.drain() {
                    circuit1.insert(position);
                }
            }
            [circuit] => {
                circuit.insert(from);
                circuit.insert(to);
            }
            _ => panic!("More than two matching circuits found"),
        }

        circuits.retain(|circuit| !circuit.is_empty());

        let max = circuits.iter().map(|c| c.len()).max().unwrap_or(0);
        if max > max_circuit_size {
            max_circuit_size = max;
            answer = from.x * to.x;
        }
    }

    answer
}

fn parse(input: &str) -> Vec<Position> {
    input
        .trim()
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line
                .trim()
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            Position {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! pos {
        ($x:expr, $y:expr, $z:expr) => {
            Position {
                x: $x,
                y: $y,
                z: $z,
            }
        };
    }

    const EXAMPLE_INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1_with_count(EXAMPLE_INPUT, 10), 40);
    }

    #[test]
    fn test_all_pairs_distances() {
        let positions = parse(EXAMPLE_INPUT);
        let distances = all_pairs_distances(&positions);

        assert!(matches!(
            distances[0],
            (pos!(162, 817, 812), pos!(425, 690, 689), _)
        ));
        assert!(matches!(
            distances[1],
            (pos!(162, 817, 812), pos!(431, 825, 988), _)
        ));
        assert!(matches!(
            distances[2],
            (pos!(906, 360, 560), pos!(805, 96, 715), _)
        ));
        assert!(matches!(
            distances[3],
            (pos!(431, 825, 988), pos!(425, 690, 689), _)
        ));
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 25272);
    }
}
