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

struct Location {
    x: usize,
    y: usize,
    z: usize,
}

impl Directions {
    fn to_location(&self) -> Location {}
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

#[must_use]
pub fn part1(input: &str) -> usize {
    let tile_directions: Vec<Directions> =
        input.lines().map(|line| line.parse().unwrap()).collect();
    let tile_locations: Vec<Location> = tile_directions
        .iter()
        .map(|directions| directions.to_location())
        .collect();

    dbg!(&tile_locations);

    tile_directions.len()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.len()
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

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
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
            ),
            10
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("quux"), 4)
    }
}
