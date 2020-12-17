use std::collections::HashMap;
use std::str::FromStr;

type Location = (isize, isize, isize, isize);

#[derive(Debug)]
struct Grid {
    data: HashMap<Location, Cell>,
    active_count: usize,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
    w_min: isize,
    w_max: isize,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            active_count: 0,
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            z_min: 0,
            z_max: 0,
            w_min: 0,
            w_max: 0,
        }
    }

    fn insert(&mut self, location: Location, cell: Cell) {
        match &cell {
            Cell::Active => self.active_count += 1,
            _ => {}
        }

        if location.0 < self.x_min {
            self.x_min = location.0
        }
        if location.0 > self.x_max {
            self.x_max = location.0
        }

        if location.1 < self.y_min {
            self.y_min = location.1
        }
        if location.1 > self.y_max {
            self.y_max = location.1
        }

        if location.2 < self.z_min {
            self.z_min = location.2
        }
        if location.2 > self.z_max {
            self.z_max = location.2
        }

        if location.3 < self.w_min {
            self.w_min = location.3
        }
        if location.3 > self.w_max {
            self.w_max = location.3
        }

        self.data.insert(location, cell);
    }

    fn at(&self, location: &Location) -> &Cell {
        self.data.get(location).unwrap_or(&Cell::Inactive)
    }

    fn next(&mut self, four_dimensions: bool) {
        let mut grid = Self::new();
        let w_range = if four_dimensions {
            (self.w_min - 1)..(self.w_max + 2)
        } else {
            0..1
        };

        for w in w_range {
            for x in (self.x_min - 1)..(self.x_max + 2) {
                for y in (self.y_min - 1)..(self.y_max + 2) {
                    for z in (self.z_min - 1)..(self.z_max + 2) {
                        let location = (x, y, z, w);
                        let cell = self.at(&location);
                        let active_neighbor_count =
                            self.active_neighbor_count(&location, four_dimensions);
                        let next_generation = match cell {
                            Cell::Active => {
                                if active_neighbor_count == 2 || active_neighbor_count == 3 {
                                    Cell::Active
                                } else {
                                    Cell::Inactive
                                }
                            }
                            Cell::Inactive => {
                                if active_neighbor_count == 3 {
                                    Cell::Active
                                } else {
                                    Cell::Inactive
                                }
                            }
                        };

                        grid.insert(location, next_generation);
                    }
                }
            }
        }

        *self = grid;
    }

    fn active_neighbor_count(&self, (x, y, z, w): &Location, four_dimensions: bool) -> usize {
        let mut count: usize = 0;
        let delta_ws = if four_dimensions {
            vec![-1, 0, 1]
        } else {
            vec![0]
        };

        for delta_w in delta_ws {
            for delta_x in vec![-1, 0, 1] {
                for delta_y in vec![-1, 0, 1] {
                    for delta_z in vec![-1, 0, 1] {
                        if delta_x == 0 && delta_y == 0 && delta_z == 0 && delta_w == 0 {
                        } else {
                            match self.at(&(x + delta_x, y + delta_y, z + delta_z, w + delta_w)) {
                                Cell::Active => {
                                    count += 1;
                                    if count > 3 {
                                        break;
                                    }
                                }
                                Cell::Inactive => {}
                            }
                        }
                    }
                }
            }
        }

        count
    }

    #[allow(dead_code)]
    fn print(&self) {
        for w in self.w_min..(self.w_max + 1) {
            for z in self.z_min..(self.z_max + 1) {
                println!("z = {}, w = {}", z, w);
                for y in self.y_min..(self.y_max + 1) {
                    for x in self.x_min..(self.x_max + 1) {
                        match self.at(&(x, y, z, w)) {
                            Cell::Active => print!("#"),
                            Cell::Inactive => print!("."),
                        }
                    }
                    println!();
                }
                println!();
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Cell {
    Active,
    Inactive,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::new();
        let z: isize = 0;
        let w: isize = 0;
        for (y, line) in string.trim().lines().enumerate() {
            for (x, ch) in line.trim().chars().enumerate() {
                let cell = match ch {
                    '#' => Cell::Active,
                    '.' => Cell::Inactive,
                    _ => panic!("invalid cell found"),
                };
                grid.insert((x as isize, y as isize, z, w), cell);
            }
        }
        Ok(grid)
    }
}

impl FromStr for Cell {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "#" => Ok(Cell::Active),
            "." => Ok(Cell::Inactive),
            _ => Err(()),
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();

    for _generation in 0..6 {
        grid.next(false);
    }

    grid.active_count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();

    for _generation in 0..6 {
        grid.next(true);
    }

    grid.active_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "\
.#.\n\
..#\n\
###\n"
            ),
            112
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "\
.#.\n\
..#\n\
###\n"
            ),
            848
        )
    }
}
