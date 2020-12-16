#[must_use]
pub fn part1(input: &str) -> u32 {
    number_at(input, 2020)
}

#[must_use]
pub fn part2(input: &str) -> u32 {
    number_at(input, 30000000)
}

#[allow(clippy::cast_possible_truncation)]
fn number_at(input: &str, index: u32) -> u32 {
    let numbers: Vec<u32> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut cache: Vec<Option<u32>> = vec![None; index as usize];
    for (index, &number) in numbers.iter().enumerate() {
        cache[number as usize] = Some((index + 1) as u32)
    }

    let mut count: u32 = numbers.len() as u32;
    let mut current: u32 = *numbers.last().unwrap();

    while count < index {
        let next: u32 = match cache[current as usize] {
            Some(number) => count - number,
            None => 0,
        };

        cache[current as usize] = Some(count);
        current = next;
        count += 1;
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("0,3,6"), 436);
        assert_eq!(part1("1,3,2"), 1);
        assert_eq!(part1("2,1,3"), 10);
        assert_eq!(part1("1,2,3"), 27);
        assert_eq!(part1("2,3,1"), 78);
        assert_eq!(part1("3,2,1"), 438);
        assert_eq!(part1("3,1,2"), 1836);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("0,3,6"), 175594);
        //        assert_eq!(part2("1,3,2"), 2578);
        //        assert_eq!(part2("2,1,3"), 3544142);
        //        assert_eq!(part2("1,2,3"), 261214);
        //        assert_eq!(part2("2,3,1"), 6895259);
        //        assert_eq!(part2("3,2,1"), 18);
        //        assert_eq!(part2("3,1,2"), 362);
    }
}
