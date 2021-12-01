#[must_use]
pub fn part1(input: &str) -> usize {
    let depths: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    increase_count(depths)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let depths: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let three_measurement_depths: Vec<u32> = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((&first, &second), &third)| first + second + third)
        .collect();

    increase_count(three_measurement_depths)
}

fn increase_count(depths: Vec<u32>) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(&previous, &current)| (current as i32 - previous as i32) > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"),
            7
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"),
            5
        )
    }
}
