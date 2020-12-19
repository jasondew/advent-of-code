use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Rule {
    Sequence(Vec<Rule>),
    Either(Vec<Rule>, Vec<Rule>),
    A,
    B,
}

#[derive(Debug)]
enum RuleItem {
    ID(u8),
    A,
    B,
}

type RuleMap = HashMap<u8, Vec<Vec<RuleItem>>>;

#[derive(Debug)]
struct Input {
    rule: Rule,
    rule_map: RuleMap,
    strings: Vec<String>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (rule_lines, string_lines): (&str, &str) =
            match input.splitn(2, "\n\n").collect::<Vec<&str>>()[..] {
                [rule_lines, string_lines] => (rule_lines, string_lines),
                _ => panic!("unexpected input"),
            };

        let mut rule_map: RuleMap = HashMap::new();

        for line in rule_lines.lines() {
            match line.splitn(2, ": ").collect::<Vec<&str>>()[..] {
                [id_string, rule_string] => {
                    let rule: Vec<Vec<RuleItem>> = rule_string
                        .split(" | ")
                        .map(|sequence| {
                            sequence
                                .trim_matches('"')
                                .split(" ")
                                .map(|ch| match ch {
                                    "a" => RuleItem::A,
                                    "b" => RuleItem::B,
                                    _ => RuleItem::ID(ch.parse().unwrap()),
                                })
                                .collect()
                        })
                        .collect();

                    rule_map.insert(id_string.parse().unwrap(), rule);
                }
                _ => panic!("invalid rule input"),
            }
        }

        fn map_sequence(rule: &Vec<RuleItem>, rule_map: &RuleMap) -> Vec<Rule> {
            rule.iter()
                .map(|item| match item {
                    RuleItem::A => Rule::A,
                    RuleItem::B => Rule::B,
                    RuleItem::ID(id) => match &rule_map.get(id).unwrap()[..] {
                        [sequence] => Rule::Sequence(map_sequence(sequence, rule_map)),
                        [sequence_one, sequence_two] => Rule::Either(
                            map_sequence(sequence_one, rule_map),
                            map_sequence(sequence_two, rule_map),
                        ),
                        _ => panic!("unexpected structure"),
                    },
                })
                .collect()
        }

        let rule_zero: &Vec<RuleItem> = &rule_map.get(&0).unwrap()[0];
        let rule: Rule = Rule::Sequence(map_sequence(rule_zero, &rule_map));

        let strings: Vec<String> = string_lines
            .lines()
            .map(|string| string.to_owned())
            .collect();

        Ok(Input {
            rule,
            rule_map,
            strings,
        })
    }
}

#[must_use]
pub fn part1(input_string: &str) -> usize {
    let input: Input = input_string.parse().unwrap();

    input
        .strings
        .iter()
        .filter(|string| {
            let (matching, consumed) = matches_rule(string.as_str(), &input.rule);
            matching && consumed == string.len()
        })
        .count()
}

fn matches_rule(string: &str, rule: &Rule) -> (bool, usize) {
    match rule {
        Rule::A => {
            if string.chars().nth(0).unwrap() == 'a' {
                (true, 1)
            } else {
                (false, 0)
            }
        }
        Rule::B => {
            if string.chars().nth(0).unwrap() == 'b' {
                (true, 1)
            } else {
                (false, 0)
            }
        }
        Rule::Sequence(rules) => matches_rules(string, rules),
        Rule::Either(some_rules, other_rules) => {
            let (matching, consumed) = matches_rules(string, some_rules);
            if matching {
                (matching, consumed)
            } else {
                matches_rules(string, other_rules)
            }
        }
    }
}

fn matches_rules(string: &str, rules: &[Rule]) -> (bool, usize) {
    let mut index: usize = 0;
    let mut all_match: bool = true;

    for rule in rules {
        let (matching, consumed): (bool, usize) = matches_rule(&string[index..], rule);

        if matching {
            index += consumed;
        } else {
            all_match = false;
            break;
        }
    }

    (all_match, index)
}

#[must_use]
pub fn part2(input_string: &str) -> usize {
    let input: Input = input_string.parse().unwrap();
    //    dbg!(&input.rule_map);
    input.rule_map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "\
0: 4 1 5\n\
1: 2 3 | 3 2\n\
2: 4 4 | 5 5\n\
3: 4 5 | 5 4\n\
4: \"a\"\n\
5: \"b\"\n\
\n\
ababbb\n\
bababa\n\
abbbab\n\
aaabbb\n\
aaaabbb\n"
            ),
            2
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "\
42: 9 14 | 10 1\n\
9: 14 27 | 1 26\n\
10: 23 14 | 28 1\n\
1: \"a\"\n\
11: 42 31\n\
5: 1 14 | 15 1\n\
19: 14 1 | 14 14\n\
12: 24 14 | 19 1\n\
16: 15 1 | 14 14\n\
31: 14 17 | 1 13\n\
6: 14 14 | 1 14\n\
2: 1 24 | 14 4\n\
0: 8 11\n\
13: 14 3 | 1 12\n\
15: 1 | 14\n\
17: 14 2 | 1 7\n\
23: 25 1 | 22 14\n\
28: 16 1\n\
4: 1 1\n\
20: 14 14 | 1 15\n\
3: 5 14 | 16 1\n\
27: 1 6 | 14 18\n\
14: \"b\"\n\
21: 14 1 | 1 14\n\
25: 1 1 | 1 14\n\
22: 14 14\n\
8: 42\n\
26: 14 22 | 1 20\n\
18: 15 15\n\
7: 14 5 | 1 21\n\
24: 14 1\n\
\n\
abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n\
bbabbbbaabaabba\n\
babbbbaabbbbbabbbbbbaabaaabaaa\n\
aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n\
bbbbbbbaaaabbbbaaabbabaaa\n\
bbbababbbbaaaaaaaabbababaaababaabab\n\
ababaaaaaabaaab\n\
ababaaaaabbbaba\n\
baabbaaaabbaaaababbaababb\n\
abbbbabbbbaaaababbbbbbaaaababb\n\
aaaaabbaabaaaaababaa\n\
aaaabbaaaabbaaa\n\
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n\
babaaabbbaaabaababbaabababaaab\n\
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba\n"
            ),
            12
        )
    }
}
