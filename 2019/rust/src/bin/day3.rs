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

#[derive(Debug)]
enum LineSegment {
    Horizontal { x: (i32, i32), y: i32 },
    Vertical { x: i32, y: (i32, i32) },
}

fn to_line_segments(path: &Path) -> Vec<LineSegment> {
    let mut current: Point = Point::origin();
    let mut line_segments: Vec<LineSegment> = Vec::new();

    for vector in path {
        current = match vector.direction {
            Direction::Up => {
                let new_y = current.y + vector.magnitude as i32;

                line_segments.push(LineSegment::Vertical {
                    x: current.x,
                    y: (current.y, new_y),
                });
                Point {
                    x: current.x,
                    y: new_y,
                }
            }
            Direction::Down => {
                let new_y = current.y - vector.magnitude as i32;

                line_segments.push(LineSegment::Vertical {
                    x: current.x,
                    y: (new_y, current.y),
                });
                Point {
                    x: current.x,
                    y: new_y,
                }
            }
            Direction::Left => {
                let new_x = current.x - vector.magnitude as i32;
                line_segments.push(LineSegment::Horizontal {
                    x: (new_x, current.x),
                    y: current.y,
                });
                Point {
                    x: new_x,
                    y: current.y,
                }
            }
            Direction::Right => {
                let new_x = current.x + vector.magnitude as i32;
                line_segments.push(LineSegment::Horizontal {
                    x: (current.x, new_x),
                    y: current.y,
                });
                Point {
                    x: new_x,
                    y: current.y,
                }
            }
        };
    }

    line_segments
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

fn intersecting_point(line_segment_a: &LineSegment, line_segment_b: &LineSegment) -> Option<Point> {
    match &line_segment_a {
        LineSegment::Horizontal {
            x: (ax_from, _ax_to),
            y: ay,
        } => match &line_segment_b {
            LineSegment::Horizontal {
                x: (bx_from, _bx_to),
                y: by,
            } => {
                if ay == by {
                    if ax_from <= bx_from {
                        horizontal_horizontal_intersecting_point(&line_segment_a, &line_segment_b)
                    } else {
                        horizontal_horizontal_intersecting_point(&line_segment_b, &line_segment_a)
                    }
                } else {
                    None
                }
            }
            LineSegment::Vertical { .. } => {
                horizontal_vertical_intersecting_point(&line_segment_a, &line_segment_b)
            }
        },
        LineSegment::Vertical {
            x: ax,
            y: (ay_from, _ay_to),
        } => match &line_segment_b {
            LineSegment::Horizontal { .. } => {
                horizontal_vertical_intersecting_point(&line_segment_b, &line_segment_a)
            }
            LineSegment::Vertical {
                x: bx,
                y: (by_from, _by_to),
            } => {
                if ax == bx {
                    if ay_from <= by_from {
                        vertical_vertical_intersecting_point(&line_segment_a, &line_segment_b)
                    } else {
                        vertical_vertical_intersecting_point(&line_segment_b, &line_segment_a)
                    }
                } else {
                    None
                }
            }
        },
    }
}

fn horizontal_horizontal_intersecting_point(
    line_segment_a: &LineSegment,
    line_segment_b: &LineSegment,
) -> Option<Point> {
    if let LineSegment::Horizontal {
        x: (ax_from, ax_to),
        y: ay,
    } = line_segment_a
    {
        if let LineSegment::Horizontal {
            x: (bx_from, bx_to),
            y: _by,
        } = line_segment_b
        {
            if ax_to >= bx_from && ax_from <= bx_to {
                Some(Point {
                    x: *bx_from,
                    y: *ay,
                })
            } else {
                None
            }
        } else {
            panic!("wtf")
        }
    } else {
        panic!("wtf")
    }
}

fn vertical_vertical_intersecting_point(
    line_segment_a: &LineSegment,
    line_segment_b: &LineSegment,
) -> Option<Point> {
    if let LineSegment::Vertical {
        x: ax,
        y: (ay_from, ay_to),
    } = line_segment_a
    {
        if let LineSegment::Vertical {
            x: _bx,
            y: (by_from, by_to),
        } = line_segment_b
        {
            if ay_to >= by_from && ay_from <= by_to {
                Some(Point {
                    x: *ax,
                    y: *by_from,
                })
            } else {
                None
            }
        } else {
            panic!("wtf")
        }
    } else {
        panic!("wtf")
    }
}
fn horizontal_vertical_intersecting_point(
    line_segment_a: &LineSegment,
    line_segment_b: &LineSegment,
) -> Option<Point> {
    if let LineSegment::Horizontal {
        x: (ax_from, ax_to),
        y: ay,
    } = line_segment_a
    {
        if let LineSegment::Vertical {
            x: bx,
            y: (by_from, by_to),
        } = line_segment_b
        {
            if bx >= ax_from && bx <= ax_to && ay >= by_from && ay <= by_to {
                Some(Point { x: *bx, y: *ay })
            } else {
                None
            }
        } else {
            panic!("wtf")
        }
    } else {
        panic!("wtf")
    }
}

fn manhattan_distance(&point: &Point) -> i32 {
    point.x.abs() + point.y.abs()
}

fn signal_delay(&point: &Point, line_segments: &Vec<LineSegment>) -> i32 {
    let (total, _found) = line_segments
        .iter()
        .fold((0, false), |(total, found), line_segment| {
            if found {
                (total, found)
            } else {
                match *line_segment {
                    LineSegment::Horizontal {
                        x: (from_x, to_x),
                        y,
                    } => {
                        if point.y == y && point.x >= from_x && point.x <= to_x {
                            (total + (point.x - from_x).abs(), true)
                        } else {
                            (total + (to_x - from_x).abs(), false)
                        }
                    }
                    LineSegment::Vertical {
                        x,
                        y: (from_y, to_y),
                    } => {
                        if point.x == x && point.y >= from_y && point.y <= to_y {
                            (total + (point.y - from_y).abs(), true)
                        } else {
                            (total + (to_y - from_y).abs(), false)
                        }
                    }
                }
            }
        });
    total
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
        let path_one_line_segments = to_line_segments(path_one);
        let path_two_line_segments = to_line_segments(path_two);
        let mut intersecting_points = Vec::new();

        for line_segment_a in &path_one_line_segments {
            for line_segment_b in &path_two_line_segments {
                if let Some(point) = intersecting_point(&line_segment_a, &line_segment_b) {
                    if point.x > 0 && point.y > 0 {
                        intersecting_points.push(point)
                    }
                }
            }
        }

        if let Some(minimum_point) = intersecting_points
            .iter()
            .min_by_key(|point| manhattan_distance(*point))
        {
            println!("part 1: {:?}", manhattan_distance(minimum_point));
        }

        if let Some(minimum_point) = intersecting_points.iter().min_by_key(|point| {
            signal_delay(*point, &path_one_line_segments)
                + signal_delay(*point, &path_two_line_segments)
        }) {
            println!(
                "part 1: {:?}",
                signal_delay(minimum_point, &path_one_line_segments)
                    + signal_delay(minimum_point, &path_two_line_segments)
            );
        }

        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "bad input"))
    }
}
