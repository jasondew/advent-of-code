use core::cmp;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Direction {
    Northeast,
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
}

#[derive(Debug, PartialEq)]
struct Directions(Vec<Direction>);

type Coordinate = i16;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl Directions {
    fn to_location(&self) -> Location {
        let mut x: Coordinate = 0;
        let mut y: Coordinate = 0;
        let mut z: Coordinate = 0;

        for direction in &self.0 {
            match direction {
                Direction::Northeast => {
                    z -= 1;
                    x += 1;
                }
                Direction::East => {
                    x += 1;
                    y -= 1;
                }
                Direction::Southeast => {
                    y -= 1;
                    z += 1;
                }
                Direction::Southwest => {
                    x -= 1;
                    z += 1;
                }
                Direction::West => {
                    x -= 1;
                    y += 1;
                }
                Direction::Northwest => {
                    y += 1;
                    z -= 1;
                }
            }
        }

        Location { x, y, z }
    }
}

impl std::str::FromStr for Directions {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut chars = input.chars();
        let mut directions: Vec<Direction> = Vec::new();

        while let Some(char) = chars.next() {
            match char {
                'e' => directions.push(Direction::East),
                's' => match chars.next().unwrap() {
                    'e' => directions.push(Direction::Southeast),
                    'w' => directions.push(Direction::Southwest),
                    _ => panic!("unexpected char"),
                },
                'w' => directions.push(Direction::West),
                'n' => match chars.next().unwrap() {
                    'e' => directions.push(Direction::Northeast),
                    'w' => directions.push(Direction::Northwest),
                    _ => panic!("unexpected char"),
                },
                _ => panic!("unexpected char"),
            }
        }

        Ok(Directions(directions))
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
enum NeighborCount {
    Zero,
    One,
    Two,
    MoreThanTwo,
}

#[derive(Debug)]
struct Map {
    data: HashMap<Location, Color>,
    x_min: Coordinate,
    x_max: Coordinate,
    y_min: Coordinate,
    y_max: Coordinate,
    z_min: Coordinate,
    z_max: Coordinate,
}

impl Map {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            z_min: 0,
            z_max: 0,
        }
    }

    fn from_directions(tile_directions: &[Directions]) -> Self {
        let mut map: Map = Self::new();

        for directions in tile_directions {
            map.flip(directions.to_location())
        }

        map
    }

    fn flip(&mut self, location: Location) {
        self.x_min = cmp::min(location.x, self.x_min);
        self.x_max = cmp::max(location.x, self.x_max);
        self.y_min = cmp::min(location.y, self.y_min);
        self.y_max = cmp::max(location.y, self.y_max);
        self.z_min = cmp::min(location.z, self.z_min);
        self.z_max = cmp::max(location.z, self.z_max);

        let color = self
            .data
            .get(&location)
            .map_or(Color::Black, |color| match color {
                Color::Black => Color::White,
                Color::White => Color::Black,
            });
        self.data.insert(location, color);
    }

    fn neighbor_count(&self, location: &Location) -> NeighborCount {
        let mut count: usize = 0;
        let neighbor_locations = vec![
            Location {
                x: location.x + 1,
                y: location.y - 1,
                z: location.z,
            },
            Location {
                x: location.x + 1,
                y: location.y,
                z: location.z - 1,
            },
            Location {
                x: location.x,
                y: location.y + 1,
                z: location.z - 1,
            },
            Location {
                x: location.x - 1,
                y: location.y + 1,
                z: location.z,
            },
            Location {
                x: location.x - 1,
                y: location.y,
                z: location.z + 1,
            },
            Location {
                x: location.x,
                y: location.y - 1,
                z: location.z + 1,
            },
        ];

        for neighbor_location in neighbor_locations {
            if self.data.get(&neighbor_location) == Some(&Color::Black) {
                count += 1;
                if count > 2 {
                    break;
                }
            }
        }

        match count {
            0 => NeighborCount::Zero,
            1 => NeighborCount::One,
            2 => NeighborCount::Two,
            _ => NeighborCount::MoreThanTwo,
        }
    }

    fn step(&mut self) {
        let mut locations_to_flip: Vec<Location> = Vec::new();

        for x in (self.x_min - 2)..(self.x_max + 3) {
            for y in (self.y_min - 2)..(self.y_max + 3) {
                let location: Location = Location { x, y, z: -x - y };
                match self.data.get(&location) {
                    Some(Color::Black) => match self.neighbor_count(&location) {
                        NeighborCount::Zero | NeighborCount::MoreThanTwo => {
                            locations_to_flip.push(location)
                        }
                        _ => {}
                    },
                    _white => {
                        if self.neighbor_count(&location) == NeighborCount::Two {
                            locations_to_flip.push(location)
                        }
                    }
                }
            }
        }

        for location in locations_to_flip {
            self.flip(location)
        }
    }

    fn black_count(&self) -> usize {
        self.data
            .values()
            .filter(|&color| color == &Color::Black)
            .count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        let from: Coordinate = -10;
        let to: Coordinate = 10;
        for y in from..to {
            for x in from..to {
                let location: Location = Location { x, y, z: -x - y };

                if x == from && location.z % 2 == 1 {
                    print!(" ");
                }

                match self.data.get(&location) {
                    Some(Color::Black) => print!("# "),
                    _ => print!(". "),
                }
            }
            println!();
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let tile_directions: Vec<Directions> =
        input.lines().map(|line| line.parse().unwrap()).collect();
    let map: Map = Map::from_directions(&tile_directions);

    map.black_count()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let tile_directions: Vec<Directions> =
        input.lines().map(|line| line.parse().unwrap()).collect();
    let mut map: Map = Map::from_directions(&tile_directions);

    for iteration in 1..101 {
        map.step();
        println!("Day {}: {}", iteration, map.black_count());
    }

    map.print();
    map.black_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        use Direction::{East, Northeast, Southeast};
        let directions: Directions = "esenee".parse().unwrap();
        assert_eq!(directions.0, vec![East, Southeast, Northeast, East]);
    }

    fn input() -> &'static str {
        "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 10)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 2208)
    }
}
