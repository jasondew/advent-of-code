use std::cmp::PartialEq;
use std::fmt;
use std::fs;
use std::io;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Vector {
    direction: Direction,
    magnitude: u16,
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.direction, self.magnitude)
    }
}

type Path = Vec<Vector>;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn to_points(path: &Path) -> Vec<Point> {
    let mut current: Point = Point::origin();
    let mut points: Vec<Point> = Vec::new();

    for vector in path {
        let mut new_points: Vec<Point> = match vector.direction {
            Direction::Up => ((current.y + 1)..=(current.y + vector.magnitude as i32))
                .map(|y| Point { x: current.x, y: y })
                .collect(),
            Direction::Down => ((current.y - vector.magnitude as i32)..current.y)
                .rev()
                .map(|y| Point { x: current.x, y: y })
                .collect(),
            Direction::Left => ((current.x - vector.magnitude as i32)..current.x)
                .rev()
                .map(|x| Point { x: x, y: current.y })
                .collect(),
            Direction::Right => ((current.x + 1)..=(current.x + vector.magnitude as i32))
                .map(|x| Point { x: x, y: current.y })
                .collect(),
        };

        if let Some(new_current) = new_points.last() {
            current = *new_current
        };
        points.append(&mut new_points)
    }

    points
}

fn to_direction(string: &str) -> Direction {
    match string {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("bad direction seen: {}", string),
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("../inputs/03")?;
    if let Some([path_one, path_two]) = input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|string| {
                    let (direction, magnitude_string) = string.split_at(1);
                    Vector {
                        direction: to_direction(direction),
                        magnitude: magnitude_string.parse().unwrap(),
                    }
                })
                .collect::<Path>()
        })
        .collect::<Vec<Path>>()
        .get(0..2)
    {
        let path_one_points = to_points(path_one);
        let path_two_points = to_points(path_two);

        if let Some(minimum_point) = path_two_points
            .iter()
            .filter(|point| path_one_points.contains(point))
            .min_by_key(|point| (point.x + point.y).abs())
        {
            let part1 = (minimum_point.x + minimum_point.y).abs();
            println!("part 1: {:?}", part1);
        }

        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "bad input"))
    }
}
