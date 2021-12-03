#[must_use]
pub fn part1(input: &str) -> usize {
    let diagnostics: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let gamma_string: String = most_common_bits(&diagnostics).into_iter().collect();
    let gamma: usize = usize::from_str_radix(&gamma_string, 2).unwrap();

    let epsilon_string: String = gamma_string
        .replace("0", "x")
        .replace("1", "0")
        .replace("x", "1");
    let epsilon: usize = usize::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let diagnostics: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut index = 0;
    let mut oxygen_generator_rating_candidates = diagnostics.clone();

    while oxygen_generator_rating_candidates.len() > 1 {
        let mut most_common_bit = most_common_bits(&oxygen_generator_rating_candidates)[index];

        if most_common_bit == 'x' {
            most_common_bit = '1'
        };

        oxygen_generator_rating_candidates = oxygen_generator_rating_candidates
            .iter()
            .filter(|number| number[index] == most_common_bit)
            .cloned()
            .collect();

        index += 1;
    }

    let oxygen_generator_rating = usize::from_str_radix(
        &oxygen_generator_rating_candidates[0]
            .iter()
            .collect::<String>(),
        2,
    )
    .unwrap();

    index = 0;
    let mut c02_scrubber_rating_candidates = diagnostics.clone();

    while c02_scrubber_rating_candidates.len() > 1 {
        let mut most_common_bit = most_common_bits(&c02_scrubber_rating_candidates)[index];

        if most_common_bit == 'x' {
            most_common_bit = '0'
        } else if most_common_bit == '1' {
            most_common_bit = '0'
        } else if most_common_bit == '0' {
            most_common_bit = '1'
        };

        c02_scrubber_rating_candidates = c02_scrubber_rating_candidates
            .iter()
            .filter(|number| number[index] == most_common_bit)
            .cloned()
            .collect();

        index += 1;
    }

    let c02_scrubber_rating = usize::from_str_radix(
        &c02_scrubber_rating_candidates[0].iter().collect::<String>(),
        2,
    )
    .unwrap();

    oxygen_generator_rating * c02_scrubber_rating
}

fn most_common_bits(diagnostics: &Vec<Vec<char>>) -> Vec<char> {
    let total_count = diagnostics.len();
    let mut one_counts: Vec<usize> = vec![0; diagnostics[0].len()];

    for number in diagnostics {
        for (index, &bit) in number.iter().enumerate() {
            if bit == '1' {
                one_counts[index] += 1
            }
        }
    }

    one_counts
        .iter()
        .map(|&ones_count| {
            if 2 * ones_count == total_count {
                'x'
            } else {
                if 2 * ones_count > total_count {
                    '1'
                } else {
                    '0'
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"), 198)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"), 230)
    }
}
