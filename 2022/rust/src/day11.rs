use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Item {
    worry_level: usize,
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn execute(&self, worry_level: usize) -> usize {
        match self {
            Operation::Add(amount) => worry_level + amount,
            Operation::Multiply(amount) => worry_level * amount,
            Operation::Square => worry_level * worry_level,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: usize,
    true_monkey_id: MonkeyID,
    false_monkey_id: MonkeyID,
}

impl Test {
    fn run(&self, value: usize) -> MonkeyID {
        if value % self.divisor == 0 {
            self.true_monkey_id
        } else {
            self.false_monkey_id
        }
    }
}

type MonkeyID = u8;

#[derive(Debug)]
struct Monkey {
    #[allow(dead_code)]
    id: MonkeyID,
    inventory: VecDeque<Item>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

impl Monkey {
    fn play(&mut self, transfer_queue: &mut HashMap<MonkeyID, Vec<Item>>) {
        for mut item in self.inventory.drain(..) {
            item.worry_level = self.operation.execute(item.worry_level) / 3;
            self.inspection_count += 1;

            let to_id = self.test.run(item.worry_level);
            transfer_queue.get_mut(&to_id).unwrap().push(item);
        }
    }

    fn play_rough(
        &mut self,
        transfer_queue: &mut HashMap<MonkeyID, Vec<Item>>,
    ) {
        for mut item in self.inventory.drain(..) {
            item.worry_level = self.operation.execute(item.worry_level);
            self.inspection_count += 1;

            let to_id = self.test.run(item.worry_level);
            transfer_queue.get_mut(&to_id).unwrap().push(item);
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut monkeys = parse(input);
    let mut transfer_queue: HashMap<MonkeyID, Vec<Item>> = HashMap::new();
    let mut ids: Vec<MonkeyID> = monkeys.keys().copied().collect();
    ids.sort_unstable();

    for _round in 0..20 {
        for id in &ids {
            for &id in monkeys.keys() {
                transfer_queue.insert(id, Vec::new());
            }

            monkeys.get_mut(id).unwrap().play(&mut transfer_queue);

            for (to_id, items) in transfer_queue.drain() {
                let recipient = monkeys.get_mut(&to_id).unwrap();
                for item in items {
                    recipient.inventory.push_back(item);
                }
            }
        }
    }

    product_of_highest_two_inspection_counts(&monkeys)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut monkeys = parse(input);
    let modulo: usize = monkeys
        .iter()
        .map(|(_id, monkey)| monkey.test.divisor)
        .product();
    let mut transfer_queue: HashMap<MonkeyID, Vec<Item>> = HashMap::new();
    let mut ids: Vec<MonkeyID> = monkeys.keys().copied().collect();
    ids.sort_unstable();

    for _round in 0..10_000 {
        for id in &ids {
            for &id in monkeys.keys() {
                transfer_queue.insert(id, Vec::new());
            }

            monkeys.get_mut(id).unwrap().play_rough(&mut transfer_queue);

            for (to_id, items) in transfer_queue.drain() {
                let recipient = monkeys.get_mut(&to_id).unwrap();
                for mut item in items {
                    item.worry_level %= modulo;
                    recipient.inventory.push_back(item);
                }
            }
        }
    }

    product_of_highest_two_inspection_counts(&monkeys)
}

fn product_of_highest_two_inspection_counts(
    monkeys: &HashMap<MonkeyID, Monkey>,
) -> usize {
    let mut inspection_counts: Vec<usize> = monkeys
        .iter()
        .map(|(_id, monkey)| monkey.inspection_count)
        .collect();
    inspection_counts.sort_unstable();
    inspection_counts.reverse();
    inspection_counts.iter().take(2).product()
}

fn parse(input: &str) -> HashMap<MonkeyID, Monkey> {
    input
        .split("\n\n")
        .map(|monkey_lines| {
            let mut iter = monkey_lines.lines();
            let id = iter
                .next()
                .unwrap()
                .strip_prefix("Monkey ")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<MonkeyID>()
                .unwrap();
            let inventory = iter
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|worry_string| Item {
                    worry_level: worry_string.parse::<usize>().unwrap(),
                })
                .collect();
            let operation = match iter
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_once(' ')
                .unwrap()
            {
                ("*", "old") => Operation::Square,
                ("*", value_string) => {
                    Operation::Multiply(value_string.parse::<usize>().unwrap())
                }
                ("+", value_string) => {
                    Operation::Add(value_string.parse::<usize>().unwrap())
                }
                _ => panic!("invalid operation"),
            };
            let divisor = iter
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let test = Test {
                divisor,
                true_monkey_id: iter
                    .next()
                    .unwrap()
                    .strip_prefix("    If true: throw to monkey ")
                    .unwrap()
                    .parse::<MonkeyID>()
                    .unwrap(),
                false_monkey_id: iter
                    .next()
                    .unwrap()
                    .strip_prefix("    If false: throw to monkey ")
                    .unwrap()
                    .parse::<MonkeyID>()
                    .unwrap(),
            };

            (
                id,
                Monkey {
                    id,
                    inventory,
                    operation,
                    test,
                    inspection_count: 0,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 10605)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 2713310158)
    }
}
