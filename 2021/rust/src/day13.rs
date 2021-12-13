use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:},{:})", &self.x, &self.y)
    }
}

#[derive(Debug)]
enum Fold {
    Horizontally(isize),
    Vertically(isize),
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (points, folds) = parse(input);
    let mut set: HashSet<Point> = HashSet::new();

    let fold = folds.iter().next().unwrap();
    for mut point in points {
        reflect(&mut point, fold);
        set.insert(point);
    }

    set.len()
}

fn reflect(point: &mut Point, fold: &Fold) {
    match fold {
        Fold::Horizontally(at) => {
            reflect_horizontally(point, *at);
        }
        Fold::Vertically(at) => {
            reflect_vertically(point, *at);
        }
    }
}

fn reflect_horizontally(point: &mut Point, at: isize) {
    if point.y > at {
        point.y = point.y - 2 * (point.y - at)
    }
}

fn reflect_vertically(point: &mut Point, at: isize) {
    if point.x > at {
        point.x = point.x - 2 * (point.x - at)
    }
}

#[must_use]
pub fn part2(input: &str) -> String {
    let (points, folds) = parse(input);
    let mut set: HashSet<Point> = HashSet::new();

    for mut point in points {
        for fold in &folds {
            reflect(&mut point, fold);
        }
        set.insert(point);
    }

    format(&set)
}

fn format(points: &HashSet<Point>) -> String {
    let max_x = points.iter().map(|point| point.x).max().unwrap();
    let max_y = points.iter().map(|point| point.y).max().unwrap();
    let mut row: Vec<String> = Vec::new();
    let mut lines: Vec<String> = Vec::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(_) = points.get(&Point { x: x, y: y }) {
                row.push("##".into());
            } else {
                row.push("..".into());
            }
        }
        lines.push(row.join(""));
        row.clear();
    }

    lines.join("\n")
}

fn parse(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let mut points: Vec<Point> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();

    for line in input.trim_end().lines() {
        if let Some(_) = line.find(',') {
            let (x_string, y_string) = line.split_once(',').unwrap();
            points.push(Point {
                x: x_string.parse().unwrap(),
                y: y_string.parse().unwrap(),
            });
        } else if line.starts_with("fold") {
            let (directive, value) = line.split_once('=').unwrap();
            match directive {
                "fold along x" => folds.push(Fold::Vertically(value.parse().unwrap())),
                "fold along y" => folds.push(Fold::Horizontally(value.parse().unwrap())),
                _ => panic!("unknown directive: {}", directive),
            }
        }
    }

    (points, folds)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 17)
    }
}
