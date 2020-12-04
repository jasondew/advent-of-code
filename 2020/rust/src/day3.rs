#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
enum Cell {
    Open,
    Tree,
}

#[derive(Debug)]
pub struct Map {
    data: Vec<Vec<Cell>>,
    rows: usize,
    columns: usize,
}

impl Map {
    fn at(&self, x: usize, y: usize) -> &Cell {
        self.data.get(y).unwrap().get(x).unwrap()
    }
}

impl std::str::FromStr for Map {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        let data: Vec<Vec<Cell>> = string
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '#' => Cell::Tree,
                        '.' => Cell::Open,
                        _ => panic!("invalid character found: {:?}", ch),
                    })
                    .collect()
            })
            .collect();
        let rows = data.len();
        let columns = data.get(0).unwrap().len();

        Ok(Map {
            data,
            rows,
            columns,
        })
    }
}

pub fn part1(map: &Map) -> i64 {
    collisions(map, (0, 0), (3, 1), 0)
}

pub fn part2(map: &Map) -> i64 {
    collisions(map, (0, 0), (1, 1), 0)
        * collisions(map, (0, 0), (3, 1), 0)
        * collisions(map, (0, 0), (5, 1), 0)
        * collisions(map, (0, 0), (7, 1), 0)
        * collisions(map, (0, 0), (1, 2), 0)
}

fn collisions(map: &Map, (x, y): (usize, usize), (vx, vy): (usize, usize), trees: i64) -> i64 {
    if y >= map.rows {
        trees
    } else {
        let next_y = y + vy;
        let next_x = (x + vx) % map.columns;

        if *map.at(x, y) == Cell::Tree {
            collisions(map, (next_x, next_y), (vx, vy), trees + 1)
        } else {
            collisions(map, (next_x, next_y), (vx, vy), trees)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let map: Map = "..##.......\n\
                       #...#...#..\n\
                       .#....#..#.\n\
                       ..#.#...#.#\n\
                       .#...##..#.\n\
                       ..#.##.....\n\
                       .#.#.#....#\n\
                       .#........#\n\
                       #.##...#...\n\
                       #...##....#\n\
                       .#..#...#.#"
            .parse()
            .unwrap();

        assert_eq!(part1(&map), 7)
    }

    #[test]
    fn part2_example() {
        let map: Map = "..##.......\n\
                       #...#...#..\n\
                       .#....#..#.\n\
                       ..#.#...#.#\n\
                       .#...##..#.\n\
                       ..#.##.....\n\
                       .#.#.#....#\n\
                       .#........#\n\
                       #.##...#...\n\
                       #...##....#\n\
                       .#..#...#.#"
            .parse()
            .unwrap();

        assert_eq!(part2(&map), 336)
    }
}
