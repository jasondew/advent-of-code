use regex::Regex;
use std::{collections::HashSet, ops::RangeInclusive};

struct Point {
    x: isize,
    y: isize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[allow(clippy::cast_possible_wrap)]
impl Point {
    fn distance(&self, other: &Point) -> isize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as isize
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    size: isize,
}

#[allow(clippy::cast_possible_wrap)]
impl Sensor {
    fn row_span(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let radius = self.size - self.location.y.abs_diff(y) as isize;

        if radius > 0 {
            Some((self.location.x - radius)..=(self.location.x + radius))
        } else {
            None
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    part1_(input, 2_000_000)
}

fn part1_(input: &str, target_row: isize) -> usize {
    let sensors = parse(input);
    let mut xs: HashSet<isize> = HashSet::new();

    for sensor in sensors {
        if let Some(range) = sensor.row_span(target_row) {
            for x in range {
                xs.insert(x);
            }
        }
    }

    xs.len() - 1
}

#[must_use]
pub fn part2(input: &str) -> usize {
    part2_(input, 4_000_000)
}

#[allow(clippy::cast_sign_loss)]
fn part2_(input: &str, max_coordinate: isize) -> usize {
    let sensors = parse(input);

    for y in 0..=max_coordinate {
        let mut spans: Vec<RangeInclusive<isize>> = sensors
            .iter()
            .filter_map(|sensor| sensor.row_span(y))
            .collect();
        spans.sort_unstable_by_key(|range| (*range.start(), *range.end()));

        let mut x = spans[0].end() + 1;

        for range in spans.iter().skip(1) {
            if x < *range.start() {
                return (x * 4_000_000 + y) as usize;
            }

            x = x.max(range.end() + 1);
        }
    }

    0
}

fn parse(input: &str) -> Vec<Sensor> {
    let regex = Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();

    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line.trim()).unwrap();
            let location = Point {
                x: captures[1].parse::<isize>().unwrap(),
                y: captures[2].parse::<isize>().unwrap(),
            };
            let nearest_beacon = Point {
                x: captures[3].parse::<isize>().unwrap(),
                y: captures[4].parse::<isize>().unwrap(),
            };
            let size = location.distance(&nearest_beacon);

            Sensor { location, size }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1_(input(), 10), 26)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_(input(), 20), 56000011)
    }
}
