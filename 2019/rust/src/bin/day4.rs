use std::fs;
use std::io;

fn non_decreasing(number: u32) -> bool {
    false
}

fn two_in_a_row(number: u32) -> bool {
    false
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
            if non_decreasing(number) && two_in_a_row(number) {
                count = count + 1
            }
        }

        println!("part 1: {}", count);
    }

    Ok(())
}
