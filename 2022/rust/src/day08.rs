use std::borrow::Borrow;

#[derive(Debug)]
struct HeightMap {
    row_major: Vec<Vec<u8>>,
    col_major: Vec<Vec<u8>>,
}

impl HeightMap {
    fn north_of(&self, row: usize, col: usize) -> &[u8] {
        &self.col_major[col][0..row]
    }

    fn south_of(&self, row: usize, col: usize) -> &[u8] {
        &self.col_major[col][row + 1..]
    }

    fn east_of(&self, row: usize, col: usize) -> &[u8] {
        &self.row_major[row][col + 1..]
    }

    fn west_of(&self, row: usize, col: usize) -> &[u8] {
        &self.row_major[row][0..col]
    }

    fn visible(&self, row: usize, col: usize) -> bool {
        [
            self.north_of(row, col),
            self.south_of(row, col),
            self.east_of(row, col),
            self.west_of(row, col),
        ]
        .iter()
        .any(|direction| {
            direction
                .iter()
                .max()
                .map_or(true, |&max| max < self.row_major[row][col])
        })
    }

    fn visible_count<I>(iterator: I, max_height: u8) -> usize
    where
        I: IntoIterator,
        I::Item: Borrow<u8>,
    {
        let mut count = 0usize;
        let mut blocked = false;

        for height in iterator {
            if !blocked {
                if height.borrow() < &max_height {
                    count += 1;
                } else {
                    count += 1;
                    blocked = true;
                }
            }
        }

        count
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let north_count = Self::visible_count(
            self.north_of(row, col).iter().rev(),
            self.row_major[row][col],
        );
        let south_count = Self::visible_count(
            self.south_of(row, col).iter(),
            self.row_major[row][col],
        );
        let east_count = Self::visible_count(
            self.east_of(row, col).iter(),
            self.row_major[row][col],
        );
        let west_count = Self::visible_count(
            self.west_of(row, col).iter().rev(),
            self.row_major[row][col],
        );

        north_count * south_count * east_count * west_count
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let height_map = parse(input);
    let mut visible_count = 0usize;

    for (row, row_heights) in height_map.row_major.iter().enumerate() {
        for (col, _height) in row_heights.iter().enumerate() {
            if height_map.visible(row, col) {
                visible_count += 1;
            }
        }
    }

    visible_count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let height_map = parse(input);

    height_map
        .row_major
        .iter()
        .enumerate()
        .map(|(row, row_heights)| {
            row_heights
                .iter()
                .enumerate()
                .map(|(col, _height)| height_map.scenic_score(row, col))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn parse(input: &str) -> HeightMap {
    let row_major: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch as u8 - 48).collect())
        .collect();

    let mut col_major: Vec<Vec<u8>> =
        row_major.iter().map(|_| vec![]).collect();

    for i in 0..row_major.len() {
        for row in &row_major {
            col_major[i].push(row[i])
        }
    }

    HeightMap {
        row_major,
        col_major,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
30373
25512
65332
33549
35390\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 21)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 8)
    }
}
