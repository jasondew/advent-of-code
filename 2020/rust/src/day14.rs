use std::collections::HashMap;

struct Mask {
    // mask value result
    // 0      0      0
    // 0      1      0
    // 1      0      1
    // 1      1      1
    and_mask: usize,
    or_mask: usize,
}

impl Mask {
    fn mask(&self, value: usize) -> usize {
        (value & self.and_mask) | self.or_mask
    }
}

impl std::str::FromStr for Mask {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let and_mask = usize::from_str_radix(&string.replace("X", "1"), 2).unwrap();
        let or_mask = usize::from_str_radix(&string.replace("X", "0"), 2).unwrap();

        Ok(Mask { and_mask, or_mask })
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut memory: HashMap<&str, usize> = HashMap::new();
    let mut mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".parse().unwrap();

    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(2, " = ").collect();

        match parts.as_slice() {
            ["mask", new_mask] => mask = new_mask.parse().unwrap(),
            [memory_location, value_string] => {
                let parts: Vec<&str> = memory_location.split(|ch| ch == '[' || ch == ']').collect();
                let value: usize = value_string.parse().unwrap();
                memory.insert(parts[1], mask.mask(value));
            }
            _ => panic!(),
        }
    }

    memory.values().sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    // combinations
    // 18 * 2**4 + 16 * 2**5 + 13 * 2**6 + 19 * 2**7 + 15 * 2**8 + 16 * 2**9 = 16096
    input.len()
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
        assert_eq!(part2("quux"), 4)
    }
}
