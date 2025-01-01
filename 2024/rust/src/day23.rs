use std::collections::{HashMap, HashSet};

#[must_use]
pub fn part1(input: &str) -> usize {
    let connections = parse(input);
    let mut groups: HashSet<Vec<&str>> = HashSet::new();

    for (computer, connected_computers) in &connections {
        for (a, b) in connected_pairs(connected_computers, &connections) {
            let mut group = vec![computer, a, b];
            group.sort();
            groups.insert(group);
        }
    }

    groups
        .iter()
        .filter(|group| group.iter().any(|c| c.starts_with('t')))
        .count()
}

fn connected_pairs<'a>(
    computers: &HashSet<&'a str>,
    connections: &HashMap<&'a str, HashSet<&'a str>>,
) -> Vec<(&'a str, &'a str)> {
    let mut pairs: Vec<(&str, &str)> = Vec::new();

    for computer_a in computers {
        for computer_b in computers {
            if is_connected(computer_a, computer_b, connections) {
                pairs.push((computer_a, computer_b));
            }
        }
    }

    pairs
}

fn is_connected(
    computer_a: &str,
    computer_b: &str,
    connections: &HashMap<&str, HashSet<&str>>,
) -> bool {
    connections
        .get(computer_a)
        .map_or(false, |cs| cs.contains(&computer_b))
}

#[must_use]
pub fn part2(input: &str) -> String {
    let connections = parse(input);

    let mut groups: HashSet<Vec<&str>> = HashSet::new();

    for (computer, connected_computers) in &connections {
        for (a, b) in connected_pairs(connected_computers, &connections) {
            let mut group = vec![computer, a, b];
            group.sort();
            groups.insert(group);
        }
    }

    let mut sets: Vec<HashSet<&str>> = groups
        .into_iter()
        .map(|group| group.into_iter().collect())
        .collect();

    for set in &mut sets {
        for (computer, connected_computers) in &connections {
            if set.intersection(connected_computers).count() == set.len() {
                set.insert(computer);
            }
        }
    }

    let mut largest_set: Vec<&str> = sets
        .into_iter()
        .max_by_key(|set| set.len())
        .unwrap()
        .into_iter()
        .collect();

    largest_set.sort();

    largest_set.into_iter().collect::<Vec<&str>>().join(",")
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();

        map.entry(left).or_insert(HashSet::new()).insert(right);
        map.entry(right).or_insert(HashSet::new()).insert(left);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), "co,de,ka,ta".to_owned());
    }
}
