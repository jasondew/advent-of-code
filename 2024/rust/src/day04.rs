#[must_use]
pub fn part1(input: &str) -> usize {
    let chars: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut count: usize = 0;

    for row in 0..chars.len() {
        for col in 0..chars[0].len() {
            if chars[row][col] == 'X' {
                if right_xmas(&chars, row, col) {
                    //                    dbg!(("right", row, col));
                    count += 1
                };
                if left_xmas(&chars, row, col) {
                    //                    dbg!(("left", row, col));
                    count += 1
                };
                if up_xmas(&chars, row, col) {
                    //                    dbg!(("up", row, col));
                    count += 1
                };
                if down_xmas(&chars, row, col) {
                    //                    dbg!(("down", row, col));
                    count += 1
                };
                if up_left_xmas(&chars, row, col) {
                    //                    dbg!(("up left", row, col));
                    count += 1
                };
                if up_right_xmas(&chars, row, col) {
                    //                    dbg!(("up right", row, col));
                    count += 1
                };
                if down_left_xmas(&chars, row, col) {
                    //                    dbg!(("down left", row, col));
                    count += 1
                };
                if down_right_xmas(&chars, row, col) {
                    //                    dbg!(("down right", row, col));
                    count += 1
                };
            }
        }
    }

    count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let chars: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut count: usize = 0;

    for row in 0..chars.len() {
        for col in 0..chars[0].len() {
            if chars[row][col] == 'A' && x_mas(&chars, row, col) {
                //                dbg!(("x_mas", row, col));
                count += 1;
            }
        }
    }

    count
}

fn x_mas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    row >= 1
        && row + 1 < chars.len()
        && col >= 1
        && col + 1 < chars[row].len()
        && ((chars[row - 1][col - 1] == 'M' && chars[row + 1][col + 1] == 'S')
            || (chars[row - 1][col - 1] == 'S'
                && chars[row + 1][col + 1] == 'M'))
        && ((chars[row - 1][col + 1] == 'M' && chars[row + 1][col - 1] == 'S')
            || (chars[row - 1][col + 1] == 'S'
                && chars[row + 1][col - 1] == 'M'))
}

fn right_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    col + 3 < chars[row].len()
        && chars[row][col + 1] == 'M'
        && chars[row][col + 2] == 'A'
        && chars[row][col + 3] == 'S'
}

fn left_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    col >= 3
        && chars[row][col - 1] == 'M'
        && chars[row][col - 2] == 'A'
        && chars[row][col - 3] == 'S'
}

fn up_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    row >= 3
        && chars[row - 1][col] == 'M'
        && chars[row - 2][col] == 'A'
        && chars[row - 3][col] == 'S'
}

fn down_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    row + 3 < chars.len()
        && chars[row + 1][col] == 'M'
        && chars[row + 2][col] == 'A'
        && chars[row + 3][col] == 'S'
}

fn down_left_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    col >= 3
        && row + 3 < chars.len()
        && chars[row + 1][col - 1] == 'M'
        && chars[row + 2][col - 2] == 'A'
        && chars[row + 3][col - 3] == 'S'
}

fn down_right_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    col + 3 < chars[row].len()
        && row + 3 < chars.len()
        && chars[row + 1][col + 1] == 'M'
        && chars[row + 2][col + 2] == 'A'
        && chars[row + 3][col + 3] == 'S'
}

fn up_left_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    row >= 3
        && col >= 3
        && chars[row - 1][col - 1] == 'M'
        && chars[row - 2][col - 2] == 'A'
        && chars[row - 3][col - 3] == 'S'
}

fn up_right_xmas(chars: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    row >= 3
        && col + 3 < chars[row].len()
        && chars[row - 1][col + 1] == 'M'
        && chars[row - 2][col + 2] == 'A'
        && chars[row - 3][col + 3] == 'S'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 18);

        assert_eq!(right_xmas(&vec!["XMASMMM".chars().collect()], 0, 0), true);
        assert_eq!(right_xmas(&vec!["MMMXMAS".chars().collect()], 0, 3), true);

        assert_eq!(left_xmas(&vec!["SAMXMMM".chars().collect()], 0, 3), true);
        assert_eq!(left_xmas(&vec!["MMMSAMX".chars().collect()], 0, 6), true);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 9)
    }
}
