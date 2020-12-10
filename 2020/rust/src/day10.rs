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

    let mut cache: HashMap<usize, usize> = HashMap::new();

    chain_count(&voltages, 0, 1, &mut cache)
}

fn chain_count(
    voltages: &[usize],
    current: usize,
    count: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(count) = cache.get(&current) {
        *count
    } else {
        let (options, rest): (Vec<usize>, Vec<usize>) =
            voltages.iter().partition(|&v| v <= &(current + 3));
        let count = match options.as_slice() {
            [] => count,
            [x] => chain_count(&rest, *x, count, cache),
            [x, y] => {
                let a = chain_count(&rest, *y, count, cache);
                let mut rr: Vec<usize> = rest.to_vec();
                rr.insert(0, *y);
                let b = chain_count(&rr, *x, count, cache);
                a + b
            }
            [x, y, z] => {
                let a = chain_count(&rest, *z, count, cache);
                let mut rr: Vec<usize> = rest.to_vec();
                rr.insert(0, *z);
                let b = chain_count(&rr, *y, count, cache);
                rr.insert(0, *y);
                let c = chain_count(&rr, *x, count, cache);

                a + b + c
            }
            _ => panic!(""),
        };
        cache.insert(current, count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n"), 35);
        assert_eq!(part1("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n"), 220);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n"), 8);
        assert_eq!(part2("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n"), 19208);
    }
}
