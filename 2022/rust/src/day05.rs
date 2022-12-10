use regex::Regex;
use std::collections::VecDeque;

type Crate = char;
type StackId = usize;

#[derive(Debug)]
#[allow(dead_code)]
struct Stack {
    id: StackId,
    contents: VecDeque<Crate>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Instruction {
    count: usize,
    from: StackId,
    to: StackId,
}

#[must_use]
pub fn part1(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);

    for instruction in instructions {
        for _ in 1..=instruction.count {
            move_single(&mut stacks, instruction.from, instruction.to)
        }
    }

    stacks
        .iter()
        .map(|stack| stack.contents.iter().next().unwrap())
        .collect()
}

#[must_use]
pub fn part2(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);

    for instruction in instructions {
        move_multiple(
            &mut stacks,
            instruction.from,
            instruction.to,
            instruction.count,
        )
    }

    stacks
        .iter()
        .map(|stack| stack.contents.iter().next().unwrap())
        .collect()
}

fn move_single(stacks: &mut Vec<Stack>, from_id: StackId, to_id: StackId) {
    let crate_to_move = stacks
        .iter_mut()
        .find(|stack| stack.id == from_id)
        .unwrap()
        .contents
        .pop_front()
        .unwrap();
    stacks
        .iter_mut()
        .find(|stack| stack.id == to_id)
        .unwrap()
        .contents
        .push_front(crate_to_move);
}

fn move_multiple(
    stacks: &mut Vec<Stack>,
    from_id: StackId,
    to_id: StackId,
    count: usize,
) {
    let from_stack = &mut stacks
        .iter_mut()
        .find(|stack| stack.id == from_id)
        .unwrap()
        .contents;

    let mut crates_to_move: VecDeque<char> = VecDeque::new();

    for _ in 1..=count {
        crates_to_move.push_front(from_stack.pop_front().unwrap())
    }

    let to_stack =
        &mut stacks.iter_mut().find(|stack| stack.id == to_id).unwrap();

    for crate_to_move in crates_to_move {
        to_stack.contents.push_front(crate_to_move);
    }
}

fn parse(input: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let (stack_lines, instruction_lines) =
        input.split_once("\n\n").expect("valid input");

    (
        parse_stacks(stack_lines),
        parse_instructions(instruction_lines),
    )
}

fn parse_stacks(stack_lines: &str) -> Vec<Stack> {
    let map: Vec<Vec<char>> = stack_lines
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let size: usize = map.len() - 1;
    let id_row: usize = map.len() - 1;
    let mut column: usize = 1;
    let mut stacks: Vec<Stack> = Vec::new();

    loop {
        stacks.push(Stack {
            id: map[id_row][column] as usize - 48,
            contents: map
                .iter()
                .take(size)
                .map(|line| line[column])
                .filter(char::is_ascii_alphabetic)
                .collect(),
        });

        column += 4;
        if column >= map[0].len() {
            break;
        }
    }

    stacks
}

fn parse_instructions(instruction_lines: &str) -> Vec<Instruction> {
    let instruction_regex =
        Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("valid regex");

    instruction_lines
        .lines()
        .map(|line| {
            let captures = instruction_regex
                .captures(line.trim())
                .expect("instruction format");
            Instruction {
                count: captures[1].parse::<usize>().expect("numeric count"),
                from: captures[2].parse::<usize>().expect("numeric from"),
                to: captures[3].parse::<usize>().expect("numeric to"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), "CMZ")
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), "MCD")
    }
}
