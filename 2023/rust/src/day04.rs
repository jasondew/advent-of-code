use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    number: usize,
    winning_numbers: HashSet<usize>,
    revealed_numbers: HashSet<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let count = self.match_count();
        if count > 0 {
            2_usize.pow((count - 1) as u32)
        } else {
            0
        }
    }

    fn match_count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.revealed_numbers)
            .count()
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let cards = parse(input);

    cards.iter().map(|card| card.points()).sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let cards = parse(input);
    let mut card_counts = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let card_count = card_counts[index];
        let match_count = card.match_count();

        if match_count > 0 {
            for copied_card_index in (index + 1)..=(index + match_count) {
                card_counts[copied_card_index] += card_count;
            }
        }
    }

    card_counts.iter().sum()
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (id_string, numbers) = line.split_once(':').unwrap();
            let number_string =
                id_string.split_whitespace().skip(1).next().unwrap();
            let (winning_numbers_string, revealed_numbers_string) =
                numbers.split_once('|').unwrap();

            let winning_numbers: HashSet<usize> = winning_numbers_string
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let revealed_numbers: HashSet<usize> = revealed_numbers_string
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            Card {
                number: number_string.parse().unwrap(),
                winning_numbers,
                revealed_numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 13)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 30)
    }
}
