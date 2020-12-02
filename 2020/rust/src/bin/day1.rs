use std::fs::File;
use std::io::Read;

fn part1(expenses: &[i32]) -> Option<i32> {
    if let Some((a, b)) = find_pair_sum(expenses, 2020) {
        Some(a * b)
    } else {
        None
    }
}

fn find_pair_sum(values: &[i32], target: i32) -> Option<(i32, i32)> {
    if let Some((value, rest)) = values.split_first() {
        if let Some(other_value) = rest
            .iter()
            .find(|&other_value| value + other_value == target)
        {
            Some((*value, *other_value))
        } else {
            find_pair_sum(rest, target)
        }
    } else {
        None
    }
}

fn part2(expenses: &[i32]) -> Option<i32> {
    if let Some((a, b, c)) = find_triple_sum(expenses, 2020) {
        Some(a * b * c)
    } else {
        None
    }
}

fn find_triple_sum(values: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    if let Some((value, rest)) = values.split_first() {
        if let Some((a, b)) = find_pair_sum(rest, target - value) {
            Some((*value, a, b))
        } else {
            find_triple_sum(rest, target)
        }
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../inputs/01")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expenses: Vec<i32> = contents
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();

    if let Some(part1) = part1(&expenses) {
        println!("part 1: {}", part1);
    }

    if let Some(part2) = part2(&expenses) {
        println!("part 2: {}", part2);
    }

    Ok(())
}
