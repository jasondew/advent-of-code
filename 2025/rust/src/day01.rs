pub fn part1(input: &str) -> usize {
    let mut dial = 50;
    let mut zero_count = 0;

    for line in input.trim().lines() {
        let (direction, amount_string) = line.trim().split_at(1);
        let amount: usize = amount_string.trim().parse().unwrap();

        match direction {
            "L" => {
                dial = (dial + amount) % 100;
            }
            "R" => {
                dial = (dial + 100 - amount) % 100;
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part2(input: &str) -> usize {
    let mut dial = 50;
    let mut zero_count = 0;

    for line in input.trim().lines() {
        let (direction, amount_string) = line.trim().split_at(1);
        let amount: usize = amount_string.trim().parse().unwrap();

        match direction {
            "L" => {
                zero_count += if dial == 0 {
                    amount / 100
                } else {
                    (100 - dial + amount) / 100
                };
                dial = (dial + 100 - (amount % 100)) % 100;
            }
            "R" => {
                zero_count += (dial + amount) / 100;
                dial = (dial + amount) % 100;
            }
            _ => panic!("Invalid direction: {}", direction),
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    ";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }
}
