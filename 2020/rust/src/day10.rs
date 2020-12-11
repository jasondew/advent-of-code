use std::collections::HashMap;

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut voltages: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    voltages.sort_unstable();
    let deltas: Vec<usize> = voltages
        .iter()
        .skip(1)
        .zip(voltages.iter())
        .map(|(next, prev)| next - prev)
        .collect();

    let delta_threes: usize = deltas.iter().filter(|&delta| delta == &3).count();
    let delta_ones: usize = deltas.iter().filter(|&delta| delta == &1).count();

    (delta_threes + 1) * (delta_ones + 1)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut voltages: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    voltages.sort_unstable();
    voltages.insert(0, 0);

    let mut cache: HashMap<usize, usize> = HashMap::new();

    chain_count(&voltages, 0, &mut cache)
}

fn chain_count(voltages: &[usize], index: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&count) = cache.get(&index) {
        count
    } else {
        let current: usize = voltages[index];
        let to_index: usize = (index + 4).min(voltages.len());
        let options: Vec<usize> = voltages[(index + 1)..to_index]
            .iter()
            .filter(|&v| v <= &(current + 3))
            .map(|&v| v)
            .collect();
        let count = match options.as_slice() {
            [] => 1,
            [_x] => chain_count(&voltages, index + 1, cache),
            [_x, _y] => {
                chain_count(&voltages, index + 2, cache) + chain_count(&voltages, index + 1, cache)
            }
            [_x, _y, _z] => {
                chain_count(&voltages, index + 3, cache)
                    + chain_count(&voltages, index + 2, cache)
                    + chain_count(&voltages, index + 1, cache)
            }
            _ => panic!("can't get here"),
        };

        cache.insert(index, count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part1("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n"), 35);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n"), 220);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n"), 8);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n"), 19208);
    }
}
