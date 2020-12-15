use std::collections::HashMap;

#[must_use]
pub fn part1(input: &str) -> usize {
    number_at(input, 2020)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    number_at(input, 30000000)
}

pub fn number_at(input: &str, index: usize) -> usize {
    let mut numbers: Vec<usize> = input
        .trim()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();
    let mut cache: HashMap<usize, usize> = HashMap::new();

    for (index, &number) in numbers.iter().enumerate() {
        if index < numbers.len() - 1 {
            cache.insert(number, index + 1);
        }
    }

    while numbers.len() < index {
        let last: usize = *numbers.last().unwrap();
        let next: usize = match cache.get(&last) {
            Some(&number) => numbers.len() - number,
            None => 0,
        };

        cache.insert(last, numbers.len());
        numbers.push(next);
    }

    *numbers.last().unwrap()
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
