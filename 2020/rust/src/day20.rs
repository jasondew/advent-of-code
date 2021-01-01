use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    On,
    Off,
}

impl std::fmt::Debug for Cell {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Cell::On => formatter.write_str("#"),
            Cell::Off => formatter.write_str("."),
        }
    }
}

type TileID = usize;
type Discriminant = String;

#[derive(Debug)]
struct Tile {
    id: TileID,
    cells: Vec<Vec<Cell>>,
    side_discriminants: HashMap<Discriminant, (Side, Orientation)>,
}

struct LockedTile {
    id: TileID,
    cells: Vec<Vec<Cell>>,
    cells_with_borders: Vec<Vec<Cell>>,
    transform: Transform,
    side_discriminants: HashMap<Side, Discriminant>,
}

impl std::fmt::Debug for LockedTile {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(
            format!(
                "LockedTile: ID={} Transform={:?} SideDiscriminants={:?}",
                self.id, self.transform, self.side_discriminants
            )
            .as_str(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Orientation {
    Normal,
    Reversed,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Transform {
    Nothing,
    FlipVertically,
    FlipHorizontally,
    FlipBoth,
    RotateLeft,
    RotateRight,
    RotateLeftAndFlipHorizontally,
    RotateRightAndFlipHorizontally,
}

#[derive(Debug)]
struct Connections {
    map: HashMap<TileID, Vec<TileID>>,
    max: usize,
}

#[derive(Debug)]
struct Grid<'a> {
    tiles: Vec<Vec<&'a LockedTile>>,
}

#[derive(Debug)]
struct Image {
    cells: Vec<Vec<Cell>>,
}

impl std::str::FromStr for Tile {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = string.trim().lines().collect();
        let id: TileID = TileID::from_str_radix(&lines[0][5..9], 10).unwrap();
        let cells: Vec<Vec<Cell>> = lines[1..]
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

        use Orientation::{Normal, Reversed};
        use Side::{Bottom, Left, Right, Top};

        let mut side_discriminants: HashMap<Discriminant, (Side, Orientation)> = HashMap::new();

        let mut top_row: Vec<Cell> = cells[0].clone();
        side_discriminants.insert(Tile::side_discriminant(&top_row), (Top, Normal));
        top_row.reverse();
        side_discriminants.insert(Tile::side_discriminant(&top_row), (Top, Reversed));

        let mut bottom_row: Vec<Cell> = cells[9].clone();
        side_discriminants.insert(Tile::side_discriminant(&bottom_row), (Bottom, Normal));
        bottom_row.reverse();
        side_discriminants.insert(Tile::side_discriminant(&bottom_row), (Bottom, Reversed));

        let mut left_row: Vec<Cell> = cells.iter().map(|row| row[0]).collect();
        side_discriminants.insert(Tile::side_discriminant(&left_row), (Left, Normal));
        left_row.reverse();
        side_discriminants.insert(Tile::side_discriminant(&left_row), (Left, Reversed));

        let mut right_row: Vec<Cell> = cells.iter().map(|row| row[9]).collect();
        side_discriminants.insert(Tile::side_discriminant(&right_row), (Right, Normal));
        right_row.reverse();
        side_discriminants.insert(Tile::side_discriminant(&right_row), (Right, Reversed));

        Ok(Tile {
            id,
            cells,
            side_discriminants,
        })
    }
}

impl Tile {
    fn parse(input: &str) -> Vec<Self> {
        input
            .trim()
            .split("\n\n")
            .map(|tile_lines| tile_lines.parse().unwrap())
            .collect()
    }

    fn side_discriminant(cells: &Vec<Cell>) -> Discriminant {
        cells
            .iter()
            .map(|cell| match cell {
                Cell::On => "#",
                Cell::Off => ".",
            })
            .collect::<Vec<&str>>()
            .join("")
    }

    fn print_cells(cells: &Vec<Vec<Cell>>) {
        for row in cells {
            for cell in row {
                print!("{:?} ", cell);
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "ID: {} side discriminants: {:?}",
            self.id, self.side_discriminants
        );
        Self::print_cells(&self.cells);
    }
}

impl LockedTile {
    fn from_tile(tile: &Tile, transform: Transform) -> Self {
        let cells: Vec<Vec<Cell>> = transform.apply(tile.cells.clone());
        let mut side_discriminants: HashMap<Side, Discriminant> = HashMap::new();

        side_discriminants.insert(Side::Top, Tile::side_discriminant(&cells[0]));
        side_discriminants.insert(Side::Bottom, Tile::side_discriminant(&cells[9]));
        side_discriminants.insert(
            Side::Left,
            Tile::side_discriminant(&cells.iter().map(|row| row[0]).collect()),
        );
        side_discriminants.insert(
            Side::Right,
            Tile::side_discriminant(&cells.iter().map(|row| row[9]).collect()),
        );

        Self {
            id: tile.id,
            cells: Self::remove_borders(&cells),
            cells_with_borders: cells,
            transform,
            side_discriminants,
        }
    }

    fn remove_borders(cells: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        cells
            .iter()
            .skip(1)
            .take(cells.len() - 2)
            .map(|row| row.iter().skip(1).take(row.len() - 2).copied().collect())
            .collect()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("\n# Locked Tile ID {} ({:?})", self.id, self.transform);
        println!();
        Tile::print_cells(&self.cells_with_borders);
        println!();
    }
}

impl Transform {
    fn from_adjacent_tiles(
        adjacent_tiles: &HashMap<TileID, ((Side, Side), (Orientation, Orientation))>,
    ) -> Self {
        adjacent_tiles
            .iter()
            .map(|(_tile_id, (sides, _orientations))| match sides {
                (Side::Top, Side::Bottom) => Transform::FlipVertically,
                (Side::Top, Side::Left) => Transform::RotateRight,
                (Side::Bottom, Side::Top) => Transform::Nothing,
                (Side::Bottom, Side::Left) => Transform::Nothing,
                (Side::Bottom, Side::Right) => Transform::Nothing,
                (Side::Right, Side::Left) => Transform::Nothing,
                (Side::Right, Side::Right) => Transform::FlipHorizontally,
                (Side::Right, Side::Bottom) => Transform::RotateRight,
                (Side::Left, Side::Right) => Transform::FlipHorizontally,
                _ => panic!(format!("invalid side combination: {:?}", sides)),
            })
            .fold(Transform::Nothing, |acc, transform| {
                match (acc, transform) {
                    (Transform::Nothing, transform) => transform,
                    (transform, Transform::Nothing) => transform,
                    (transform, other_transform) if transform == other_transform => transform,
                    (Transform::FlipVertically, Transform::FlipHorizontally) => Transform::FlipBoth,
                    (Transform::FlipHorizontally, Transform::FlipVertically) => Transform::FlipBoth,
                    (Transform::RotateRight, Transform::FlipHorizontally)
                    | (Transform::FlipHorizontally, Transform::RotateRight) => {
                        Transform::RotateRightAndFlipHorizontally
                    }
                    pair => panic!(format!("{:?} shouldn't happen", pair)),
                }
            })
    }

    fn apply(&self, cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        fn flip_horizontally(cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
            cells
                .iter()
                .map(|row| row.iter().rev().copied().collect())
                .collect()
        }

        fn flip_vertically(cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
            cells.iter().rev().map(|row| row.clone()).collect()
        }

        fn rotate_left(cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
            let mut new_cells: Vec<Vec<Cell>> = (0..cells.len()).map(|_| Vec::new()).collect();
            let mut index: usize;

            for row in &cells {
                index = cells.len();
                for &cell in row {
                    index -= 1;
                    new_cells[index].push(cell);
                }
            }

            new_cells
        }

        fn rotate_right(cells: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
            let mut new_cells: Vec<Vec<Cell>> = (0..cells.len()).map(|_| Vec::new()).collect();
            let mut index: usize;

            for row in &cells {
                index = 0;
                for &cell in row {
                    new_cells[index].push(cell);
                    index += 1;
                }
            }

            new_cells
        }

        match self {
            Transform::Nothing => cells,
            Transform::FlipVertically => flip_vertically(cells),
            Transform::FlipHorizontally => flip_horizontally(cells),
            Transform::FlipBoth => flip_vertically(flip_horizontally(cells)),
            Transform::RotateLeft => rotate_left(cells),
            Transform::RotateLeftAndFlipHorizontally => flip_horizontally(rotate_left(cells)),
            Transform::RotateRight => rotate_right(cells),
            Transform::RotateRightAndFlipHorizontally => flip_horizontally(rotate_right(cells)),
        }
    }
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

impl<'a> Grid<'a> {
    fn export(&self) -> Image {
        let tile_length = self
            .tiles
            .iter()
            .nth(0)
            .unwrap()
            .iter()
            .nth(0)
            .unwrap()
            .cells
            .len();
        let length: usize = tile_length * self.tiles.len();
        let mut cells: Vec<Vec<Cell>> = (0..length).map(|_| vec![]).collect();
        let mut output_row: usize = 0;

        for grid_row in &self.tiles {
            for input_row in 0..tile_length {
                for tile in grid_row {
                    for cell in tile.cells.iter().nth(input_row).unwrap() {
                        cells[output_row].push(cell.clone())
                    }
                }
                output_row += 1;
            }
        }

        Image { cells }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let tile_length = self
            .tiles
            .iter()
            .nth(0)
            .unwrap()
            .iter()
            .nth(0)
            .unwrap()
            .cells_with_borders
            .len();
        for grid_row in &self.tiles {
            for tile in grid_row {
                print!("[  {}  ] ", tile.id);
            }
            println!();

            for input_row in 0..tile_length {
                for tile in grid_row {
                    for cell in tile.cells_with_borders.iter().nth(input_row).unwrap() {
                        print!("{:?}", cell);
                    }
                    print!(" ");
                }
                println!();
            }
        }
    }
}

impl Image {
    fn find_sea_monsters(&self) -> Vec<(usize, usize)> {
        //                   #
        // #    ##    ##    ###
        //  #  #  #  #  #  #
        let sea_monster: Vec<(usize, usize)> = vec![
            (0, 18),
            (1, 0),
            (1, 5),
            (1, 6),
            (1, 11),
            (1, 12),
            (1, 17),
            (1, 18),
            (1, 19),
            (2, 1),
            (2, 4),
            (2, 7),
            (2, 10),
            (2, 13),
            (2, 16),
        ];
        let mut found_locations: Vec<(usize, usize)> = vec![];

        for row_offset in 0..(self.cells.len() - 3) {
            for col_offset in 0..(self.cells.len() - 20) {
                if sea_monster
                    .iter()
                    .all(|(row, col)| self.is_on(row + row_offset, col + col_offset))
                {
                    found_locations.push((row_offset, col_offset));
                }
            }
        }

        found_locations
    }

    fn is_on(&self, row: usize, col: usize) -> bool {
        self.cells[row][col] == Cell::On
    }

    fn flip(&mut self) {
        for row in self.cells.iter_mut() {
            *row = row.iter().rev().copied().collect::<Vec<Cell>>();
        }
    }

    fn rotate(&mut self) {
        let mut new_cells: Vec<Vec<Cell>> = (0..self.cells.len()).map(|_| vec![]).collect();
        let mut index: usize;

        for row in &self.cells {
            index = 0;
            for &cell in row {
                new_cells[index].push(cell);
                index += 1;
            }
        }

        self.cells = new_cells;
    }

    fn on_count(&self) -> usize {
        let mut count: usize = 0;

        for row in &self.cells {
            for cell in row {
                if cell == &Cell::On {
                    count += 1
                }
            }
        }

        count
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.cells {
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

    let mut all_locked_tiles: Vec<LockedTile> = vec![];

    for tile in &tiles {
        all_locked_tiles.push(LockedTile::from_tile(tile, Transform::Nothing));
        all_locked_tiles.push(LockedTile::from_tile(tile, Transform::RotateRight));
        all_locked_tiles.push(LockedTile::from_tile(tile, Transform::RotateLeft));
        all_locked_tiles.push(LockedTile::from_tile(
            tile,
            Transform::RotateLeftAndFlipHorizontally,
        ));
        all_locked_tiles.push(LockedTile::from_tile(
            tile,
            Transform::RotateRightAndFlipHorizontally,
        ));
        all_locked_tiles.push(LockedTile::from_tile(tile, Transform::FlipVertically));
        all_locked_tiles.push(LockedTile::from_tile(tile, Transform::FlipHorizontally));
        all_locked_tiles.push(LockedTile::from_tile(tile, Transform::FlipBoth));
    }

    let grid: Grid = assemble_grid(&tiles, &all_locked_tiles);
    //    grid.print();

    let mut image: Image = grid.export();
    //    image.print();

    for _rotations in 0..3 {
        image.rotate();
        for _flips in 0..2 {
            image.flip();
            let sea_monsters = image.find_sea_monsters();

            if sea_monsters.len() > 0 {
                return image.on_count() - sea_monsters.len() * 15;
            }
        }
    }

    panic!("if we got here, we failed :'(");
}

fn assemble_grid<'a>(
    all_tiles: &Vec<Tile>,
    possible_locked_tiles: &'a Vec<LockedTile>,
) -> Grid<'a> {
    let length: usize = (all_tiles.len() as f32).sqrt() as usize;
    let mut tiles: Vec<Vec<&LockedTile>> = (0..length).map(|_| vec![]).collect();
    let mut used_tile_ids: Vec<TileID> = vec![];

    let connections: Connections = Connections::new(&all_tiles);
    let corner_tile: &Tile = find_corner_tiles(&all_tiles, &connections)[0];

    let adjacent_tiles = find_adjacent_tiles(&corner_tile, &all_tiles);
    let transform = Transform::from_adjacent_tiles(&adjacent_tiles);
    let corner_locked_tile = possible_locked_tiles
        .iter()
        .find(|tile| tile.id == corner_tile.id && tile.transform == transform)
        .unwrap();

    used_tile_ids.push(corner_tile.id);
    tiles[0].push(&corner_locked_tile);

    #[allow(dead_code)]
    fn print_tiles(tiles: &Vec<Vec<LockedTile>>) {
        for row in tiles {
            for tile in row {
                println!("{} ", tile.id);
                Tile::print_cells(&tile.cells_with_borders);
            }
            println!();
        }
        println!();
    }

    for col in 0..length {
        for row in 1..length {
            //            print_tiles(&tiles);
            let previous_locked_tile = &tiles[row - 1][col];
            let next_locked_tile = next_locked_tile(
                &possible_locked_tiles,
                &used_tile_ids,
                Side::Top,
                previous_locked_tile.side_discriminants[&Side::Bottom].clone(),
            );

            //            next_locked_tile.print();
            used_tile_ids.push(next_locked_tile.id);
            tiles[row].push(next_locked_tile);
        }

        if col < length - 1 {
            let next_column_locked_tile = next_locked_tile(
                &possible_locked_tiles,
                &used_tile_ids,
                Side::Left,
                tiles[0][col].side_discriminants[&Side::Right].clone(),
            );
            //            next_column_locked_tile.print();
            used_tile_ids.push(next_column_locked_tile.id);
            tiles[0].push(next_column_locked_tile);
        }
    }

    Grid { tiles }
}

fn next_locked_tile<'a>(
    possible_locked_tiles: &'a Vec<LockedTile>,
    used_tile_ids: &Vec<TileID>,
    target_side: Side,
    discriminant: Discriminant,
) -> &'a LockedTile {
    possible_locked_tiles
        .iter()
        .filter(|tile| !used_tile_ids.contains(&tile.id))
        .find(|tile| tile.side_discriminants[&target_side] == discriminant)
        .unwrap()
}

fn find_adjacent_tiles(
    tile: &Tile,
    tiles: &Vec<Tile>,
) -> HashMap<TileID, ((Side, Side), (Orientation, Orientation))> {
    let mut adjacent_tiles: HashMap<TileID, ((Side, Side), (Orientation, Orientation))> =
        HashMap::new();

    for (side_discriminant, &(side, orientation)) in &tile.side_discriminants {
        if let Some(matching_tile) = tiles
            .iter()
            .filter(|other_tile| other_tile.id != tile.id)
            .find(|tile| {
                tile.side_discriminants
                    .keys()
                    .collect::<Vec<&Discriminant>>()
                    .contains(&side_discriminant)
            })
        {
            let existing = adjacent_tiles.get(&matching_tile.id);
            match existing {
                // TODO: figure out how to not repeat this...
                None => {
                    let (matching_tile_side, matching_tile_orientation) =
                        matching_tile.side_discriminants[side_discriminant];

                    adjacent_tiles.insert(
                        matching_tile.id,
                        (
                            (side, matching_tile_side),
                            (orientation, matching_tile_orientation),
                        ),
                    );
                }
                Some((_sides, (Orientation::Reversed, Orientation::Reversed))) => {
                    let (matching_tile_side, matching_tile_orientation) =
                        matching_tile.side_discriminants[side_discriminant];

                    adjacent_tiles.insert(
                        matching_tile.id,
                        (
                            (side, matching_tile_side),
                            (orientation, matching_tile_orientation),
                        ),
                    );
                }
                _ => {}
            }
        }
    }

    adjacent_tiles
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
