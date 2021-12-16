use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i8,
    y: i8,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:},{:})", &self.x, &self.y)
    }
}

type RiskLevel = u8;

const MAX_STEPS: usize = 1000;

#[must_use]
pub fn part1(input: &str) -> usize {
    let map: HashMap<Point, RiskLevel> = parse(input);
    let destination: &Point = map.keys().max_by_key(|point| point.x + point.y).unwrap();

    0
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.chars().count()
}

fn neighborhood(point: &Point) -> Vec<Point> {
    vec![
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: point.x,
            y: point.y + 1,
        },
    ]
}

fn parse(input: &str) -> HashMap<Point, RiskLevel> {
    let mut map: HashMap<Point, RiskLevel> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            map.insert(
                Point {
                    x: x as i8,
                    y: y as i8,
                },
                chr as u8 - 48,
            );
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 40)
    }

    #[test]
    fn part2_example() {
        //        assert_eq!(part2(input()), 16)
    }
}
