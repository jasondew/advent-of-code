use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<Range<usize>>,
}

impl Rule {
    fn contains(&self, value: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let name_and_ranges: Vec<&str> = input.split(": ").collect();
        let name = name_and_ranges[0].to_owned();
        let ranges: Vec<Range<usize>> = name_and_ranges[1]
            .split(" or ")
            .map(|range| {
                let parts: Vec<usize> = range.split('-').map(|x| x.parse().unwrap()).collect();
                Range {
                    start: parts[0],
                    end: parts[1] + 1,
                }
            })
            .collect();
        Ok(Self { name, ranges })
    }
}

#[derive(Debug)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn at(&self, index: usize) -> usize {
        self.0[index]
    }

    fn is_valid(&self, rules: &[Rule]) -> bool {
        !self
            .0
            .iter()
            .any(|&value| !rules.iter().any(|rule| rule.contains(value)))
    }

    fn invalid_sum(&self, rules: &[Rule]) -> usize {
        self.0
            .iter()
            .filter(|&&value| !rules.iter().any(|rule| rule.contains(value)))
            .sum()
    }
}

impl FromStr for Ticket {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values = input
            .split(',')
            .map(|value| value.parse().unwrap())
            .collect();
        Ok(Ticket(values))
    }
}

#[derive(Debug)]
struct Notes {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Notes {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let groups: Vec<&str> = input.trim().split("\n\n").collect();
        let rules: Vec<Rule> = groups[0]
            .lines()
            .map(|rule| rule.parse().unwrap())
            .collect();
        let my_ticket: Ticket = groups[1].lines().nth(1).unwrap().parse().unwrap();
        let nearby_tickets: Vec<Ticket> = groups[2]
            .lines()
            .skip(1)
            .map(|ticket| ticket.parse().unwrap())
            .collect();
        Ok(Self {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }
}

#[derive(Debug)]
enum Mapping {
    Definitely(usize),
    Possibilities(Vec<usize>),
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let notes: Notes = input.parse().unwrap();
    notes
        .nearby_tickets
        .iter()
        .map(|ticket| ticket.invalid_sum(&notes.rules))
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    use Mapping::{Definitely, Possibilities};

    let notes: Notes = input.parse().unwrap();
    let rule_count: usize = notes.rules.len();

    let valid_tickets: Vec<&Ticket> = notes
        .nearby_tickets
        .iter()
        .filter(|ticket| ticket.is_valid(&notes.rules))
        .collect();

    let mut mappings: HashMap<String, Mapping> = HashMap::new();
    let mut assigned_indexes = vec![];

    for rule in notes.rules {
        let possible_indexes: Vec<usize> = (0..rule_count)
            .filter(|index| {
                !assigned_indexes.contains(index)
                    && valid_tickets
                        .iter()
                        .all(|ticket| rule.contains(ticket.at(*index)))
            })
            .collect();
        let mapping = match possible_indexes[..] {
            [index] => {
                assigned_indexes.push(index);
                Definitely(index)
            }
            _ => Possibilities(possible_indexes),
        };

        mappings.insert(rule.name, mapping);
    }

    loop {
        for (_rule_name, mapping) in mappings.iter_mut() {
            match mapping {
                Definitely(_) => {}
                Possibilities(possible_indexes) => {
                    possible_indexes.retain(|x| !assigned_indexes.contains(x));
                    if possible_indexes.len() == 1 {
                        let index = *possible_indexes.first().unwrap();

                        assigned_indexes.push(index);
                        *mapping = Definitely(index)
                    }
                }
            }
        }

        if assigned_indexes.len() == mappings.len() {
            break;
        }
    }

    let mut answer: usize = 1;

    for (rule, mapping) in mappings {
        if rule.starts_with("departure") {
            match mapping {
                Definitely(index) => answer *= notes.my_ticket.at(index),
                Possibilities(_) => panic!(),
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "\
class: 1-3 or 5-7\n\
row: 6-11 or 33-44\n\
seat: 13-40 or 45-50\n\
\n\
your ticket:\n\
7,1,14\n\
\n\
nearby tickets:\n\
7,3,47\n\
40,4,50\n\
55,2,20\n\
38,6,12\n"
            ),
            71
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "\
departure class: 0-1 or 4-19\n\
row: 0-5 or 8-19\n\
departure lane: 0-13 or 16-19\n\
\n\
your ticket:\n\
11,12,13\n\
\n\
nearby tickets:\n\
3,9,18\n\
15,1,5\n\
5,14,9\n"
            ),
            12 * 13
        )
    }
}
