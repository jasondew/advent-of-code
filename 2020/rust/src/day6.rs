use std::collections::{HashMap, HashSet};

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    let mut total: usize = 0;

    for line in input.lines() {
        if line == "" {
            total += set.len();
            set = HashSet::new();
        } else {
            for c in line.trim().chars() {
                set.insert(c);
            }
        }
    }

    total + set.len()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut total: usize = 0;
    let mut group_size: usize = 0;

    for line in input.lines() {
        if line == "" {
            total += common_count(&map, group_size);
            map = HashMap::new();
            group_size = 0;
        } else {
            for c in line.trim().chars() {
                let new_count = map.get(&c).map_or(1, |count| count + 1);
                map.insert(c, new_count);
            }
            group_size += 1;
        }
    }

    total + common_count(&map, group_size)
}

fn common_count(map: &HashMap<char, usize>, group_size: usize) -> usize {
    map.iter()
        .filter(|(_c, &count)| count == group_size)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n"), 11)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n"), 6)
    }
}
