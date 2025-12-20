use std::env;
use std::fs;
use std::process;

macro_rules! days {
    ($($day:literal => $mod:ident),* $(,)?) => {
        fn run_day(day: u32, input: &str) {
            match day {
                $(
                    $day => {
                        use advent_of_code::$mod;
                        println!("Part 1: {}", $mod::part1(input));
                        println!("Part 2: {}", $mod::part2(input));
                    }
                )*
                _ => {
                    eprintln!("Day {} not yet implemented", day);
                    process::exit(1);
                }
            }
        }
    };
}

days!(
    1 => day01,
    2 => day02,
    3 => day03,
    4 => day04,
    5 => day05,
    6 => day06,
    7 => day07,
    8 => day08,
    9 => day09,
    10 => day10,
    11 => day11,
);

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

    if !(1..=25).contains(&day) {
        eprintln!("Error: day must be between 1 and 25");
        process::exit(1);
    }

    let input_path = format!("../inputs/{:02}", day);
    let input = fs::read_to_string(&input_path).unwrap_or_else(|err| {
        eprintln!("Error reading {}: {}", input_path, err);
        process::exit(1);
    });

    run_day(day, &input);
}
