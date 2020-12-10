#[must_use]
pub fn part1(input: &str) -> usize {
    let mut voltages: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    voltages.sort();
    let deltas: Vec<usize> = voltages
        .iter()
        .skip(1)
        .zip(voltages.iter())
        .map(|(next, prev)| next - prev)
        .collect();

    let delta_threes = deltas.iter().filter(|&delta| delta == &3usize).count();
    let delta_ones = deltas.iter().filter(|&delta| delta == &1usize).count();

    (delta_threes + 1) * (delta_ones + 1)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n"), 35);
        assert_eq!(part1("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n"), 220)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("quux"), 4)
    }
}
