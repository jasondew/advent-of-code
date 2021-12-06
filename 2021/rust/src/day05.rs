use core::fmt::Debug;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

enum Orientation {
    Horizontal,
    Vertical,
    Slope(isize),
}

impl Debug for Orientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
            Self::Slope(m) => write!(f, "m={:}", m),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:},{:})", &self.x, &self.y)
    }
}

#[derive(Debug)]
struct LineSegment {
    from: Point,
    to: Point,
    orientation: Orientation,
}

impl LineSegment {
    fn to_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        let mut x = self.from.x;
        let mut y = self.from.y;

        let (dx, dy): (isize, isize) = match self.orientation {
            Orientation::Horizontal => (1, 0),
            Orientation::Vertical => (0, 1),
            Orientation::Slope(1) => (1, 1),
            Orientation::Slope(-1) => (1, -1),
            _ => panic!("unsupported slope"),
        };

        if dx > 0 {
            while x <= self.to.x {
                points.push(Point { x, y });
                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;
            }
        } else {
            let a: usize;
            let b: usize;

            if self.from.y <= self.to.y {
                a = self.from.y;
                b = self.to.y;
            } else {
                b = self.from.y;
                a = self.to.y;
            }

            for y in a..=b {
                points.push(Point { x, y });
                x = (x as isize + dx) as usize;
            }
        }

        points
    }

    fn overlaps(&self, other: &Self) -> HashSet<Point> {
        use Orientation::*;

        match (&self.orientation, &other.orientation) {
            (Horizontal, Horizontal) => {
                if self.from.y == other.from.y {
                    Self::linear_overlap(self.from.x, self.to.x, other.from.x, other.to.x)
                        .map(|x| Point {
                            x: x,
                            y: self.from.y,
                        })
                        .collect()
                } else {
                    HashSet::new()
                }
            }
            (Vertical, Vertical) => {
                if self.from.x == other.from.x {
                    Self::linear_overlap(self.from.y, self.to.y, other.from.y, other.to.y)
                        .map(|y| Point {
                            x: self.from.x,
                            y: y,
                        })
                        .collect()
                } else {
                    HashSet::new()
                }
            }
            (Horizontal, Vertical) => {
                if Self::in_range(self.from.y, other.from.y, other.to.y)
                    && Self::in_range(other.from.x, self.from.x, self.to.x)
                {
                    self.singleton(&other, other.from.x, self.from.y)
                } else {
                    HashSet::new()
                }
            }
            (Vertical, Horizontal) => {
                if Self::in_range(self.from.x, other.from.x, other.to.x)
                    && Self::in_range(other.from.y, self.from.y, self.to.y)
                {
                    self.singleton(&other, self.from.x, other.from.y)
                } else {
                    HashSet::new()
                }
            }
            (Slope(1), Slope(1)) => {
                if Self::in_range(self.from.x, other.from.x, other.to.x) {
                    self.diagonal_points(
                        &other,
                        self.from.x,
                        self.from.y,
                        self.to.x.min(other.to.x),
                        1,
                    )
                } else if Self::in_range(other.from.x, self.from.x, self.to.x) {
                    self.diagonal_points(
                        &other,
                        other.from.x,
                        other.from.y,
                        other.to.x.min(self.to.x),
                        1,
                    )
                } else {
                    HashSet::new()
                }
            }
            (Slope(-1), Slope(-1)) => {
                if Self::in_range(self.from.x, other.from.x, other.to.x) {
                    self.diagonal_points(
                        &other,
                        self.from.x,
                        self.from.y,
                        self.to.x.min(other.to.x),
                        -1,
                    )
                } else if Self::in_range(other.from.x, self.from.x, self.to.x) {
                    self.diagonal_points(
                        &other,
                        other.from.x,
                        other.from.y,
                        other.to.x.min(self.to.x),
                        -1,
                    )
                } else {
                    HashSet::new()
                }
            }
            (Slope(1), Slope(-1)) => self.intersecting_point(1, &other, -1),
            (Slope(-1), Slope(1)) => self.intersecting_point(-1, &other, 1),
            (Horizontal, Slope(m)) => self.intersecting_point(0, other, *m),
            (Slope(m), Horizontal) => self.intersecting_point(*m, other, 0),
            (Vertical, Slope(m)) => {
                let y = (m * (self.from.x as isize - other.from.x as isize) + other.from.y as isize)
                    as usize;
                self.singleton(&other, self.from.x, y)
            }
            (Slope(m), Vertical) => {
                let y = (m * (other.from.x as isize - self.from.x as isize) + self.from.y as isize)
                    as usize;
                other.singleton(&self, other.from.x, y)
            }
            (Slope(_), Slope(_)) => panic!("invalid slopes: {:?} and {:?}", &self, &other),
        }
    }

    fn intersecting_point(&self, m1: isize, other: &LineSegment, m2: isize) -> HashSet<Point> {
        let x1 = self.from.x as isize;
        let x2 = other.from.x as isize;
        let y1 = self.from.y as isize;
        let y2 = other.from.y as isize;

        let x = (m1 * x1 - m2 * x2 - y1 + y2).checked_div(m1 - m2).unwrap();
        let y = m1 * (x - x1) + y1;

        let point = Point {
            x: x as usize,
            y: y as usize,
        };

        self.singleton(other, point.x, point.y)
    }

    fn coincident(&self, point: &Point) -> bool {
        Self::in_range(point.x, self.from.x, self.to.x)
            && Self::in_range(point.y, self.from.y, self.to.y)
    }

    fn diagonal_points(
        &self,
        other: &LineSegment,
        x1: usize,
        y1: usize,
        x2: usize,
        m: isize,
    ) -> HashSet<Point> {
        let mut set = HashSet::new();
        let mut y = y1;
        for x in x1..(x2 + 1) {
            let point = Point { x, y };
            if self.coincident(&point) && other.coincident(&point) {
                set.insert(point);
            }
            y = (y as isize + m) as usize;
        }
        set
    }

    fn singleton(&self, other: &LineSegment, x: usize, y: usize) -> HashSet<Point> {
        let point: Point = Point { x, y };
        if self.coincident(&point) && other.coincident(&point) {
            let mut set = HashSet::new();
            set.insert(point);
            set
        } else {
            HashSet::new()
        }
    }

    fn in_range(value: usize, a: usize, b: usize) -> bool {
        if a < b {
            a <= value && value <= b
        } else {
            b <= value && value <= a
        }
    }

    fn linear_overlap(mut a1: usize, mut a2: usize, mut b1: usize, mut b2: usize) -> Range<usize> {
        if a1 > a2 {
            let tmp = a1;
            a1 = a2;
            a2 = tmp;
        }

        if b1 > b2 {
            let tmp = b1;
            b1 = b2;
            b2 = tmp;
        }

        if a1 <= b1 && b1 <= a2 {
            if b2 > a2 {
                b1..(a2 + 1)
            } else {
                b1..(b2 + 1)
            }
        } else if b1 <= a1 && a1 <= b2 {
            if a2 > b2 {
                a1..(b2 + 1)
            } else {
                a1..(a2 + 1)
            }
        } else {
            0..0
        }
    }

    fn orientation(from: &Point, to: &Point) -> Orientation {
        let numerator = from.y as isize - to.y as isize;
        let denominator = from.x as isize - to.x as isize;

        if denominator == 0 {
            Orientation::Vertical
        } else if numerator == 0 {
            Orientation::Horizontal
        } else {
            Orientation::Slope(numerator / denominator)
        }
    }
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = string.split_once(",") {
            Ok(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
        } else {
            Err("invalid point")
        }
    }
}

