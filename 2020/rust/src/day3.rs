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

#[must_use]
pub fn part1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    collisions(&map, (0, 0), (3, 1), 0)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    collisions(&map, (0, 0), (1, 1), 0)
        * collisions(&map, (0, 0), (3, 1), 0)
        * collisions(&map, (0, 0), (5, 1), 0)
        * collisions(&map, (0, 0), (7, 1), 0)
        * collisions(&map, (0, 0), (1, 2), 0)
}

fn collisions(map: &Map, (x, y): (usize, usize), (vx, vy): (usize, usize), trees: usize) -> usize {
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
        let input = "..##.......\n\
                     #...#...#..\n\
                     .#....#..#.\n\
                     ..#.#...#.#\n\
                     .#...##..#.\n\
                     ..#.##.....\n\
                     .#.#.#....#\n\
                     .#........#\n\
                     #.##...#...\n\
                     #...##....#\n\
                     .#..#...#.#";

        assert_eq!(part1(&input), 7)
    }

    #[test]
    fn part2_example() {
        let input = "..##.......\n\
                     #...#...#..\n\
                     .#....#..#.\n\
                     ..#.#...#.#\n\
                     .#...##..#.\n\
                     ..#.##.....\n\
                     .#.#.#....#\n\
                     .#........#\n\
                     #.##...#...\n\
                     #...##....#\n\
                     .#..#...#.#";

        assert_eq!(part2(&input), 336)
    }
}
