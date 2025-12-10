use std::collections::HashMap;

type Position = (usize, usize);

#[derive(PartialEq, Eq)]
enum Tile {
    Open,
    RollOfPaper,
}

struct Map {
    tiles: HashMap<Position, Tile>,
    rows: usize,
    cols: usize,
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                match self.tiles.get(&(row, col)) {
                    Some(Tile::RollOfPaper) => print!("@"),
                    Some(Tile::Open) => print!("."),
                    None => print!(" "),
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_with_accessible_rolls(&self, accessible_rolls: &Vec<Position>) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if accessible_rolls.contains(&(row, col)) {
                    print!("x");
                } else {
                    match self.tiles.get(&(row, col)) {
                        Some(Tile::RollOfPaper) => print!("@"),
                        Some(Tile::Open) => print!("."),
                        None => print!(" "),
                    }
                }
            }
            println!();
        }
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<Position> {
        let mut neighbors = Vec::<Position>::new();
        let positions = [
            row.checked_sub(1)
                .and_then(|row| col.checked_sub(1).map(|col| (row, col))), // NW
            row.checked_sub(1).map(|row| (row, col)), // N
            row.checked_sub(1).map(|row| (row, col + 1)), // NE
            //
            col.checked_sub(1).map(|col| (row, col)), // W
            Some((row, col + 1)),                     // E
            //
            col.checked_sub(1).map(|col| (row + 1, col)), // SW
            Some((row + 1, col)),                         // E
            Some((row + 1, col + 1)),                     // E
        ];

        for maybe_position in positions {
            if let Some(position) = maybe_position
                && self.tiles.get(&position) == Some(&Tile::RollOfPaper)
            {
                neighbors.push(position);
            }
        }

        neighbors
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let map = parse_map(input);
    let mut accessible_rolls = Vec::<Position>::new();

    for row in 0..map.rows {
        for col in 0..map.cols {
            if let Some(Tile::RollOfPaper) = map.tiles.get(&(row, col))
                && (map.neighbors(row, col).len() < 4)
            {
                accessible_rolls.push((row, col));
            }
        }
    }

    accessible_rolls.len()
}

fn parse_map(input: &str) -> Map {
    let mut tiles = HashMap::new();
    let rows = input.trim().lines().count();
    let cols = input.lines().next().unwrap().trim().chars().count();

    for (row, line) in input.trim().lines().enumerate() {
        for (col, ch) in line.trim().chars().enumerate() {
            match ch {
                '@' => {
                    tiles.insert((row, col), Tile::RollOfPaper);
                }
                '.' => {
                    tiles.insert((row, col), Tile::Open);
                }
                _ => {
                    panic!("Unexpected character in map: {}", ch);
                }
            }
        }
    }

    Map { tiles, rows, cols }
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut map = parse_map(input);
    let mut accessible_roll_count = 0usize;
    let mut accessible_rolls = Vec::<Position>::new();

    loop {
        for row in 0..map.rows {
            for col in 0..map.cols {
                if let Some(Tile::RollOfPaper) = map.tiles.get(&(row, col))
                    && (map.neighbors(row, col).len() < 4)
                {
                    accessible_rolls.push((row, col));
                }
            }
        }

        if accessible_rolls.is_empty() {
            break;
        } else {
            accessible_roll_count += accessible_rolls.len();
            for position in &accessible_rolls {
                map.tiles.remove(position);
            }
            accessible_rolls.clear();
        }
    }

    accessible_roll_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 43);
    }
}
