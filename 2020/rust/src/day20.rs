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

#[derive(Debug)]
struct LockedTile {
    id: TileID,
    cells: Vec<Vec<Cell>>,
    side_discriminants: HashMap<Side, Discriminant>,
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
struct Grid {
    tiles: Vec<Vec<LockedTile>>,
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

        use Orientation::{Normal, Reversed};
        use Side::{Bottom, Left, Right, Top};

        let mut side_discriminants: HashMap<Discriminant, (Side, Orientation)> = HashMap::new();

        let mut top_row: Vec<Cell> = cells[0].clone();
        side_discriminants.insert(side_discriminant(&top_row), (Top, Normal));
        top_row.reverse();
        side_discriminants.insert(side_discriminant(&top_row), (Top, Reversed));

        let mut bottom_row: Vec<Cell> = cells[9].clone();
        side_discriminants.insert(side_discriminant(&bottom_row), (Bottom, Normal));
        bottom_row.reverse();
        side_discriminants.insert(side_discriminant(&bottom_row), (Bottom, Reversed));

        let mut left_row: Vec<Cell> = cells.iter().map(|row| row[0]).collect();
        side_discriminants.insert(side_discriminant(&left_row), (Left, Normal));
        left_row.reverse();
        side_discriminants.insert(side_discriminant(&left_row), (Left, Reversed));

