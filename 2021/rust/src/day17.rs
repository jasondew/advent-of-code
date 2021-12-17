use std::fmt::Debug;
use std::ops::RangeInclusive;

struct Probe {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Debug for Probe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x={:} y={:} dx={:} dy={:}",
            &self.x, &self.y, &self.dx, &self.dy
        )
    }
}

impl Probe {
    fn new(dx: isize, dy: isize) -> Self {
        Self { x: 0, y: 0, dx, dy }
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dy -= 1;
        if self.dx > 0 {
            self.dx -= 1;
        } else if self.dx < 0 {
            self.dx += 1;
        }
    }
}

enum SimulationResult {
    Hit(usize),
    Miss,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (x_range_string, y_range_string) = input
        .trim_start_matches("target area: ")
        .trim_end()
        .split_once(", ")
        .unwrap();
    let x_range = parse_range(x_range_string);
    let y_range = parse_range(y_range_string);
    let mut max_y: usize = 0;

    for dx in 0..100 {
        for dy in 0..150 {
            match simulate(Probe::new(dx, dy), &x_range, &y_range) {
                SimulationResult::Hit(height) => max_y = height.max(max_y),
                SimulationResult::Miss => {}
            }
        }
    }

    max_y
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (x_range_string, y_range_string) = input
        .trim_start_matches("target area: ")
        .trim_end()
        .split_once(", ")
        .unwrap();
    let x_range = parse_range(x_range_string);
    let y_range = parse_range(y_range_string);
    let mut hit_count: usize = 0;

    for dx in 0..250 {
        for dy in -250..250 {
            match simulate(Probe::new(dx, dy), &x_range, &y_range) {
                SimulationResult::Hit(_height) => {
                    hit_count += 1;
                }
                SimulationResult::Miss => {}
            }
        }
    }

    hit_count
}

fn simulate(
    mut probe: Probe,
    x_range: &RangeInclusive<isize>,
    y_range: &RangeInclusive<isize>,
) -> SimulationResult {
    let mut max_y: isize = 0;

    loop {
        probe.step();
        max_y = probe.y.max(max_y);
        if x_range.contains(&probe.x) && y_range.contains(&probe.y) {
            return SimulationResult::Hit(max_y as usize);
        }
        if probe.x > *x_range.end() || probe.y < *y_range.start() {
            return SimulationResult::Miss;
        }
    }
}

fn parse_range(input: &str) -> RangeInclusive<isize> {
    let chars_to_remove: &[_] = &['x', 'y', '='];
    let (start_string, end_string) = input
        .trim_start_matches(chars_to_remove)
        .split_once("..")
        .unwrap();
    start_string.parse().unwrap()..=end_string.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "target area: x=20..30, y=-10..-5\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 45)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 112)
    }
}
