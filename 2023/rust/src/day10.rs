use std::collections::HashMap;

type Map = HashMap<Location, Tile>;

type Location = (usize, usize);

#[derive(Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

fn part1_with_start_tile(input: &str, start_tile: Tile) -> usize {
    let (start_location, _bounds, mut map) = parse(input);
    map.insert(start_location, start_tile);

    find_loop(start_location, &map).len() / 2
}

#[must_use]
pub fn part1(input: &str) -> usize {
    part1_with_start_tile(input, Tile::SouthWest)
}

fn part2_with_start_tile(input: &str, start_tile: Tile) -> usize {
    let (start_location, (max_x, max_y), mut map) = parse(input);
    map.insert(start_location, start_tile);

    let loop_path = find_loop(start_location, &map);
    let mut inside = false;
    let mut in_count = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            use Tile::*;
            let in_loop_path = loop_path.contains(&(x, y));

            match map.get(&(x, y)).unwrap() {
                Ground => {
                    if inside {
                        in_count += 1;
                    }
                }
                Vertical | SouthEast | SouthWest => {
                    if in_loop_path {
                        inside = !inside;
                    } else {
                        if inside {
                            in_count += 1;
                        }
                    }
                }
                _ => {
                    if inside && !in_loop_path {
                        in_count += 1;
                    }
                }
            }
        }
    }

    in_count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    part2_with_start_tile(input, Tile::SouthWest)
}

fn find_loop(location: Location, map: &Map) -> Vec<Location> {
    find_path(location, &location, &map, vec![]).unwrap()
}

fn find_path(
    location: Location,
    target_location: &Location,
    map: &Map,
    mut path_so_far: Vec<Location>,
) -> Option<Vec<Location>> {
    use Tile::*;

    path_so_far.push(location);

    let (x, y) = location;
    let next_locations: Vec<Location> = match map.get(&location).unwrap() {
        Vertical => vec![(x, y - 1), (x, y + 1)],
        Horizontal => vec![(x - 1, y), (x + 1, y)],
        NorthEast => vec![(x, y - 1), (x + 1, y)],
        NorthWest => vec![(x, y - 1), (x - 1, y)],
        SouthEast => vec![(x, y + 1), (x + 1, y)],
        SouthWest => vec![(x, y + 1), (x - 1, y)],
        _ => vec![],
    }
    .iter()
    .cloned()
    .collect();

    if next_locations.contains(target_location) && path_so_far.len() > 2 {
        return Some(path_so_far);
    }

    next_locations
        .iter()
        .filter(|location| !path_so_far.contains(location))
        .find_map(|next_location| {
            find_path(*next_location, target_location, map, path_so_far.clone())
        })
}

fn parse(input: &str) -> (Location, (usize, usize), Map) {
    let mut map: HashMap<Location, Tile> = HashMap::new();
    let mut start_location: Location = (0, 0);
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            use Tile::*;

            let tile = match ch {
                '|' => Vertical,
                '-' => Horizontal,
                'L' => NorthEast,
                'J' => NorthWest,
                '7' => SouthWest,
                'F' => SouthEast,
                '.' => Ground,
                'S' => Start,
                invalid => panic!("invalid tile type found: {:?}", invalid),
            };

            if tile == Start {
                start_location = (x, y);
            }

            map.insert((x, y), tile);
        }
    }

    (start_location, (max_x, max_y), map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
.....
.S-7.
.|.|.
.L-J.
....."
    }

    fn input2() -> &'static str {
        "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
    }

    fn input3() -> &'static str {
        "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
    }

    fn input4() -> &'static str {
        "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
    }

    fn input5() -> &'static str {
        "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
    }

    #[test]
    fn part1_example1() {
        assert_eq!(part1_with_start_tile(input(), Tile::SouthEast), 4)
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1_with_start_tile(input2(), Tile::SouthEast), 8)
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2_with_start_tile(input3(), Tile::SouthEast), 4)
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2_with_start_tile(input4(), Tile::SouthEast), 8);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2_with_start_tile(input5(), Tile::SouthWest), 10);
    }
}
