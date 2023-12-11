use std::collections::HashMap;

type Galaxy = usize;
type Location = (usize, usize);

#[derive(Debug)]
struct Image {
    max_x: usize,
    max_y: usize,
    galaxy_count: usize,
    map: HashMap<Location, Galaxy>,
    reverse_map: HashMap<Galaxy, Location>,
}

impl Image {
    fn expand(&self, factor: usize) -> Self {
        let mut expanded_map = HashMap::new();
        let mut expanded_reverse_map = HashMap::new();

        let row_expansions: Vec<usize> = (0..(self.max_y))
            .filter(|y| !self.map.iter().any(|(location, _)| location.1 == *y))
            .collect();

        let column_expansions: Vec<usize> = (0..(self.max_x))
            .filter(|x| !self.map.iter().any(|(location, _)| location.0 == *x))
            .collect();

        for (location, galaxy) in &self.map {
            let new_location = (
                location.0
                    + column_expansions
                        .iter()
                        .filter(|&c| c < &location.0)
                        .count()
                        * factor,
                location.1
                    + row_expansions
                        .iter()
                        .filter(|&r| r < &location.1)
                        .count()
                        * factor,
            );

            expanded_map.insert(new_location, *galaxy);
            expanded_reverse_map.insert(*galaxy, new_location);
        }

        Image {
            max_x: self.max_x + column_expansions.len(),
            max_y: self.max_y + row_expansions.len(),
            galaxy_count: self.galaxy_count,
            map: expanded_map,
            reverse_map: expanded_reverse_map,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if let Some(galaxy) = self.map.get(&(x, y)) {
                    print!("{:?}", galaxy)
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    fn distance(&self, from: Galaxy, to: Galaxy) -> usize {
        let (from_x, from_y) = self.galaxy_location(from);
        let (to_x, to_y) = self.galaxy_location(to);

        ((from_x as isize - to_x as isize).abs()
            + (from_y as isize - to_y as isize).abs()) as usize
    }

    fn galaxy_location(&self, galaxy: Galaxy) -> Location {
        *self.reverse_map.get(&galaxy).unwrap()
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let image = parse(input).expand(1);
    let mut total = 0;

    for from in 1..=image.galaxy_count {
        for to in (from + 1)..=image.galaxy_count {
            total += image.distance(from, to);
        }
    }

    total
}

#[must_use]
pub fn part2(input: &str) -> usize {
    part2_with_expansion_factor(input, 1000000 - 1)
}

fn part2_with_expansion_factor(input: &str, expansion_factor: usize) -> usize {
    let image = parse(input).expand(expansion_factor);
    let mut total = 0;

    for from in 1..=image.galaxy_count {
        for to in (from + 1)..=image.galaxy_count {
            total += image.distance(from, to);
        }
    }

    total
}

fn parse(input: &str) -> Image {
    let max_x = input.lines().next().unwrap().chars().count();
    let max_y = input.lines().count();

    let mut next_galaxy_number = 1;
    let mut map: HashMap<Location, Galaxy> = HashMap::new();
    let mut reverse_map: HashMap<Galaxy, Location> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((x, y), next_galaxy_number);
                reverse_map.insert(next_galaxy_number, (x, y));
                next_galaxy_number += 1;
            }
        }
    }

    Image {
        max_x,
        max_y,
        map,
        reverse_map,
        galaxy_count: next_galaxy_number - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 374)
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2_with_expansion_factor(input(), 9), 1030)
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2_with_expansion_factor(input(), 99), 8410)
    }
}
