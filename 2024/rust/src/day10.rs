use std::collections::{HashMap, HashSet};

type Position = (usize, usize);
type Height = usize;
type Map = HashMap<Position, Height>;
type Path = Vec<(Position, Height)>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (map, trail_heads) = parse(input);
    let mut score: usize = 0;

    for trail_head in trail_heads {
        let paths = walk(&vec![(trail_head, 0)], vec![], &map);

        //        dbg!(trail_head);
        //        for path in &paths {
        //            print_path(&path);
        //        }

        let end_positions: HashSet<&Position> = paths
            .iter()
            .filter_map(|path| path.last())
            .map(|(position, _height)| position)
            .collect::<HashSet<_, _>>();

        //        dbg!(&end_positions.len());

        score += end_positions.len();
    }

    score
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (map, trail_heads) = parse(input);
    let mut score: usize = 0;

    for trail_head in trail_heads {
        let paths = walk(&vec![(trail_head, 0)], vec![], &map);
        score += paths.len();
    }

    score
}

fn walk(path: &Path, mut visited: Vec<Position>, map: &Map) -> Vec<Path> {
    let mut paths = Vec::new();
    let (current_position, current_height) = *path.last().unwrap();

    for (candidate_position, candidate_height) in
        neighbors(&current_position, map)
    {
        if candidate_height == current_height + 1
            && !visited.contains(&candidate_position)
        {
            visited.push(candidate_position);

            let mut new_path = path.clone();

            new_path.push((candidate_position, candidate_height));

            if candidate_height == 9 {
                paths.push(new_path);
            } else {
                paths.append(&mut walk(&new_path, visited.clone(), map));
            }
        }
    }

    paths
}

fn neighbors(position: &Position, map: &Map) -> Vec<(Position, Height)> {
    let (row, col) = position;
    let neighboring_locations: Vec<Option<Position>> = vec![
        row.checked_sub(1).map(|r| (r, *col)),
        col.checked_sub(1).map(|c| (*row, c)),
        Some((row + 1, *col)),
        Some((*row, col + 1)),
    ];

    neighboring_locations
        .into_iter()
        .filter_map(|maybe_position| {
            maybe_position.and_then(|position| {
                map.get(&position).map(|height| (position, *height))
            })
        })
        .collect()
}

#[allow(dead_code)]
fn print_path(path: &Path) {
    for ((row, col), height) in path {
        print!(" -> {height}({row}, {col})");
    }
    println!();
}

fn parse(input: &str) -> (Map, Vec<Position>) {
    let mut map = HashMap::new();
    let mut trail_heads = Vec::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let height = parse_usize(char);
            map.insert((row, col), height);
            if height == 0 {
                trail_heads.push((row, col));
            }
        }
    }

    (map, trail_heads)
}

fn parse_usize(ch: char) -> usize {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        non_digit_char => {
            panic!("non-digit character seen: {non_digit_char:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 81)
    }
}
