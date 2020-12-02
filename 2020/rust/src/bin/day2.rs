use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn part1(password_entries: &Vec<PasswordEntry>) -> usize {
    password_entries
        .iter()
        .filter(|password_entry| password_entry.is_valid())
        .count()
}

fn part2(password_entries: &Vec<PasswordEntry>) -> usize {
    password_entries
        .iter()
        .filter(|password_entry| password_entry.is_valid_part_two())
        .count()
}

#[derive(Debug)]
struct PasswordEntry {
    password: String,
    letter: char,
    left: usize,
    right: usize,
}

impl PasswordEntry {
    fn is_valid(self: &Self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        count >= self.left && count <= self.right
    }

    fn is_valid_part_two(self: &Self) -> bool {
        let left: char = self.password.chars().nth(self.left - 1).unwrap();
        let right: char = self.password.chars().nth(self.right - 1).unwrap();

        ((left == self.letter) && !(right == self.letter))
            || ((right == self.letter) && !(left == self.letter))
    }
}

impl FromStr for PasswordEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ": ").collect();
        let password: &str = parts[1];
        let parts: Vec<&str> = parts[0].splitn(2, " ").collect();
        let letter: char = parts[1].chars().next().unwrap();
        let numbers: Vec<usize> = parts[0]
            .splitn(2, "-")
            .map(|str| str.parse().unwrap())
            .collect();

        Ok(PasswordEntry {
            password: password.into(),
            letter: letter,
            left: numbers[0],
            right: numbers[1],
        })
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../inputs/02")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let password_entries: Vec<PasswordEntry> =
        contents.lines().map(|str| str.parse().unwrap()).collect();

    println!("part 1: {}", part1(&password_entries));
    println!("part 2: {}", part2(&password_entries));

    Ok(())
}
