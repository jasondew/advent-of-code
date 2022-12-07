#[must_use]
pub fn part1(input: &str) -> usize {
    all_different_position::<4>(input)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    all_different_position::<14>(input)
}

fn all_different_position<const WINDOW: usize>(input: &str) -> usize {
    let mut recent: [u8; WINDOW] = [0; WINDOW];

    for (position, ch) in input.chars().enumerate() {
        recent[position % WINDOW] = ch as u8;
        if position >= WINDOW && all_different::<WINDOW>(&recent) {
            return position + 1;
        }
    }

    0
}

fn all_different<const N: usize>(values: &[u8; N]) -> bool {
    let mut duplicate_found: bool = false;

    for i in 0..N - 1 {
        for j in i + 1..N {
            if values[i] == values[j] {
                duplicate_found = true;
                break;
            }
        }
    }

    !duplicate_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
