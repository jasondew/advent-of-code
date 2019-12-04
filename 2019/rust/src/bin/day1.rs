use std::fs::File;
use std::io::Read;

fn fuel_needed(&mass: &i32) -> i32 {
    mass / 3 - 2
}

fn fuel_needed_including_fuel(&mass: &i32) -> i32 {
    let fuel = fuel_needed(&mass);
    if fuel > 8 {
        fuel + fuel_needed_including_fuel(&fuel)
    } else {
        fuel
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("../inputs/01")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let masses: Vec<i32> = contents
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();

    let part1: i32 = masses.iter().map(|mass| fuel_needed(mass)).sum();
    println!("part 1: {}", part1);

    let part2: i32 = masses
        .iter()
        .map(|mass| fuel_needed_including_fuel(mass))
        .sum();
    println!("part 2: {}", part2);

    Ok(())
}
