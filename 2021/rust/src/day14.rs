use std::collections::HashMap;

type Rules = HashMap<(char, char), char>;
type Polymer = Vec<char>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (polymer, rules) = parse(input);
    max_element_count_difference(&polymer, &rules, 10)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (polymer, rules) = parse(input);
    max_element_count_difference(&polymer, &rules, 40)
}

fn max_element_count_difference(polymer: &Polymer, rules: &Rules, steps: usize) -> usize {
    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    let mut new: char;

    for (a, b) in polymer.iter().zip(polymer.iter().skip(1)) {
        *pair_counts.entry((*a, *b)).or_insert(0) += 1;
    }

    for _step in 1..=steps {
        for ((a, b), count) in pair_counts.clone().into_iter() {
            new = *rules.get(&(a, b)).unwrap();
            *pair_counts.entry((a, new)).or_insert(0) += count;
            *pair_counts.entry((new, b)).or_insert(0) += count;
            *pair_counts.entry((a, b)).or_insert(0) -= count;
        }
    }

    let mut counts: HashMap<char, usize> = HashMap::new();

    for ((_a, b), count) in pair_counts.into_iter() {
        *counts.entry(b).or_insert(0) += count;
    }

    let largest_count = counts.values().max().unwrap();
    let smallest_count = counts.values().min().unwrap();

    largest_count - smallest_count
}

#[allow(dead_code)]
fn step(rules: &Rules, polymer: Polymer) -> Polymer {
    let mut next_polymer: Vec<char> = vec![*polymer.get(0).unwrap()];

    for (a, b) in polymer.iter().zip(polymer.iter().skip(1)) {
        next_polymer.push(*rules.get(&(*a, *b)).unwrap());
        next_polymer.push(*b);
    }

    next_polymer
}

#[allow(dead_code)]
fn print(polymer: &Polymer) {
    println!("{}", &polymer.iter().cloned().collect::<String>());
}

fn parse(input: &str) -> (Polymer, Rules) {
    let (template, rule_lines) = input.trim_end().split_once("\n\n").unwrap();
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    for line in rule_lines.lines() {
        let (pair, element) = line.split_once(" -> ").unwrap();
        let mut pair_elements = pair.chars();
        let a = pair_elements.next().unwrap();
        let b = pair_elements.next().unwrap();
        let c = element.chars().next().unwrap();

        rules.insert((a, b), c);
    }

    (template.chars().collect(), rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C\n"
    }

    #[test]
    fn examples() {
        assert_eq!(part1(input()), 1588);
        assert_eq!(part2(input()), 2188189693529);
    }
}
