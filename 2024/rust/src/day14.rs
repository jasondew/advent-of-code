use std::collections::HashMap;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}
impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Velocity {
    x: isize,
    y: isize,
}
impl Debug for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn step(&mut self, width: usize, height: usize) {
        self.position.x =
            Self::modular_add(self.position.x, self.velocity.x, width);
        self.position.y =
            Self::modular_add(self.position.y, self.velocity.y, height);
    }

    fn modular_add(a: usize, b: isize, m: usize) -> usize {
        let sum = a as isize + b;
        let isize_m = m as isize;

        (((sum % isize_m) + isize_m) % isize_m) as usize
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    part1_with_params(input, 101, 103, 100)
}

pub fn part1_with_params(
    input: &str,
    width: usize,
    height: usize,
    steps: usize,
) -> usize {
    let mut robots = parse(input);

    //    print_map(&robots, width, height, 0);

    for _step in 1..=steps {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }
        if possible_tree(&robots, width, height) {
            //            print_map(&robots, width, height, step);
        }
    }

    let left_half = 0..width / 2;
    let right_half = (width / 2 + 1)..width;
    let top_half = 0..height / 2;
    let bottom_half = (height / 2 + 1)..height;

    let quadrants = [
        (&left_half, &top_half),
        (&left_half, &bottom_half),
        (&right_half, &top_half),
        (&right_half, &bottom_half),
    ];
    let mut totals = [0, 0, 0, 0];

    for robot in robots {
        if let Some(index) = quadrants.iter().position(|(x_range, y_range)| {
            x_range.contains(&robot.position.x)
                && y_range.contains(&robot.position.y)
        }) {
            totals[index] += 1;
        }
    }

    totals.iter().fold(1, |acc, total| acc * total)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    part2_with_params(input, 101, 103)
}

pub fn part2_with_params(input: &str, width: usize, height: usize) -> usize {
    let mut robots = parse(input);
    let mut answer: usize = 0;

    //    print_map(&robots, width, height, 0);

    for step in 1..=100000 {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }

        if possible_tree(&robots, width, height) {
            answer = step;
            //            print_map(&robots, width, height, step);
            break;
        }
    }

    answer
}

fn possible_tree(robots: &Vec<Robot>, width: usize, height: usize) -> bool {
    let mut locations: HashMap<&Position, usize> = HashMap::new();

    let mut top_left_corner_count: usize = 0;
    let mut top_right_corner_count: usize = 0;

    for robot in robots {
        *locations.entry(&robot.position).or_insert(0) += 1;
        if (robot.position.x + robot.position.y) < width / 2 {
            top_left_corner_count += 1
        }
        if ((101 - robot.position.x) + robot.position.y) < width / 2 {
            top_right_corner_count += 1
        }
    }

    if top_left_corner_count < 25 && top_right_corner_count < 25 {
        let mut longest_contiguous: usize = 0;
        let mut run: usize = 0;

        for x in 0..width {
            for y in 0..height {
                if locations.contains_key(&Position { x, y }) {
                    run += 1;
                } else {
                    longest_contiguous = longest_contiguous.max(run);
                    run = 0;
                }
            }
            longest_contiguous = longest_contiguous.max(run);
            run = 0;
        }

        longest_contiguous > 10
    } else {
        false
    }
}

#[allow(dead_code)]
fn print_map(robots: &Vec<Robot>, width: usize, height: usize, steps: usize) {
    print!("\x1B[2J\x1B[1;1H");
    println!("STEP {}", steps);
    for y in 0..height {
        for x in 0..width {
            let count = robots
                .iter()
                .filter(|robot| robot.position == Position { x: x, y: y })
                .count();
            if count > 0 {
                print!("{}", count)
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!("\n");
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (position_string, velocity_string) =
                line.split_once(" ").unwrap();

            Robot {
                position: parse_position(position_string),
                velocity: parse_velocity(velocity_string),
            }
        })
        .collect()
}

fn parse_position(input: &str) -> Position {
    let (_, pair) = input.split_once("=").unwrap();
    let (x_string, y_string) = pair.split_once(",").unwrap();

    Position {
        x: parse_usize(x_string),
        y: parse_usize(y_string),
    }
}

fn parse_velocity(input: &str) -> Velocity {
    let (_, pair) = input.split_once("=").unwrap();
    let (x_string, y_string) = pair.split_once(",").unwrap();

    Velocity {
        x: parse_isize(x_string),
        y: parse_isize(y_string),
    }
}

fn parse_usize(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(value) => value,
        Err(_error) => panic!("invalid digit: {}", input),
    }
}

fn parse_isize(input: &str) -> isize {
    match input.parse::<isize>() {
        Ok(value) => value,
        Err(_error) => panic!("invalid digit: {}", input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    }

    #[test]
    fn robot_step_tests() {
        let mut robot = Robot {
            position: Position { x: 2, y: 4 },
            velocity: Velocity { x: 2, y: -3 },
        };
        for _ in 0..5 {
            robot.step(11, 7)
        }
        assert_eq!(robot.position, Position { x: 1, y: 3 });
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1_with_params(input(), 11, 7, 100), 12);
    }
}
