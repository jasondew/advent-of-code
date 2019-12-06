use std::fs;
use std::io;

fn to_digits(n: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut n = n;

    while n > 9 {
        digits.push((n % 10) as u8);
        n = n / 10;
    }
    digits.push(n as u8);
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
        let mut count: u16 = 0;
        for number in *from..*to {
            let digits = to_digits(number);
            let mut previous_digit: u8 = 0;
            let mut non_decreasing = true;
            let mut two_in_a_row = false;

            digits.iter().for_each(|digit| {
                if digit < &previous_digit {
                    non_decreasing = false
                };
                if digit == &previous_digit {
                    two_in_a_row = true
                };

                previous_digit = *digit
            });

            if non_decreasing && two_in_a_row {
                count = count + 1
            }
        }

        println!("part 1: {}", count);
    }

    Ok(())
}
