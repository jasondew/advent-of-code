use std::fs;
use std::io;

fn to_digits(n: u32) -> Vec<i8> {
    let mut digits: Vec<i8> = Vec::new();
    let mut n = n;

    while n > 9 {
        digits.push((n % 10) as i8);
        n = n / 10;
    }
    digits.push(n as i8);
    digits.reverse();

    digits
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("../inputs/04")?;
    if let Some([from, to]) = input
        .trim()
        .split("-")
        .map(|string| string.parse().unwrap())
        .collect::<Vec<u32>>()
        .get(0..2)
    {
        let mut part1_count: u16 = 0;
        let mut part2_count: u16 = 0;

        for number in *from..*to {
            let digits = to_digits(number);
            let mut previous_digit: i8 = -1;
            let mut non_decreasing = true;
            let mut two_in_a_row = false;
            let mut strictly_two_in_a_row = false;
            let mut duplicity = 1;

            digits.iter().for_each(|digit| {
                if &previous_digit > digit {
                    non_decreasing = false
                };

                if &previous_digit == digit {
                    two_in_a_row = true;
                    duplicity += 1;
                } else {
                    if duplicity == 2 {
                        strictly_two_in_a_row = true
                    };
                    duplicity = 1;
                };

                previous_digit = *digit;
            });

            if duplicity == 2 {
                strictly_two_in_a_row = true
            };

            if non_decreasing && two_in_a_row {
                part1_count += 1
            };

            if non_decreasing && strictly_two_in_a_row {
                part2_count += 1
            };
        }

        println!("part 1: {}", part1_count);
        println!("part 2: {}", part2_count);
    }

    Ok(())
}
