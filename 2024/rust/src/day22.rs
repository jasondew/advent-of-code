use std::{collections::HashMap, collections::HashSet, ops::BitXor};

type ID = [isize; 4];

#[must_use]
pub fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|x| evolve_times(x, 2000))
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut maps: HashMap<usize, HashMap<ID, usize>> = HashMap::new();

    for x in parse(input).into_iter() {
        let mut map: HashMap<ID, usize> = HashMap::new();

        for window in evolve_prices_with_derivatives(x, 2000).windows(4) {
            let id = [window[0].1, window[1].1, window[2].1, window[3].1];
            let price = window[3].0;

            if map.get(&id).is_none() {
                map.insert(id, price);
            }
        }

        maps.insert(x, map);
    }

    let mut all_ids: HashSet<ID> = HashSet::new();

    for (_, map) in &maps {
        for id in map.keys() {
            all_ids.insert(id.clone());
        }
    }

    let mut best_sum: usize = 0;

    for id in all_ids {
        let sum = maps
            .iter()
            .map(|(_, map)| map.get(&id).map_or(0, |price| *price))
            .sum();

        if sum > best_sum {
            best_sum = sum;
        }
    }

    best_sum
}

fn evolve_times(mut x: usize, iterations: usize) -> usize {
    for _ in 0..iterations {
        x = evolve(x)
    }

    x
}

fn evolve_prices_with_derivatives(
    mut x: usize,
    iterations: usize,
) -> Vec<(usize, isize)> {
    (0..iterations)
        .map(|_| {
            let new_x = evolve(x);
            let new_price = new_x % 10;
            let price_delta = new_price as isize - (x % 10) as isize;

            x = new_x;

            (new_price, price_delta)
        })
        .collect()
}

#[allow(dead_code)]
fn evolve_iterations(mut x: usize, iterations: usize) -> Vec<usize> {
    (0..iterations)
        .map(|_| {
            x = evolve(x);
            x.clone()
        })
        .collect()
}

fn evolve(mut x: usize) -> usize {
    x = prune(mix(x, x * 64));
    x = prune(mix(x, x / 32));
    prune(mix(x, x * 2048))
}

fn mix(x: usize, y: usize) -> usize {
    x.bitxor(y)
}

fn prune(x: usize) -> usize {
    x % 16777216
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim_end()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
1
10
100
2024\n"
    }

    fn part2_input() -> &'static str {
        "\
1
2
3
2024\n"
    }

    #[test]
    fn examples() {
        assert_eq!(evolve(123), 15887950);
        assert_eq!(
            evolve_iterations(123, 10),
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156,
                11100544, 12249484, 7753432, 5908254
            ]
        );
        assert_eq!(evolve_times(1, 2000), 8685429);
        assert_eq!(evolve_times(10, 2000), 4700978);
        assert_eq!(evolve_times(100, 2000), 15273692);
        assert_eq!(evolve_times(2024, 2000), 8667524);
        assert_eq!(
            evolve_prices_with_derivatives(123, 9),
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2)
            ]
        );
        assert_eq!(evolve_prices_with_derivatives(123, 2000).len(), 2000);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(part2_input()), 23);
    }
}
