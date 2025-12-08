use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        process::exit(1);
    }

    let day: u32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: day must be a number");
        process::exit(1);
    });

    if !(1..=12).contains(&day) {
        eprintln!("Error: day must be between 1 and 12");
        process::exit(1);
    }

    let input_path = format!("../inputs/{:02}", day);
    let input = fs::read_to_string(&input_path).unwrap_or_else(|err| {
        eprintln!("Error reading {}: {}", input_path, err);
        process::exit(1);
    });

    match day {
        1 => {
            use advent_of_code::day01;
            println!("Part 1: {}", day01::part1(&input));
            println!("Part 2: {}", day01::part2(&input));
        }
        _ => {
            eprintln!("Day {} not yet implemented", day);
            process::exit(1);
        }
    }
}
