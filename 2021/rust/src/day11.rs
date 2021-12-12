use core::fmt::Debug;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i8,
    y: i8,
}
impl Debug for Point {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:},{:})", &self.x, &self.y)
    }
}

#[derive(Debug, PartialEq)]
enum FlashStatus {
    Off,
    Triggered,
    On,
}

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flash_status: FlashStatus,
}

impl Octopus {
    fn increase(&mut self) {
        self.energy += 1;

        if self.energy >= 10 && self.flash_status == FlashStatus::Off {
            self.flash_status = FlashStatus::Triggered;
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    flash_count(input, 100)
}

pub fn flash_count(input: &str, steps: usize) -> usize {
    let mut octopi = parse(input);
    let mut flash_count: usize = 0;

    //    println!("Before any steps:");
    //    print(&octopi);

    for _step_count in 1..=steps {
        flash_count += step(&mut octopi);
        //        println!("After step {}:", step_count);
        //        print(&octopi);
    }

    flash_count
}

fn step(octopi: &mut HashMap<Point, Octopus>) -> usize {
    // increase all energy levels by 1
    for y in 0..10 {
        for x in 0..10 {
            octopi.entry(Point { x, y }).and_modify(|o| o.increase());
        }
    }

    // propagate flashes
    propagate_flashes(octopi);

    // set energy level to 0 if flashed
    let mut flash_count: usize = 0;

    for (_location, octopus) in octopi {
        if octopus.flash_status == FlashStatus::On {
            flash_count += 1;
            octopus.energy = 0;
            octopus.flash_status = FlashStatus::Off;
        }
    }

    flash_count
}

fn propagate_flashes(octopi: &mut HashMap<Point, Octopus>) {
    let mut flash_points: Vec<Point> = vec![];

    for y in 0..10 {
        for x in 0..10 {
            let location = Point { x, y };
            if octopi.get(&location).unwrap().flash_status == FlashStatus::Triggered {
                octopi
                    .entry(location.clone())
                    .and_modify(|o| o.flash_status = FlashStatus::On);
                flash_points.push(location);
            }
        }
    }

    for flash_point in &flash_points {
        for point in neighborhood(flash_point) {
            octopi.entry(point).and_modify(|o| {
                o.increase();
            });
        }
    }

    if flash_points.len() > 0 {
        propagate_flashes(octopi)
    }
}

fn neighborhood(location: &Point) -> Vec<Point> {
    vec![
        Point {
            x: location.x - 1,
            y: location.y - 1,
        },
        Point {
            x: location.x,
            y: location.y - 1,
        },
        Point {
            x: location.x + 1,
            y: location.y - 1,
        },
        Point {
            x: location.x - 1,
            y: location.y,
        },
        Point {
            x: location.x + 1,
            y: location.y,
        },
        Point {
            x: location.x - 1,
            y: location.y + 1,
        },
        Point {
            x: location.x,
            y: location.y + 1,
        },
        Point {
            x: location.x + 1,
            y: location.y + 1,
        },
    ]
}

#[allow(dead_code)]
fn print(octopi: &HashMap<Point, Octopus>) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{} ", octopi.get(&Point { x, y }).unwrap().energy);
        }
        println!();
    }
    println!();
}

fn parse(input: &str) -> HashMap<Point, Octopus> {
    let mut octopi: HashMap<Point, Octopus> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            octopi.insert(
                Point {
                    x: x as i8,
                    y: y as i8,
                },
                Octopus {
                    energy: (chr as u8 - 48),
                    flash_status: FlashStatus::Off,
                },
            );
        }
    }

    octopi
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut octopi = parse(input);

    for step_count in 1..=1000 {
        step(&mut octopi);

        if octopi
            .iter()
            .all(|(_location, octopus)| octopus.energy == 0)
        {
            return step_count;
        }
    }
    panic!("not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(flash_count(input(), 10), 204);
        assert_eq!(flash_count(input(), 100), 1656)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 195)
    }
}
