#[derive(Debug, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Cell>>,
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

        Grid { data: next_data }
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
        Ok(Grid { data })
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
    input.len()
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
        assert_eq!(part2("quux"), 4)
    }
}
