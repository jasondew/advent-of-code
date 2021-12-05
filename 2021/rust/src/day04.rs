use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Unmarked(usize),
    Marked(usize),
}

#[derive(Debug)]
struct Board {
    row_major: Vec<Vec<Cell>>,
    column_major: Vec<Vec<Cell>>,
}

impl Board {
    fn call(&mut self, number: usize) {
        for row in self.row_major.iter_mut() {
            for cell in row.iter_mut() {
                if cell == &Cell::Unmarked(number) {
                    *cell = Cell::Marked(number)
                }
            }
        }

        for column in self.column_major.iter_mut() {
            for cell in column.iter_mut() {
                if cell == &Cell::Unmarked(number) {
                    *cell = Cell::Marked(number)
                }
            }
        }
    }

    fn won(&self) -> bool {
        let won_by_row = self
            .row_major
            .iter()
            .find(|row| {
                row.iter().all(|cell| match cell {
                    Cell::Marked(_) => true,
                    Cell::Unmarked(_) => false,
                })
            })
            .is_some();
        let won_by_column = self
            .column_major
            .iter()
            .find(|column| {
                column.iter().all(|cell| match cell {
                    Cell::Marked(_) => true,
                    Cell::Unmarked(_) => false,
                })
            })
            .is_some();

        won_by_row || won_by_column
    }

    fn unmarked_sum(&self) -> usize {
        self.row_major
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Cell::Unmarked(value) => value,
                        Cell::Marked(_) => &0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl FromStr for Board {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| Cell::Unmarked(x.parse().unwrap()))
                    .collect()
            })
            .collect();
        let mut transposed_cells = vec![vec![Cell::Unmarked(0); cells.len()]; cells[0].len()];

        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                transposed_cells[x][y] = cell.clone();
            }
        }

        Ok(Board {
            row_major: cells,
            column_major: transposed_cells,
        })
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for number in numbers {
        for board in boards.iter_mut() {
            board.call(number);
            if board.won() {
                return number * board.unmarked_sum();
            }
        }
    }

    return 0;
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for number in numbers {
        for board in boards.iter_mut() {
            board.call(number);
        }

        if boards.len() == 1 && boards[0].won() {
            return number * boards[0].unmarked_sum();
        } else {
            boards = boards.into_iter().filter(|board| !board.won()).collect();
        }
    }

    return 0;
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let (numbers_string, boards_string) = input.split_once("\n\n").unwrap();
    let numbers: Vec<usize> = numbers_string
        .split(",")
        .map(|string| string.parse().unwrap())
        .collect();
    let boards: Vec<Board> = boards_string
        .split("\n\n")
        .map(|board_string| board_string.parse().unwrap())
        .collect();

    (numbers, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 4512)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 1924)
    }
}
