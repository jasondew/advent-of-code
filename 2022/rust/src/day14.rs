use std::{collections::HashSet, fmt::Debug, ops::RangeInclusive};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(input: &str) -> Self {
        let (x_string, y_string) = input.split_once(',').unwrap();
        Point {
            x: x_string.parse().unwrap(),
            y: y_string.parse().unwrap(),
        }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Line {
    Vertical(usize, RangeInclusive<usize>),
    Horizontal(RangeInclusive<usize>, usize),
}

impl Line {
    fn from_points(points: &[Point]) -> Vec<Self> {
        points
            .windows(2)
            .map(|window| {
                let left = &window[0];
                let right = &window[1];

                if left.x == right.x {
                    let range = if left.y < right.y {
                        left.y..=right.y
                    } else {
                        right.y..=left.y
                    };
                    Self::Vertical(left.x, range)
                } else {
                    let range = if left.x < right.x {
                        left.x..=right.x
                    } else {
                        right.x..=left.x
                    };
                    Self::Horizontal(range, left.y)
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Map {
    lines: Vec<Line>,
    bottom: usize,
}

impl Map {
    fn colliding(&self, sand: &HashSet<Point>, point: Point) -> bool {
        self.lines.iter().any(|line| match line {
            Line::Horizontal(x_range, y) => {
                point.y == *y && x_range.contains(&point.x)
            }
            Line::Vertical(x, y_range) => {
                point.x == *x && y_range.contains(&point.y)
            }
        }) || sand.contains(&point)
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let source = Point { x: 500, y: 0 };
    let mut sand: HashSet<Point> = HashSet::new();
    let mut previous_point = source;

    while let Some((rest_point, new_previous_point)) =
        settle(&map, &sand, previous_point, source)
    {
        previous_point = new_previous_point;
        sand.insert(rest_point);
    }

    sand.len()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut map = parse(input);
    map.lines.push(Line::Horizontal(0..=10000, map.bottom + 2));
    map.bottom += 3;

    let source = Point { x: 500, y: 0 };
    let mut sand: HashSet<Point> = HashSet::new();
    let mut previous_point = source;

    while let Some((rest_point, new_previous_point)) =
        settle(&map, &sand, previous_point, source)
    {
        previous_point = new_previous_point;
        sand.insert(rest_point);
        if rest_point == source {
            break;
        }
    }

    sand.len()
}

fn settle(
    map: &Map,
    sand: &HashSet<Point>,
    point: Point,
    previous_point: Point,
) -> Option<(Point, Point)> {
    if point.y > map.bottom {
        return None;
    }

    for candidate in [point.down(), point.down_left(), point.down_right()] {
        if !map.colliding(sand, candidate) {
            return settle(map, sand, candidate, point);
        }
    }

    Some((point, previous_point))
}

fn parse(input: &str) -> Map {
    let lines: Vec<Line> =
        input
            .lines()
            .flat_map(|line| {
                Line::from_points(
                    &line
                        .split(" -> ")
                        .map(Point::parse)
                        .collect::<Vec<Point>>()[..],
                )
            })
            .collect();
    let bottom = *lines
        .iter()
        .map(|line| match line {
            Line::Vertical(_x, y_range) => y_range.end(),
            Line::Horizontal(_x_range, y) => y,
        })
        .max()
        .unwrap();

    Map { lines, bottom }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 24)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 93)
    }
}
