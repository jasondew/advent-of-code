use std::collections::HashMap;

type Rules = HashMap<usize, Vec<usize>>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (_must_precede, cant_succeed, updates) = parse(input);

    updates
        .into_iter()
        .filter(|update| is_valid(update, &cant_succeed))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[must_use]
pub fn part2_new(input: &str) -> usize {
    let (must_precede, cant_succeed, updates) = parse(input);
    //    for (key, values) in &must_precede {
    //        dbg!((key, values.len()));
    //    }
    //    let mut keys: Vec<usize> = must_precede.keys().copied().collect();
    //    keys.sort();
    //    dbg!(&keys);
    //    dbg!(&keys.len());

    let ordering = order(&must_precede, &cant_succeed);

    updates
        .into_iter()
        .filter(|update| !is_valid(update, &cant_succeed))
        .map(|update| {
            let corrected_update = correct_with_ordering(&update, &ordering);
            if !is_valid(&corrected_update, &cant_succeed) {
                dbg!(&must_precede);
                dbg!(&corrected_update);
                dbg!(&ordering);
                panic!("omg");
            }
            corrected_update[corrected_update.len() / 2]
        })
        .sum()
}

fn correct_with_ordering(
    update: &Vec<usize>,
    ordering: &Vec<usize>,
) -> Vec<usize> {
    let mut corrected = update.clone();
    corrected.sort_by(|a, b| index(a, ordering).cmp(&index(b, ordering)));

    corrected
}

fn index(value: &usize, ordering: &Vec<usize>) -> Option<usize> {
    ordering.iter().position(|v| v == value)
}

pub fn part2(input: &str) -> usize {
    let (must_precede, cant_succeed, updates) = parse(input);

    updates
        .into_iter()
        .filter(|update| !is_valid(update, &cant_succeed))
        .map(|update| {
            let corrected_update =
                correct(&update, &cant_succeed, &must_precede);
            corrected_update[corrected_update.len() / 2]
        })
        .sum()
}

fn order(must_precede: &Rules, cant_succeed: &Rules) -> Vec<usize> {
    let root = must_precede
        .keys()
        .find(|key| must_precede.values().all(|values| !values.contains(key)))
        .unwrap();

    dbg!(&root);

    let mut map = cant_succeed.clone();
    let mut ordering = vec![];
    let mut nexts = vec![*root];
    let mut new_nexts: Vec<usize> = vec![];

    while !nexts.is_empty() {
        for current in nexts.clone() {
            for (key, values) in map.iter_mut() {
                values.retain_mut(|v| *v != current);
                if values.is_empty() {
                    new_nexts.push(key.clone());
                }
            }

            map.retain(|_key, values| !values.is_empty());
            ordering.push(current);
        }

        nexts = new_nexts.clone();
        new_nexts.clear();
    }

    ordering
}

fn correct(
    update: &Vec<usize>,
    _must_precede: &Rules,
    cant_succeed: &Rules,
) -> Vec<usize> {
    let mut corrected: Vec<usize> = Vec::new();

    for page in update {
        for index in 0..=corrected.len() {
            corrected.insert(index, *page);
            if is_valid(&corrected, cant_succeed) {
                break;
            } else {
                corrected.remove(index);
            }
        }
    }

    corrected
}

fn is_valid(update: &Vec<usize>, rules: &Rules) -> bool {
    let mut verboten_pages: Vec<usize> = Vec::new();

    for page in update {
        if verboten_pages.contains(&page) {
            return false;
        }
        if let Some(new_verboten_pages) = rules.get(&page) {
            verboten_pages.append(&mut new_verboten_pages.clone())
        }
    }

    true
}

fn parse(input: &str) -> (Rules, Rules, Vec<Vec<usize>>) {
    let (rule_strings, update_strings) = input.split_once("\n\n").unwrap();
    let mut must_precede: Rules = HashMap::new();
    let mut cant_succeed: Rules = HashMap::new();

    for line in rule_strings.lines() {
        let (left_string, right_string) = line.split_once("|").unwrap();
        let left = parse_usize(left_string);
        let right = parse_usize(right_string);

        must_precede
            .entry(left)
            .and_modify(|v| v.push(right))
            .or_insert(vec![right]);

        cant_succeed
            .entry(right)
            .and_modify(|v| v.push(left))
            .or_insert(vec![left]);
    }

    let updates: Vec<Vec<usize>> = update_strings
        .lines()
        .map(|line| line.split(",").map(|s| parse_usize(s)).collect())
        .collect();

    (must_precede, cant_succeed, updates)
}

fn parse_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    }

    #[test]
    fn part1_example() {
        let (_must_precede, cant_succeed, _updates) = parse(input());

        assert_eq!(is_valid(&vec![75, 47, 61, 53, 29], &cant_succeed), true);
        assert_eq!(is_valid(&vec![97, 61, 53, 29, 13], &cant_succeed), true);
        assert_eq!(is_valid(&vec![75, 29, 13], &cant_succeed), true);
        assert_eq!(is_valid(&vec![75, 97, 47, 61, 53], &cant_succeed), false);
        assert_eq!(is_valid(&vec![61, 13, 29], &cant_succeed), false);
        assert_eq!(is_valid(&vec![97, 13, 75, 29, 47], &cant_succeed), false);

        assert_eq!(part1(input()), 143);
    }

    #[test]
    fn part2_example() {
        let (must_precede, cant_succeed, _updates) = parse(input());

        assert_eq!(
            correct(&vec![75, 97, 47, 61, 53], &must_precede, &cant_succeed),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            correct(&vec![61, 13, 29], &must_precede, &cant_succeed),
            vec![61, 29, 13]
        );
        assert_eq!(
            correct(&vec![97, 13, 75, 29, 47], &must_precede, &cant_succeed),
            vec![97, 75, 47, 29, 13]
        );
        assert_eq!(part2(input()), 123)
    }

    #[test]
    fn order_example() {
        let (must_precede, cant_succeed, _updates) = parse(input());
        assert_eq!(
            order(&must_precede, &cant_succeed),
            vec![97, 75, 47, 61, 53, 29, 13]
        );
    }

    #[test]
    fn correct_with_ordering_example() {
        let (must_precede, cant_succeed, _updates) = parse(input());
        let ordering = order(&must_precede, &cant_succeed);

        let test_cases: Vec<Vec<usize>> = vec![
            vec![75, 97, 11, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        for test_case in test_cases {
            assert_eq!(
                correct_with_ordering(&test_case, &ordering),
                correct(&test_case, &must_precede, &cant_succeed)
            );
        }
    }
}
