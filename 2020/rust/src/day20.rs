use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Cell {
    On,
    Off,
}

type TileID = usize;

#[derive(Debug)]
struct Tile {
    id: TileID,
    data: Vec<Vec<Cell>>,
    side_discriminants: HashMap<usize, (Side, Orientation)>,
}

impl Tile {
    fn parse(input: &str) -> Vec<Self> {
        input
            .trim()
            .split("\n\n")
            .map(|tile_lines| tile_lines.parse().unwrap())
            .collect()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "ID: {} side numbers: {:?}",
            self.id, self.side_discriminants
        );
        for row in &self.data {
            for cell in row {
                match cell {
                    Cell::On => print!("#"),
                    Cell::Off => print!("."),
                }
            }
            println!();
        }
    }
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = string.trim().lines().collect();
        let id: TileID = TileID::from_str_radix(&lines[0][5..9], 10).unwrap();
        let data: Vec<Vec<Cell>> = lines[1..]
            .iter()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '#' => Cell::On,
                        '.' => Cell::Off,
                        _ => panic!("invalid cell"),
                    })
                    .collect()
            })
            .collect();

        fn side_discriminant(data: &Vec<Cell>) -> usize {
            let binary_string: String = data
                .iter()
                .map(|cell| match cell {
                    Cell::On => "1",
                    Cell::Off => "0",
                })
                .collect::<Vec<&str>>()
                .join("");
            usize::from_str_radix(&binary_string, 2).unwrap()
        }

        use Orientation::{Initial, Reversed};
        use Side::{Bottom, Left, Right, Top};

        let mut side_discriminants: HashMap<usize, (Side, Orientation)> = HashMap::new();

        let mut top_row: Vec<Cell> = data[0].clone();
        side_discriminants.insert(side_discriminant(&top_row), (Top, Initial));
        top_row.reverse();
        side_discriminants.insert(side_discriminant(&top_row), (Top, Reversed));

        let mut bottom_row: Vec<Cell> = data[9].clone();
        side_discriminants.insert(side_discriminant(&bottom_row), (Bottom, Initial));
        bottom_row.reverse();
        side_discriminants.insert(side_discriminant(&bottom_row), (Bottom, Reversed));

        let mut left_row: Vec<Cell> = data.iter().map(|row| row[0]).collect();
        side_discriminants.insert(side_discriminant(&left_row), (Left, Initial));
        left_row.reverse();
        side_discriminants.insert(side_discriminant(&left_row), (Left, Reversed));

        let mut right_row: Vec<Cell> = data.iter().map(|row| row[9]).collect();
        side_discriminants.insert(side_discriminant(&right_row), (Right, Initial));
        right_row.reverse();
        side_discriminants.insert(side_discriminant(&right_row), (Right, Reversed));

        Ok(Tile {
            id,
            data,
            side_discriminants,
        })
    }
}

#[derive(Debug)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
enum Orientation {
    Initial,
    Reversed,
}

#[derive(Debug)]
struct Connections {
    map: HashMap<TileID, Vec<TileID>>,
    max: usize,
}

impl Connections {
    fn new(tiles: &[Tile]) -> Self {
        let mut map: HashMap<TileID, Vec<TileID>> = HashMap::new();
        let mut max: usize = 0;

        fn add_connection(
            map: &mut HashMap<TileID, Vec<TileID>>,
            max: &mut usize,
            id: TileID,
            other_id: TileID,
        ) {
            if let Some(existing_ids) = map.get_mut(&id) {
                existing_ids.push(other_id);
                if existing_ids.len() > *max {
                    *max = existing_ids.len();
                }
            } else {
                map.insert(id, vec![other_id]);
            }
        }

        for (index, tile) in tiles.iter().enumerate() {
            for other_tile in tiles.iter().skip(index + 1) {
                if tile.side_discriminants.keys().any(|side_discriminant| {
                    other_tile
                        .side_discriminants
                        .contains_key(side_discriminant)
                }) {
                    add_connection(&mut map, &mut max, tile.id, other_tile.id);
                    add_connection(&mut map, &mut max, other_tile.id, tile.id);
                }
            }
        }

        Self { map, max }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let tiles: Vec<Tile> = Tile::parse(input);
    let connections: Connections = Connections::new(&tiles);
    let corner_tiles: Vec<&Tile> = find_corner_tiles(&tiles, &connections);

    corner_tiles.iter().map(|tile| tile.id).product()
}

fn find_corner_tiles<'a>(tiles: &'a [Tile], connections: &Connections) -> Vec<&'a Tile> {
    let mut excluded_ids: Vec<TileID> = Vec::new();

    for (id, other_ids) in &connections.map {
        if other_ids.len() == connections.max {
            excluded_ids.push(*id);
            for id in other_ids {
                excluded_ids.push(*id);
            }
        }
    }

    tiles
        .iter()
        .filter(|tile| !excluded_ids.contains(&&tile.id))
        .collect()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let tiles: Vec<Tile> = Tile::parse(input);
    let connections: Connections = Connections::new(&tiles);
    let length: usize = (tiles.len() as f32).sqrt() as usize;
    let mut tile_ids_used: Vec<TileID> = Vec::new();
    let corner_tile: &Tile = find_corner_tiles(&tiles, &connections)[0];

    tile_ids_used.push(corner_tile.id);

    //    dbg!(&connections.map[&corner_tile.id]);

    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 20899048083289)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 273)
    }

    fn input() -> &'static str {
        "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"
    }
}
