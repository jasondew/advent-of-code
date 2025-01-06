type Lock = Vec<usize>;
type Key = Vec<usize>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (locks, keys) = parse(input);
    let mut count: usize = 0;

    for lock in &locks {
        for key in &keys {
            if fits(lock, key) {
                count += 1;
            }
        }
    }

    count
}

fn fits(lock: &Lock, key: &Key) -> bool {
    lock.iter()
        .zip(key.iter())
        .map(|(l, p)| l + p)
        .all(|s| s <= 7)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.lines().count()
}

fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let groups: Vec<&str> = input.trim_end().split("\n\n").collect();
    let mut locks: Vec<Lock> = Vec::new();
    let mut keys: Vec<Key> = Vec::new();

    for group in groups {
        let mut column_counts: Vec<usize> = vec![0, 0, 0, 0, 0];

        for line in group.lines() {
            for (row, ch) in line.chars().enumerate() {
                if ch == '#' {
                    column_counts[row] += 1;
                }
            }
        }

        if group.starts_with("#####") {
            locks.push(column_counts);
        } else {
            keys.push(column_counts);
        }
    }

    (locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 39);
    }
}
