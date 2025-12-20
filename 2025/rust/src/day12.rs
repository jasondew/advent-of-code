use std::collections::HashMap;

type Shape = Vec<Vec<bool>>;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (_shapes, regions) = parse(input);

    //    dbg!(&shapes);
    //    dbg!(&regions);

    regions
        .iter()
        .filter(|region| {
            region.width * region.height
                >= region.shape_counts.iter().sum::<usize>() * 9
        })
        .count()
}

#[must_use]
pub fn part2(_input: &str) -> usize {
    0
}

fn parse(input: &str) -> (HashMap<usize, Shape>, Vec<Region>) {
    let mut shapes = HashMap::new();

    let mut lines = input.split("\n\n").collect::<Vec<&str>>();
    let regions_string = lines.pop().unwrap();
    for line in lines.into_iter() {
        let shape_lines = line.trim().lines().collect::<Vec<&str>>();
        let id = shape_lines[0]
            .trim_end_matches(':')
            .parse::<usize>()
            .unwrap();
        let shape = shape_lines[1..]
            .iter()
            .map(|&l| {
                l.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Unexpected character in shape"),
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        shapes.insert(id, shape);
    }

    let regions = regions_string
        .lines()
        .map(|region_line| {
            let (dimensions_string, shape_counts_string) =
                region_line.split_once(':').unwrap();
            let dimensions: Vec<usize> = dimensions_string
                .split('x')
                .map(|d| d.parse::<usize>().unwrap())
                .collect();
            let shape_counts: Vec<usize> = shape_counts_string
                .trim()
                .split_whitespace()
                .map(|count| count.parse::<usize>().unwrap())
                .collect();

            Region {
                width: dimensions[0],
                height: dimensions[1],
                shape_counts,
            }
        })
        .collect::<Vec<Region>>();

    (shapes, regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }
}
