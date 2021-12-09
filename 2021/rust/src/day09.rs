use std::collections::HashMap;

#[must_use]
pub fn part1(input: &str) -> usize {
    let height_map: Vec<Vec<u8>> = parse(input);
    let mut low_points: Vec<u8> = vec![];

    for (y, row) in height_map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if neighbors(&height_map, x, y)
                .iter()
                .all(|&neighbor| cell < neighbor)
            {
                low_points.push(cell)
            }
        }
    }

    low_points.iter().map(|value| (value + 1) as usize).sum()
}

fn neighbors<T>(map: &Vec<Vec<T>>, x: usize, y: usize) -> Vec<T>
where
    T: Copy,
{
    let mut cells: Vec<T> = vec![];

    if x > 0 {
        cells.push(map[y][x - 1]);
    }
    if x < map[0].len() - 1 {
        cells.push(map[y][x + 1]);
    }
    if y > 0 {
        cells.push(map[y - 1][x]);
    }
    if y < map.len() - 1 {
        cells.push(map[y + 1][x]);
    }

    cells
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Unknown,
    Wall,
    Basin(usize),
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let height_map: Vec<Vec<u8>> = parse(input);
    let mut basin_map: Vec<Vec<Cell>> =
        vec![vec![Cell::Unknown; height_map[0].len()]; height_map.len()];
    let mut basin_count: usize = 0;
    let mut peers: HashMap<usize, usize> = HashMap::new();

    for (y, row) in height_map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            use Cell::*;

            if x == 0 && y == 0 {
                basin_count += 1;
                basin_map[0][0] = Basin(basin_count);
            } else if cell == 9 {
                basin_map[y][x] = Wall;
            } else {
                match adjacent_basin_numbers(&basin_map, x, y)[..] {
                    [] => {
                        basin_count += 1;
                        basin_map[y][x] = Basin(basin_count);
                    }
                    [basin_number] => {
                        basin_map[y][x] = Basin(basin_number);
                    }
                    [a, b] => {
                        if a == b {
                            basin_map[y][x] = Basin(a);
                        } else {
                            let basin_number = a.min(b);
                            let alias = a.max(b);
                            peers.insert(alias, basin_number);

                            basin_map[y][x] = Basin(basin_number);
                        }
                    }
                    _ => panic!("should never get here"),
                }
            }
        }
    }

    //    print_map(&basin_map);

    let mut basin_counts: HashMap<usize, usize> = HashMap::new();
    for row in basin_map {
        for cell in row {
            match cell {
                Cell::Basin(number) => {
                    let mut root_number = number;
                    while let Some(&other_number) = peers.get(&root_number) {
                        root_number = other_number;
                    }
                    *basin_counts.entry(root_number).or_insert(0) += 1
                }
                _ => {}
            }
        }
    }

    let mut counts: Vec<usize> = basin_counts.into_values().collect();
    counts.sort();
    counts.iter().rev().take(3).product()
}

fn adjacent_basin_numbers(map: &Vec<Vec<Cell>>, x: usize, y: usize) -> Vec<usize> {
    neighbors(map, x, y)
        .iter()
        .filter_map(|cell| match cell {
            Cell::Basin(b) => Some(b),
            _ => None,
        })
        .cloned()
        .collect()
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Cell>>) {
    for row in map.iter() {
        for &cell in row.iter() {
            match cell {
                Cell::Basin(b) => print!("{:^3}", b),
                Cell::Wall => print!("###"),
                Cell::Unknown => print!("???"),
            }
        }
        println!();
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(|digit| (digit as u8) - 48).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
2199943210
3987894921
9856789892
8767896789
9899965678\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 15)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 1134)
    }
}
