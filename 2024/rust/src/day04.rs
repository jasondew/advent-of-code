#[must_use]
pub fn part1(input: &str) -> usize {
    let chars: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut count: usize = 0;

    for row in 0..chars.len() {
        for col in 0..chars[0].len() {
            if chars[row][col] == 'X'
                && (right_xmas(&chars, row, col)
                    || left_xmas(&chars, row, col)
                    || up_xmas(&chars, row, col)
                    || down_xmas(&chars, row, col)
                    || up_left_xmas(&chars, row, col)
                    || up_right_xmas(&chars, row, col)
                    || down_left_xmas(&chars, row, col)
                    || down_right_xmas(&chars, row, col))
            {
                count += 1;
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

fn x_mas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
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

fn right_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
    col + 3 < chars[row].len()
        && chars[row][col + 1] == 'M'
        && chars[row][col + 2] == 'A'
        && chars[row][col + 3] == 'S'
}

fn left_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
    col >= 3
        && chars[row][col - 1] == 'M'
        && chars[row][col - 2] == 'A'
        && chars[row][col - 3] == 'S'
}

fn up_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
    row >= 3
        && chars[row - 1][col] == 'M'
        && chars[row - 2][col] == 'A'
        && chars[row - 3][col] == 'S'
}

fn down_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
    row + 3 < chars.len()
        && chars[row + 1][col] == 'M'
        && chars[row + 2][col] == 'A'
        && chars[row + 3][col] == 'S'
}

fn down_left_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
    col >= 3
        && row + 3 < chars.len()
        && chars[row + 1][col - 1] == 'M'
        && chars[row + 2][col - 2] == 'A'
        && chars[row + 3][col - 3] == 'S'
}

fn down_right_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
    col + 3 < chars[row].len()
        && row + 3 < chars.len()
        && chars[row + 1][col + 1] == 'M'
        && chars[row + 2][col + 2] == 'A'
        && chars[row + 3][col + 3] == 'S'
}

fn up_left_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
    row >= 3
        && col >= 3
        && chars[row - 1][col - 1] == 'M'
        && chars[row - 2][col - 2] == 'A'
        && chars[row - 3][col - 3] == 'S'
}

fn up_right_xmas(chars: &[Vec<char>], row: usize, col: usize) -> bool {
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
