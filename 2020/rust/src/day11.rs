#[derive(Debug, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn step(&self) -> Grid {
        let next_data = self
            .data
            .iter()
            .enumerate()
            .map(|(row, rows)| {
                rows.iter()
                    .enumerate()
                    .map(|(col, cell)| self.next_cell(cell, row, col))
                    .collect()
            })
            .collect();

        Grid {
            data: next_data,
            ..*self
        }
    }

    fn next_cell(&self, cell: &Cell, row: usize, col: usize) -> Cell {
        use Cell::*;

        match cell {
            Floor => Floor,
            EmptySeat => {
                if self.adjacent_occupied_seat_count(row, col) == 0 {
                    OccupiedSeat
                } else {
                    EmptySeat
                }
            }
            OccupiedSeat => {
                if self.adjacent_occupied_seat_count(row, col) >= 4 {
                    EmptySeat
                } else {
                    OccupiedSeat
                }
            }
        }
    }

    fn adjacent_occupied_seat_count(&self, row: usize, col: usize) -> usize {
        let adjacent_locations: Vec<Option<(usize, usize)>> = vec![
            row.checked_sub(1).zip(col.checked_sub(1)),
            row.checked_sub(1).map(|x| (x, col)),
            row.checked_sub(1).map(|x| (x, col + 1)),
            col.checked_sub(1).map(|y| (row, y)),
            Some((row, col + 1)),
            col.checked_sub(1).map(|y| (row + 1, y)),
            Some((row + 1, col)),
            Some((row + 1, col + 1)),
        ];

        adjacent_locations
            .iter()
            .map(|maybe_location| match maybe_location {
                Some((row, col)) => {
                    if self.cell_at(*row, *col) == Some(&Cell::OccupiedSeat) {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            })
            .sum()
    }

    fn step_all_directions(&self) -> Grid {
        let next_data = self
            .data
            .iter()
            .enumerate()
            .map(|(row, rows)| {
                rows.iter()
                    .enumerate()
                    .map(|(col, cell)| self.next_cell_all_directions(cell, row, col))
                    .collect()
            })
            .collect();

        Grid {
            data: next_data,
            ..*self
        }
    }

    fn next_cell_all_directions(&self, cell: &Cell, row: usize, col: usize) -> Cell {
        use Cell::*;

        match cell {
            Floor => Floor,
            EmptySeat => {
                if self.all_directions_occupied_seat_count(row, col) == 0 {
                    OccupiedSeat
                } else {
                    EmptySeat
                }
            }
            OccupiedSeat => {
                if self.all_directions_occupied_seat_count(row, col) >= 5 {
                    EmptySeat
                } else {
                    OccupiedSeat
                }
            }
        }
    }

    fn all_directions_occupied_seat_count(&self, row: usize, col: usize) -> usize {
        let north_west = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            row.checked_sub(1).zip(col.checked_sub(1))
        };
        let north = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            row.checked_sub(1).map(|x| (x, col))
        };
        let north_east = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            if col + 1 >= self.cols {
                None
            } else {
                row.checked_sub(1).map(|x| (x, col + 1))
            }
        };
        let south_west = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            if row + 1 >= self.rows {
                None
            } else {
                col.checked_sub(1).map(|y| (row + 1, y))
            }
        };
        let south = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            if (row + 1) >= self.rows {
                None
            } else {
                Some((row + 1, col))
            }
        };
        let south_east = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            if col + 1 >= self.cols || row + 1 >= self.rows {
                None
            } else {
                Some((row + 1, col + 1))
            }
        };
        let west = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            col.checked_sub(1).map(|y| (row, y))
        };
        let east = |(row, col): (usize, usize)| -> Option<(usize, usize)> {
            if (col + 1) >= self.cols {
                None
            } else {
                Some((row, col + 1))
            }
        };

        let mut count: usize = 0;

        if self.occupied_seat_in_direction(row, col, &north) {
            count += 1
        };

        if self.occupied_seat_in_direction(row, col, &south) {
            count += 1
        }
        if self.occupied_seat_in_direction(row, col, &east) {
            count += 1
        }
        if self.occupied_seat_in_direction(row, col, &west) {
            count += 1
        }
        if self.occupied_seat_in_direction(row, col, &north_west) {
            count += 1
        }
        if self.occupied_seat_in_direction(row, col, &north_east) {
            count += 1
        }
        if self.occupied_seat_in_direction(row, col, &south_west) {
            count += 1
        }
        if self.occupied_seat_in_direction(row, col, &south_east) {
            count += 1
        }

        count
    }

    fn occupied_seat_in_direction(
        &self,
        starting_row: usize,
        starting_col: usize,
        direction_fn: &dyn Fn((usize, usize)) -> Option<(usize, usize)>,
    ) -> bool {
        let mut row: usize = starting_row;
        let mut col: usize = starting_col;

        loop {
            if let Some((x, y)) = direction_fn((row, col)) {
                row = x;
                col = y;
                match self.cell_at(row, col) {
                    Some(&Cell::EmptySeat) => break false,
                    Some(&Cell::OccupiedSeat) => break true,
                    _ => 0,
                };
            } else {
                break false;
            }
        }
    }

    fn cell_at(&self, row: usize, col: usize) -> Option<&Cell> {
        self.data.get(row).and_then(|row_vec| row_vec.get(col))
    }

    fn occupied_seat_count(&self) -> usize {
        self.data
            .iter()
            .map(|rows| {
                rows.iter()
                    .filter(|&cell| cell == &Cell::OccupiedSeat)
                    .count()
            })
            .sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        use Cell::*;
        for row in &self.data {
            for cell in row {
                print!(
                    "{}",
                    match cell {
                        OccupiedSeat => "#",
                        EmptySeat => "L",
                        Floor => ".",
                    }
                )
            }
            println!("")
        }
    }
}

impl std::str::FromStr for Grid {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        use Cell::*;
        let data: Vec<Vec<Cell>> = string
            .lines()
            .map(|line| {
                line.chars()
                    .map(|cell| match cell {
                        '.' => Floor,
                        'L' => EmptySeat,
                        '#' => OccupiedSeat,
                        _ => panic!("unexpected input"),
                    })
                    .collect()
            })
            .collect();
        let rows: usize = data.len();
        let cols: usize = data.get(0).unwrap().len();

        Ok(Grid { data, rows, cols })
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    let mut occupied_seat_count: usize = grid.occupied_seat_count();

    loop {
        grid = grid.step();
        let next_occupied_seat_count = grid.occupied_seat_count();
        if next_occupied_seat_count == occupied_seat_count {
            break occupied_seat_count;
        } else {
            occupied_seat_count = next_occupied_seat_count;
        }
    }
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    let mut occupied_seat_count: usize = grid.occupied_seat_count();
    let mut steps: usize = 0;

    loop {
        grid = grid.step_all_directions();
        steps += 1;

        let next_occupied_seat_count = grid.occupied_seat_count();
        if next_occupied_seat_count == occupied_seat_count || steps > 100 {
            break occupied_seat_count;
        } else {
            occupied_seat_count = next_occupied_seat_count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n"), 37)
    }

    #[test]
    fn part2_example() {
        let mut grid: Grid = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n".parse().unwrap();
        assert_eq!(grid.all_directions_occupied_seat_count(0, 2), 0);
        grid = grid.step_all_directions();
        assert_eq!(grid.all_directions_occupied_seat_count(0, 2), 5);
        grid = grid.step_all_directions();
        grid.print();
        assert_eq!(grid.all_directions_occupied_seat_count(0, 2), 1);

        assert_eq!(part2("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n"), 26)
    }
}
