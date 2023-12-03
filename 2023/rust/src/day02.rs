use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    id: usize,
    results: Vec<CubeCount>,
}

#[derive(Debug)]
struct CubeCount {
    reds: usize,
    greens: usize,
    blues: usize,
}

impl CubeCount {
    fn power(&self) -> usize {
        self.reds * self.greens * self.blues
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let bag = CubeCount {
        reds: 12,
        greens: 13,
        blues: 14,
    };

    parse(input)
        .iter()
        .filter(|game| possible(game, &bag))
        .map(|game| game.id)
        .sum()
}

fn possible(game: &Game, bag: &CubeCount) -> bool {
    game.results.iter().all(|result| {
        result.reds <= bag.reds
            && result.greens <= bag.greens
            && result.blues <= bag.blues
    })
}

#[must_use]
pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|game| {
            let mut minimums = CubeCount {
                reds: 0,
                greens: 0,
                blues: 0,
            };

            for result in &game.results {
                minimums.reds = minimums.reds.max(result.reds);
                minimums.greens = minimums.greens.max(result.greens);
                minimums.blues = minimums.blues.max(result.blues);
            }

            minimums.power()
        })
        .sum()
}

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (name, results_string) = line.split_once(':').unwrap();
            let (_game, id_string) = name.split_once(' ').unwrap();
            let id = id_string.parse().unwrap();
            let results: Vec<CubeCount> = results_string
                .split(';')
                .map(|string| {
                    let color_counts: HashMap<&str, usize> = string
                        .split(',')
                        .map(|color_count_string| {
                            let (count_string, color_string) =
                                color_count_string
                                    .trim()
                                    .split_once(' ')
                                    .unwrap();
                            (color_string.trim(), count_string.parse().unwrap())
                        })
                        .collect();

                    CubeCount {
                        reds: *color_counts.get(&"red").unwrap_or(&0),
                        greens: *color_counts.get(&"green").unwrap_or(&0),
                        blues: *color_counts.get(&"blue").unwrap_or(&0),
                    }
                })
                .collect();

            Game { id, results }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 8)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 2286)
    }
}
