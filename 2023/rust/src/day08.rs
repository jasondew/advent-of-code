use num::integer;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

type Node = String;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (instructions, node_map) = parse(input);
    let starting_node = &"AAA".to_owned();

    path_length(starting_node, &instructions, &node_map)
}

fn path_length(
    starting_node: &String,
    instructions: &Vec<Instruction>,
    node_map: &HashMap<Node, (Node, Node)>,
) -> usize {
    let mut node: &String = starting_node;
    let mut steps = 0usize;

    for instruction in instructions.iter().cycle() {
        steps += 1;

        let (left_node, right_node) = node_map.get(node).unwrap();
        match instruction {
            Instruction::Left => {
                node = left_node;
            }
            Instruction::Right => {
                node = right_node;
            }
        }

        if node.ends_with("Z") {
            break;
        }
    }

    steps
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (instructions, node_map) = parse(input);
    let steps: Vec<usize> = nodes_ending_with_a(&node_map)
        .par_iter()
        .map(|starting_node| {
            path_length(starting_node, &instructions, &node_map)
        })
        .collect();

    steps
        .into_iter()
        .reduce(|acc, x| integer::lcm(acc, x))
        .unwrap()
}

fn nodes_ending_with_a(node_map: &HashMap<Node, (Node, Node)>) -> Vec<&Node> {
    node_map.keys().filter(|node| node.ends_with("A")).collect()
}

fn parse(input: &str) -> (Vec<Instruction>, HashMap<Node, (Node, Node)>) {
    let (instructions_string, nodes_string) = input.split_once("\n\n").unwrap();

    let instructions = instructions_string
        .chars()
        .map(|ch| match ch {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            invalid => panic!("invalid instruction: {:?}", invalid),
        })
        .collect();

    let mut node_map: HashMap<Node, (Node, Node)> = HashMap::new();
    for line in nodes_string.lines() {
        let (node, left_and_right) = line.split_once(" = ").unwrap();
        let parens: &[_] = &['(', ')'];
        let (left, right) = left_and_right
            .trim_matches(parens)
            .split_once(", ")
            .unwrap();
        node_map.insert(node.to_owned(), (left.to_owned(), right.to_owned()));
    }

    (instructions, node_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
    }

    fn input2() -> &'static str {
        "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    }

    fn input3() -> &'static str {
        "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 2);
        assert_eq!(part1(input2()), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input3()), 6);
    }
}