impl FromStr for LineSegment {
    type Err = &'static str;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = string.split_once(" -> ") {
            let from: Point = from.parse().unwrap();
            let to: Point = to.parse().unwrap();

            if from.x <= to.x {
                let orientation = Self::orientation(&from, &to);
                Ok(LineSegment {
                    from,
                    to,
                    orientation,
                })
            } else {
                let orientation = Self::orientation(&to, &from);
                Ok(LineSegment {
                    from: to,
                    to: from,
                    orientation,
                })
            }
        } else {
            Err("invalid point")
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let line_segments: Vec<LineSegment> = parse(input)
        .into_iter()
        .filter(|line_segment| match line_segment.orientation {
            Orientation::Horizontal | Orientation::Vertical => true,
            _ => false,
        })
        .collect();
    overlap_count(line_segments)
}

fn overlap_count_old_and_busted(line_segments: Vec<LineSegment>) -> usize {
    let mut overlaps: HashSet<Point> = HashSet::new();

    for (index, line_segment) in line_segments.iter().enumerate() {
        for other_line_segment in line_segments.iter().skip(index + 1) {
            for point in line_segment.overlaps(other_line_segment) {
                overlaps.insert(point);
            }
        }
    }

    overlaps.len()
}

fn overlap_count(line_segments: Vec<LineSegment>) -> usize {
    let mut point_counts: HashMap<Point, usize> = HashMap::new();

    for line_segment in line_segments {
        for point in line_segment.to_points().into_iter() {
            if let Some(count) = point_counts.get(&point) {
                point_counts.insert(point, count + 1);
            } else {
                point_counts.insert(point, 1);
            }
        }
    }

    point_counts
        .iter()
        .filter(|(_point, &count)| count >= 2)
        .count()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    overlap_count(parse(input))
}

fn parse(input: &str) -> Vec<LineSegment> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 5)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 12)
    }
}