        let mut right_row: Vec<Cell> = cells.iter().map(|row| row[9]).collect();
        side_discriminants.insert(side_discriminant(&right_row), (Right, Normal));
        right_row.reverse();
        side_discriminants.insert(side_discriminant(&right_row), (Right, Reversed));

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

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "ID: {} side discriminants: {:?}",
            self.id, self.side_discriminants
        );
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

impl LockedTile {
    fn from_tile(tile: &Tile, transform: Transform) -> Self {
        Self {
            id: tile.id,
            cells: transform.apply(Self::remove_borders(&tile.cells)),
            side_discriminants: transform.pick(&tile.side_discriminants),
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

    // TODO: figure out how to share this with `Tile`
    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "ID: {} side discriminants: {:?}",
            self.id, self.side_discriminants
        );
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

impl Transform {
    fn from_adjacent_tiles(
        adjacent_tiles: &HashMap<TileID, ((Side, Side), (Orientation, Orientation))>,
    ) -> Self {
        adjacent_tiles
            .iter()
            .map(|(_tile_id, (sides, _orientations))| match sides {
                (Side::Top, Side::Bottom) => Transform::FlipVertically,
                (Side::Bottom, Side::Top) => Transform::Nothing,
                (Side::Bottom, Side::Left) => Transform::Nothing,
                (Side::Bottom, Side::Right) => Transform::Nothing,
                (Side::Right, Side::Left) => Transform::Nothing,
                (Side::Right, Side::Bottom) => Transform::Nothing,
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

    fn pick(
        &self,
        side_discriminants: &HashMap<Discriminant, (Side, Orientation)>,
    ) -> HashMap<Side, Discriminant> {
        let mut map: HashMap<Side, Discriminant> = HashMap::new();

        side_discriminants
            .iter()
            .for_each(|(discriminant, &(side, orientation))| match self {
                Transform::Nothing => {
                    if orientation == Orientation::Normal {
                        map.insert(side, discriminant.clone());
                    }
                }
                Transform::FlipHorizontally => match (side, orientation) {
                    (Side::Top, Orientation::Reversed) => {
                        map.insert(Side::Top, discriminant.clone());
                    }
                    (Side::Bottom, Orientation::Reversed) => {
                        map.insert(Side::Bottom, discriminant.clone());
                    }
                    (Side::Left, Orientation::Normal) => {
                        map.insert(Side::Right, discriminant.clone());
                    }
                    (Side::Right, Orientation::Normal) => {
                        map.insert(Side::Left, discriminant.clone());
                    }
                    _ => {}
                },
                Transform::FlipVertically => match (side, orientation) {
                    (Side::Top, Orientation::Normal) => {
                        map.insert(Side::Bottom, discriminant.clone());
                    }
                    (Side::Bottom, Orientation::Normal) => {
                        map.insert(Side::Top, discriminant.clone());
                    }
                    (Side::Left, Orientation::Reversed) => {
                        map.insert(Side::Left, discriminant.clone());
                    }
                    (Side::Right, Orientation::Reversed) => {
                        map.insert(Side::Right, discriminant.clone());
                    }
                    _ => {}
                },
                Transform::FlipBoth => match (side, orientation) {
                    (Side::Top, Orientation::Reversed) => {
                        map.insert(Side::Bottom, discriminant.clone());
                    }
                    (Side::Bottom, Orientation::Reversed) => {
                        map.insert(Side::Top, discriminant.clone());
                    }
                    (Side::Left, Orientation::Reversed) => {
                        map.insert(Side::Right, discriminant.clone());
                    }
                    (Side::Right, Orientation::Reversed) => {
                        map.insert(Side::Left, discriminant.clone());
                    }
                    _ => {}
                },
                Transform::RotateLeft => match (side, orientation) {
                    (Side::Right, Orientation::Normal) => {
                        map.insert(Side::Top, discriminant.clone());
                    }
                    (Side::Bottom, Orientation::Normal) => {
                        map.insert(Side::Right, discriminant.clone());
                    }
                    (Side::Left, Orientation::Normal) => {
                        map.insert(Side::Bottom, discriminant.clone());
                    }
                    (Side::Top, Orientation::Normal) => {
                        map.insert(Side::Left, discriminant.clone());
                    }
                    _ => {}
                },
                Transform::RotateLeftAndFlipHorizontally => match (side, orientation) {
                    (Side::Right, Orientation::Reversed) => {
                        map.insert(Side::Top, discriminant.clone());
                    }
                    (Side::Bottom, Orientation::Normal) => {
                        map.insert(Side::Left, discriminant.clone());
                    }
                    (Side::Left, Orientation::Reversed) => {
                        map.insert(Side::Bottom, discriminant.clone());
                    }
                    (Side::Top, Orientation::Normal) => {
                        map.insert(Side::Right, discriminant.clone());
                    }
                    _ => {}
                },
                Transform::RotateRight => match (side, orientation) {
                    (Side::Top, Orientation::Normal) => {
                        map.insert(Side::Right, discriminant.clone());
                    }
                    (Side::Right, Orientation::Normal) => {
                        map.insert(Side::Bottom, discriminant.clone());
                    }
                    (Side::Bottom, Orientation::Normal) => {
                        map.insert(Side::Left, discriminant.clone());
                    }
                    (Side::Left, Orientation::Normal) => {
                        map.insert(Side::Top, discriminant.clone());
                    }
                    _ => {}
                },
                Transform::RotateRightAndFlipHorizontally => match (side, orientation) {
                    (Side::Top, Orientation::Normal) => {
                        map.insert(Side::Left, discriminant.clone());
                    }
                    (Side::Right, Orientation::Reversed) => {
                        map.insert(Side::Bottom, discriminant.clone());
                    }
                    (Side::Bottom, Orientation::Normal) => {
                        map.insert(Side::Right, discriminant.clone());
                    }
                    (Side::Left, Orientation::Reversed) => {
                        map.insert(Side::Top, discriminant.clone());
                    }
                    _ => {}
                },
            });
        map
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

impl Grid {
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
    let grid: Grid = assemble_grid(&tiles);

    let mut image: Image = grid.export();
    let mut sea_monsters: Vec<(usize, usize)> = vec![];

    for _rotations in 0..3 {
        image.rotate();
        for _flips in 0..2 {
            image.flip();
            sea_monsters = image.find_sea_monsters();

            if sea_monsters.len() > 0 {
                break;
            }
        }
    }

    image.on_count() - sea_monsters.len() * 15
}

fn assemble_grid(all_tiles: &Vec<Tile>) -> Grid {
    let length: usize = (all_tiles.len() as f32).sqrt() as usize;
    let mut tiles: Vec<Vec<LockedTile>> = (0..length).map(|_| vec![]).collect();
    let mut used_tile_ids: Vec<TileID> = vec![];

    let connections: Connections = Connections::new(&all_tiles);
    let corner_tile: &Tile = find_corner_tiles(&all_tiles, &connections)[1];
    dbg!(&corner_tile.id);

    let adjacent_tiles = find_adjacent_tiles(&corner_tile, &all_tiles);
    let transform = Transform::from_adjacent_tiles(&adjacent_tiles);

    //    dbg!(&all_tiles
    //        .iter()
    //        .map(|tile| (tile.id, &tile.side_discriminants))
    //        .collect::<Vec<(TileID, _)>>());

    used_tile_ids.push(corner_tile.id);
    tiles[0].push(LockedTile::from_tile(&corner_tile, transform));

    for col in 0..length {
        for row in 1..length {
            let previous_locked_tile = &tiles[row - 1][col];
            previous_locked_tile.print();
            let next_locked_tile = next_locked_tile(
                &all_tiles,
                &used_tile_ids,
                Side::Top,
                previous_locked_tile.side_discriminants[&Side::Bottom].clone(),
            );

            next_locked_tile.print();
            used_tile_ids.push(next_locked_tile.id);
            tiles[row].push(next_locked_tile);
        }

        if col < length - 1 {
            dbg!(&tiles[0][col]);
            let mut next_column_locked_tile = next_locked_tile(
                &all_tiles,
                &used_tile_ids,
                Side::Left,
                tiles[0][col].side_discriminants[&Side::Right].clone(),
            );

            if all_tiles
                .iter()
                .filter(|tile| {
                    !(used_tile_ids.contains(&tile.id) || tile.id == next_column_locked_tile.id)
                })
                .any(|tile| {
                    tile.side_discriminants
                        .get(&next_column_locked_tile.side_discriminants[&Side::Top])
                        .is_some()
                })
            {
                next_column_locked_tile = LockedTile::from_tile(
                    &all_tiles
                        .iter()
                        .find(|tile| tile.id == next_column_locked_tile.id)
                        .unwrap(),
                    Transform::FlipVertically,
                );
            }

            next_column_locked_tile.print();
            println!("\n");
            used_tile_ids.push(next_column_locked_tile.id);
            tiles[0].push(next_column_locked_tile);
        }
    }

    Grid { tiles }
}

fn next_locked_tile(
    tiles: &Vec<Tile>,
    used_tile_ids: &Vec<TileID>,
    target_side: Side,
    discriminant: Discriminant,
) -> LockedTile {
    dbg!(&target_side, &discriminant, &used_tile_ids);
    for tile in tiles.iter().filter(|t| !used_tile_ids.contains(&t.id)) {
        if let Some(&(side, orientation)) = tile.side_discriminants.get(&discriminant) {
            let transform = match (target_side, side, orientation) {
                (Side::Top, Side::Bottom, Orientation::Normal)
                | (Side::Bottom, Side::Top, Orientation::Normal) => Transform::FlipVertically,
                (Side::Top, Side::Bottom, Orientation::Reversed)
                | (Side::Bottom, Side::Top, Orientation::Reversed)
                | (Side::Left, Side::Right, Orientation::Reversed)
                | (Side::Right, Side::Left, Orientation::Reversed) => Transform::FlipBoth,
                (Side::Left, Side::Right, Orientation::Normal)
                | (Side::Right, Side::Left, Orientation::Normal) => Transform::FlipHorizontally,
                (Side::Left, Side::Top, Orientation::Reversed) => {
                    Transform::RotateLeftAndFlipHorizontally
                }
                (Side::Top, Side::Right, Orientation::Normal) => Transform::RotateLeft,
                (Side::Top, Side::Left, Orientation::Normal) => Transform::RotateRight,
                (Side::Top, Side::Right, Orientation::Reversed) => {
                    Transform::RotateLeftAndFlipHorizontally
                }
                (Side::Top, Side::Left, Orientation::Reversed) => {
                    Transform::RotateRightAndFlipHorizontally
                }
                (side, other_side, Orientation::Normal) if side == other_side => Transform::Nothing,
                (side, other_side, Orientation::Reversed)
                    if side == other_side && ([Side::Top, Side::Bottom].contains(&side)) =>
                {
                    Transform::FlipHorizontally
                }
                (side, other_side, Orientation::Reversed)
                    if side == other_side && ([Side::Left, Side::Right].contains(&side)) =>
                {
                    Transform::FlipVertically
                }
                _ => panic!("TBD: {:?} {:?} {:?}", target_side, side, orientation),
            };
            return LockedTile::from_tile(&tile, transform);
        }
    }

    panic!("must find a matching tile")
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
