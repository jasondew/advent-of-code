use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Clone, Debug)]
enum Cell {
    Box(usize),
    Wall,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    map: HashMap<Position, Cell>,
    bounds: (usize, usize),
    robot_position: Position,
}

impl Warehouse {
    fn move_robot(&mut self, direction: Direction) {
        let desired_position = Self::shift(&self.robot_position, &direction);
        match self.map.get(&desired_position) {
            Some(Cell::Box(id)) => {
                let saved_map = self.map.clone();
                if self.push_box(*id, &direction) {
                    self.robot_position = desired_position
                } else {
                    self.map = saved_map;
                }
            }
            Some(Cell::Wall) => {}
            None => self.robot_position = desired_position,
        }
    }

    fn push_box(&mut self, id: usize, direction: &Direction) -> bool {
        let mut positions: Vec<Position> = self
            .map
            .iter()
            .filter(|(_position, cell)| match cell {
                Cell::Box(box_id) => *box_id == id,
                _ => false,
            })
            .map(|(position, _cell)| position)
            .cloned()
            .collect();

        Self::sort_with_direction(&mut positions, &direction);

        for position in positions {
            let next_position = Self::shift(&position, &direction);

            match self.map.get(&next_position) {
                Some(Cell::Wall) => return false,
                Some(Cell::Box(next_id)) => {
                    if *next_id == id {
                        panic!("PANIK")
                    }

                    if self.push_box(*next_id, direction) {
                        self.move_cell(&position, next_position);
                    } else {
                        return false;
                    }
                }
                None => {
                    self.move_cell(&position, next_position);
                }
            }
        }

        true
    }

    fn sort_with_direction(
        positions: &mut Vec<Position>,
        direction: &Direction,
    ) {
        positions.sort_by_key(|position| match direction {
            Direction::Up => position.row,
            Direction::Down => 1000 - position.row,
            Direction::Left => position.col,
            Direction::Right => 1000 - position.col,
        })
    }

    fn move_cell(&mut self, from: &Position, to: Position) {
        let cell: Cell = self.map.remove(from).unwrap();
        self.map.insert(to, cell);
    }

    fn widen(&mut self) {
        let mut new_map: HashMap<Position, Cell> = HashMap::new();

        for row in 0..self.bounds.0 {
            for col in 0..self.bounds.1 {
                let position = Position { row, col };

                if let Some(cell) = self.map.get(&position) {
                    new_map
                        .insert(Position { row, col: col * 2 }, cell.clone());
                    new_map.insert(
                        Position {
                            row,
                            col: 2 * col + 1,
                        },
                        cell.clone(),
                    );
                }
            }
        }

        self.map = new_map;
        self.bounds = (self.bounds.0, self.bounds.1 * 2);
        self.robot_position = Position {
            row: self.robot_position.row,
            col: self.robot_position.col * 2,
        };
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.bounds.0 {
            for col in 0..self.bounds.1 {
                let position = Position { row, col };
                match self.map.get(&position) {
                    Some(Cell::Wall) => print!("#"),
                    Some(Cell::Box(_)) => print!("O"),
                    None => {
                        if position == self.robot_position {
                            print!("@")
                        } else {
                            print!(".")
                        }
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn shift(
        Position { row, col }: &Position,
        direction: &Direction,
    ) -> Position {
        match direction {
            Direction::Up => Position {
                row: row - 1,
                col: *col,
            },
            Direction::Down => Position {
                row: row + 1,
                col: *col,
            },
            Direction::Left => Position {
                row: *row,
                col: col - 1,
            },
            Direction::Right => Position {
                row: *row,
                col: col + 1,
            },
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (mut warehouse, moves) = parse(input);

    //    println!("Initial state:");
    //    warehouse.print();

    for direction in moves {
        //        println!("Direction {}:", &direction);
        warehouse.move_robot(direction);
        //        warehouse.print();
    }

    warehouse
        .map
        .iter()
        .map(|(position, cell)| match cell {
            Cell::Box(_) => position.row * 100 + position.col,
            Cell::Wall => 0,
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (mut warehouse, moves) = parse(input);
    let mut box_scores: HashMap<usize, usize> = HashMap::new();

    warehouse.widen();

    //    println!("Initial state:");
    //    warehouse.print();

    for direction in moves {
        //        println!("Direction {}:", &direction);
        warehouse.move_robot(direction);
        //        warehouse.print();
    }

    for (position, cell) in warehouse.map {
        match cell {
            Cell::Box(id) => {
                let score = position.row * 100 + position.col;
                box_scores
                    .entry(id)
                    .and_modify(|existing_score| {
                        if *existing_score > score {
                            *existing_score = score
                        }
                    })
                    .or_insert(score);
            }
            _ => {}
        }
    }

    box_scores.values().sum()
}

fn parse(input: &str) -> (Warehouse, Vec<Direction>) {
    let (map_string, moves_string) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<Position, Cell> = HashMap::new();
    let mut robot_position: Position = Position { row: 0, col: 0 };
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut boxes: usize = 0;

    for (row, line) in map_string.lines().enumerate() {
        cols = 0;
        for (col, ch) in line.chars().enumerate() {
            let position = Position { row, col };
            match ch {
                '#' => {
                    map.insert(position, Cell::Wall);
                }
                'O' => {
                    map.insert(position, Cell::Box(boxes));
                    boxes += 1;
                }
                '@' => {
                    robot_position = position;
                }
                '.' => (),
                _ => panic!("invalid map cell: {}", ch),
            };
            cols += 1;
        }
        rows += 1;
    }

    let moves: Vec<Direction> = moves_string
        .replace("\n", "")
        .chars()
        .map(|ch| match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("invalid move: {}", ch),
        })
        .collect();

    (
        Warehouse {
            map,
            bounds: (rows, cols),
            robot_position,
        },
        moves,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    }

    fn small_input() -> &'static str {
        "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
    }

    #[allow(dead_code)]
    fn part2_small_input() -> &'static str {
        "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(small_input()), 2028);
        assert_eq!(part1(input()), 10092);
    }

    #[test]
    fn part2_example() {
        //        assert_eq!(part2(part2_small_input()), 0);
        assert_eq!(part2(input()), 9021);
    }
}
