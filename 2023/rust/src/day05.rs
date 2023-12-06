use std::{collections::HashSet, ops::Range};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn seed_ranges(&self) -> Vec<Range<usize>> {
        self.seeds
            .chunks(2)
            .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .collect()
    }
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    from: String,
    #[allow(dead_code)]
    to: String,
    ranges: Vec<(Range<usize>, Range<usize>)>,
}

impl Map {
    fn convert(&self, value: usize) -> usize {
        for (source_range, destination_range) in self.ranges.iter() {
            if source_range.contains(&value) {
                return destination_range.start + (value - source_range.start);
            }
        }

        value
    }

    fn invert(&self, value: usize) -> usize {
        for (source_range, destination_range) in self.ranges.iter() {
            if destination_range.contains(&value) {
                return source_range.start + (value - destination_range.start);
            }
        }
        value
    }

    fn cutpoints(&self) -> Vec<usize> {
        let mut values: Vec<usize> = self
            .ranges
            .iter()
            .flat_map(|(source, _)| [source.start, source.end])
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect::<Vec<usize>>()
            .into_iter()
            .flat_map(|point| {
                if point > 0 {
                    [point, point - 1]
                } else {
                    [point, point]
                }
            })
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect();
        values.sort();
        values
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let almanac = parse(input);

    almanac
        .seeds
        .into_iter()
        .map(|seed| {
            almanac
                .maps
                .iter()
                .fold(seed, |value, map| map.convert(value))
        })
        .min()
        .unwrap()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let almanac = parse(input);

    let mut possible_values: Vec<usize> = almanac
        .maps
        .iter()
        .rev()
        .fold(vec![], |values, map| {
            let mut inverted_values = values
                .iter()
                .map(|v| map.invert(*v))
                .collect::<Vec<usize>>();

            inverted_values.extend(map.cutpoints());
            inverted_values
        })
        .into_iter()
        .collect::<HashSet<usize>>()
        .into_iter()
        .filter(|value| {
            almanac
                .seed_ranges()
                .iter()
                .any(|range| range.contains(value))
        })
        .collect();
    possible_values.sort();

    possible_values
        .into_iter()
        .map(|seed| {
            almanac
                .maps
                .iter()
                .fold(seed, |value, map| map.convert(value))
        })
        .min()
        .unwrap()
}

fn parse(input: &str) -> Almanac {
    let mut group_iter = input.split("\n\n");

    let seeds = group_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed_string| seed_string.parse().unwrap())
        .collect();
    let maps = group_iter
        .map(|group| {
            let mut line_iter = group.lines();
            let mut word_iter = line_iter
                .next()
                .unwrap()
                .split_once(' ')
                .unwrap()
                .0
                .split('-')
                .take(3);
            let from = word_iter.next().unwrap();
            let to = word_iter.skip(1).next().unwrap();
            let ranges = line_iter
                .map(|line| {
                    let mut number_iter = line.split_whitespace();
                    let destination_start =
                        number_iter.next().unwrap().parse().unwrap();
                    let source_start =
                        number_iter.next().unwrap().parse().unwrap();
                    let length: usize =
                        number_iter.next().unwrap().parse().unwrap();

                    (
                        source_start..(source_start + length),
                        destination_start..(destination_start + length),
                    )
                })
                .collect();

            Map {
                from: from.to_owned(),
                to: to.to_owned(),
                ranges,
            }
        })
        .collect();

    Almanac { seeds, maps }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 35)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 46)
    }
}
