use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn kind(&self) -> HandKind {
        let mut grouped_count: HashMap<char, usize> = HashMap::new();

        for card in &self.cards {
            grouped_count
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut sorted_counts: Vec<usize> =
            grouped_count.into_values().collect();
        sorted_counts.sort();
        sorted_counts.reverse();

        match &sorted_counts[..] {
            &[5] => HandKind::FiveOfAKind,
            &[4, 1] => HandKind::FourOfAKind,
            &[3, 2] => HandKind::FullHouse,
            &[3, 1, 1] => HandKind::ThreeOfAKind,
            &[2, 2, 1] => HandKind::TwoPair,
            &[2, 1, 1, 1] => HandKind::OnePair,
            &[1, 1, 1, 1, 1] => HandKind::HighCard,
            counts => panic!("invalid state: {:?}", counts),
        }
    }

    fn kind_part2(&self) -> HandKind {
        let mut grouped_count: HashMap<char, usize> = HashMap::new();

        for card in &self.cards {
            grouped_count
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if let Some(joker_count) = grouped_count.remove(&'J') {
            let mut cards_by_count = grouped_count
                .iter()
                .map(|(&card, &count)| (card, count))
                .collect::<Vec<(char, usize)>>();
            cards_by_count.sort_by_key(|(_card, count)| *count);
            cards_by_count.reverse();

            if let Some((highest_count_card, _)) = cards_by_count.get(0) {
                grouped_count
                    .entry(*highest_count_card)
                    .and_modify(|count| *count += joker_count);
            } else {
                grouped_count.insert('J', joker_count);
            }
        }

        let mut sorted_counts: Vec<usize> =
            grouped_count.into_values().collect();
        sorted_counts.sort();
        sorted_counts.reverse();

        // FIXME: need to handle multiple slice sizes
        match &sorted_counts[..] {
            &[5] => HandKind::FiveOfAKind,
            &[4, 1] => HandKind::FourOfAKind,
            &[3, 2] => HandKind::FullHouse,
            &[3, 1, 1] => HandKind::ThreeOfAKind,
            &[2, 2, 1] => HandKind::TwoPair,
            &[2, 1, 1, 1] => HandKind::OnePair,
            &[1, 1, 1, 1, 1] => HandKind::HighCard,
            counts => panic!("invalid state: {:?}", counts),
        }
    }

    fn strength(&self) -> Vec<usize> {
        let card_strengths = self.cards.iter().map(|ch| match ch {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            invalid => panic!("saw invalid card: {:?}", invalid),
        });

        let mut total_strength = vec![self.kind() as usize];
        total_strength.extend(card_strengths);

        total_strength
    }

    fn strength_part2(&self) -> Vec<usize> {
        let card_strengths = self.cards.iter().map(|ch| match ch {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            invalid => panic!("saw invalid card: {:?}", invalid),
        });

        let mut total_strength = vec![self.kind_part2() as usize];
        total_strength.extend(card_strengths);

        total_strength
    }
}

type Bid = usize;

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut hands_and_bids = parse(input);

    hands_and_bids.sort_by(|(hand_a, _), (hand_b, _)| {
        hand_a.strength().partial_cmp(&hand_b.strength()).unwrap()
    });

    hands_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, (_hand, bid))| acc + (rank + 1) * bid)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut hands_and_bids = parse(input);

    hands_and_bids.sort_by(|(hand_a, _), (hand_b, _)| {
        hand_a
            .strength_part2()
            .partial_cmp(&hand_b.strength_part2())
            .unwrap()
    });

    hands_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, (_hand, bid))| acc + (rank + 1) * bid)
}

fn parse(input: &str) -> Vec<(Hand, Bid)> {
    input
        .lines()
        .map(|line| {
            let (cards_string, bid_string) = line.split_once(' ').unwrap();
            (
                Hand {
                    cards: cards_string.chars().collect(),
                },
                bid_string.parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 6440)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 5905)
    }
}
