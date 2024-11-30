#[derive(Clone, Debug, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    counts: Vec<usize>,
}

impl ConditionRecord {
    fn unfold(&mut self) {
        let mut conditions_with_joiner = self.conditions.clone();
        conditions_with_joiner.push(Condition::Unknown);

        self.conditions = conditions_with_joiner
            .iter()
            .cycle()
            .take(self.conditions.len() * 5 + 4)
            .cloned()
            .collect();
        self.counts = self
            .counts
            .iter()
            .cycle()
            .take(self.counts.len() * 5)
            .cloned()
            .collect();
    }

    fn possibilities(&self) -> usize {
        self.unknown_patterns()
            .iter()
            .filter(|pattern| {
                self.group_count_with_pattern(&pattern) == self.counts
            })
            .count()
    }

    fn unknown_patterns(&self) -> Vec<Vec<usize>> {
        let size = Self::unknown_count(&self.conditions);

        (0..2_usize.pow(size as u32))
            .map(|value| {
                format!("{:0>width$b}", value, width = size)
                    .chars()
                    .map(|c| if c == '0' { 0 } else { 1 })
                    .collect()
            })
            .collect()
    }

    fn group_count_with_pattern(&self, pattern: &Vec<usize>) -> Vec<usize> {
        let mut pattern_iter = pattern.iter();

        let conditions: Vec<Condition> = self
            .conditions
            .iter()
            .map(|condition| match condition {
                Condition::Operational => Condition::Operational,
                Condition::Damaged => Condition::Damaged,
                Condition::Unknown => match pattern_iter.next() {
                    Some(1) => Condition::Damaged,
                    Some(0) => Condition::Operational,
                    _ => panic!("shouldn't get here"),
                },
            })
            .collect();

        Self::group_count(&conditions)
    }

    fn unknown_count(conditions: &[Condition]) -> usize {
        conditions
            .iter()
            .filter(|c| *c == &Condition::Unknown)
            .count()
    }

    fn group_count(conditions: &Vec<Condition>) -> Vec<usize> {
        conditions
            .split(|c| c == &Condition::Operational)
            .map(|cs| cs.len())
            .filter(|s| s > &0)
            .collect()
    }

    #[allow(dead_code)]
    fn debug(conditions: &[Condition]) {
        for condition in conditions {
            let ch = match condition {
                Condition::Operational => '.',
                Condition::Damaged => '#',
                Condition::Unknown => '?',
            };
            print!("{}", ch);
        }
        println!("");
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let condition_records = parse(input);

    condition_records
        .iter()
        .map(|condition_record| condition_record.possibilities())
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut condition_records = parse(input);

    condition_records
        .iter_mut()
        .map(|condition_record| {
            dbg!(&condition_record);
            condition_record.unfold();
            dbg!(&condition_record);
            condition_record.possibilities()
        })
        .sum()
}

fn parse(input: &str) -> Vec<ConditionRecord> {
    input
        .lines()
        .map(|line| {
            let (conditions_string, counts_string) =
                line.split_once(' ').unwrap();
            let conditions = conditions_string
                .chars()
                .map(|ch| match ch {
                    '?' => Condition::Unknown,
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    invalid => panic!("invalid condition: {:?}", invalid),
                })
                .collect();
            let counts: Vec<usize> = counts_string
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect();
            ConditionRecord { conditions, counts }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edge1() -> &'static str {
        "?????#?#.???.?.?###? 1,4,1,5"
    }

    fn edge2() -> &'static str {
        ".##.?#??.#.?# 2,1,1,1"
    }

    fn input() -> &'static str {
        "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(edge2()), 1);
        assert_eq!(part1(edge1()), 13);
        assert_eq!(part1(input()), 21)
    }

    #[test]
    fn part2_example() {
        //        assert_eq!(part2(input()), 525152)
    }
}
