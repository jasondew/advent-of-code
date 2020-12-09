#[must_use]
pub fn part1(input: &str) -> usize {
    input.lines().map(|line| seat_id(line)).max().unwrap()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut seat_ids: Vec<usize> = input.lines().map(|line| seat_id(line)).collect();
    let mut last: usize = 0;

    seat_ids.sort_unstable();
    for current in seat_ids {
        if current - last == 2 {
            return current - 1;
        } else {
            last = current
        }
    }

    panic!("something went terribly wrong")
}

fn seat_id(string: &str) -> usize {
    usize::from_str_radix(
        string
            .chars()
            .map(|c| match c {
                'F' | 'L' => '0',
                'B' | 'R' => '1',
                _ => panic!("found unexpected character {:?}", c),
            })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_id_examples() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n"), 820);
    }

    #[test]
    fn part2_example() {}
}
