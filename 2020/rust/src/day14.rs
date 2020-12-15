use std::collections::HashMap;
use std::str::FromStr;

enum Instruction {
    SetMask(Mask),
    SetMemory(usize, usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let parts: Vec<&str> = string.splitn(2, " = ").collect();

        match parts.as_slice() {
            ["mask", mask] => Ok(SetMask(mask.parse().unwrap())),
            [memory_location, value_string] => {
                let parts: Vec<&str> = memory_location.split(|ch| ch == '[' || ch == ']').collect();
                let value: usize = value_string.parse().unwrap();
                let location: usize = parts[1].parse().unwrap();

                Ok(SetMemory(location, value))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Mask {
    pattern: String,
    and_mask: usize,
    or_mask: usize,
}

impl Mask {
    fn mask(&self, value: usize) -> usize {
        (value & self.and_mask) | self.or_mask
    }

    fn addresses(&self, base: usize) -> Vec<usize> {
        let x_count: usize = self.pattern.chars().filter(|&ch| ch == 'X').count();

        (0..2usize.pow(x_count as u32))
            .map(|i| {
                let base_string = Self::padded_binary_string(base, 36);
                let values = Self::padded_binary_string(i, x_count);
                let mut values_to_fill = values.chars();

                let masked: String = base_string
                    .chars()
                    .zip(self.pattern.chars())
                    .map(|(b, m)| match (b, m) {
                        (c, '0') => c,
                        (_c, '1') => '1',
                        (_c, 'X') => values_to_fill.next().unwrap(),
                        _ => panic!(),
                    })
                    .collect();
                usize::from_str_radix(&masked, 2).unwrap()
            })
            .collect()
    }

    fn padded_binary_string(value: usize, size: usize) -> String {
        let string = format!("{:b}", value);
        "0".repeat(size - string.len()) + &string
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let pattern = string.to_owned();
        let and_mask = usize::from_str_radix(&string.replace("X", "1"), 2).unwrap();
        let or_mask = usize::from_str_radix(&string.replace("X", "0"), 2).unwrap();

        Ok(Mask {
            pattern,
            and_mask,
            or_mask,
        })
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    use Instruction::*;

    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse().unwrap();

    for line in input.lines() {
        match line.parse::<Instruction>().unwrap() {
            SetMask(new_mask) => mask = new_mask,
            SetMemory(location, value) => {
                memory.insert(location, mask.mask(value));
            }
        }
    }

    memory.values().sum()
}

// combinations
// 18 * 2**4 + 16 * 2**5 + 13 * 2**6 + 19 * 2**7 + 15 * 2**8 + 16 * 2**9 = 16096
#[must_use]
pub fn part2(input: &str) -> usize {
    use Instruction::*;
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse().unwrap();

    for line in input.lines() {
        match line.parse::<Instruction>().unwrap() {
            SetMask(new_mask) => mask = new_mask,
            SetMemory(location, value) => {
                for address in mask.addresses(location) {
                    memory.insert(address, value);
                }
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_examples() {
        assert_eq!(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                .parse::<Mask>()
                .unwrap()
                .mask(11),
            73
        );
        assert_eq!(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                .parse::<Mask>()
                .unwrap()
                .mask(101),
            101
        );
        assert_eq!(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                .parse::<Mask>()
                .unwrap()
                .mask(0),
            64
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
mem[8] = 11\n\
mem[7] = 101\n\
mem[8] = 0\n"
            ),
            165
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "\
mask = 000000000000000000000000000000X1001X\n\
mem[42] = 100\n\
mask = 00000000000000000000000000000000X0XX\n\
mem[26] = 1\n"
            ),
            208
        )
    }
}
