use std::collections::{HashMap, HashSet};

type Device = String;

#[must_use]
pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let paths = bfs("you".to_string(), &map);

    paths
        .iter()
        .filter(|path| path.last().unwrap() == "out")
        .count()
}

fn bfs(start: Device, map: &HashMap<Device, Vec<Device>>) -> Vec<Vec<Device>> {
    let mut paths: Vec<Vec<Device>> = vec![vec![start.clone()]];
    let mut frontier = vec![start];
    let mut visited = Vec::new();

    loop {
        if frontier.is_empty() {
            break;
        }

        let current = frontier.remove(0);

        visited.push(current.clone());

        if let Some(nexts) = map.get(&current) {
            for next in nexts {
                if !visited.contains(next) && next != "out" {
                    frontier.push(next.clone());
                }
            }

            let paths_to_update: Vec<Vec<Device>> = paths
                .iter()
                .filter(|path| path.last().unwrap() == &current)
                .cloned()
                .collect();

            paths.retain(|path| path.last().unwrap() != &current);

            for next in nexts {
                for path in &paths_to_update {
                    let mut updated_path = path.clone();
                    updated_path.push(next.clone());
                    paths.push(updated_path);
                }
            }
        }
    }

    paths
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let map = parse(input);
    let required = vec!["dac".to_string(), "fft".to_string()];
    count_paths_through_devices("svr".to_string(), required, &map)
}

fn count_paths_through_devices(
    start: Device,
    required: Vec<Device>,
    map: &HashMap<Device, Vec<Device>>,
) -> usize {
    let mut memo: HashMap<(Device, Vec<Device>), usize> = HashMap::new();
    let mut path = HashSet::new();
    path.insert(start.clone());

    count_paths_dfs(start, vec![], &required, map, &mut path, &mut memo)
}

fn count_paths_dfs(
    current: Device,
    visited_required: Vec<Device>,
    required: &[Device],
    map: &HashMap<Device, Vec<Device>>,
    path: &mut HashSet<Device>,
    memo: &mut HashMap<(Device, Vec<Device>), usize>,
) -> usize {
    if current == "out" {
        if required.iter().all(|r| visited_required.contains(r)) {
            return 1;
        } else {
            return 0;
        }
    }

    // Check memo for this state (sorted for consistency)
    let mut state_key = visited_required.clone();
    state_key.sort();
    let state = (current.clone(), state_key);
    if let Some(&cached) = memo.get(&state) {
        return cached;
    }

    let mut count = 0;

    if let Some(nexts) = map.get(&current) {
        for next in nexts {
            if !path.contains(next) {
                // Add to path
                path.insert(next.clone());

                // Update visited required nodes
                let mut new_visited = visited_required.clone();
                if required.contains(next) && !new_visited.contains(next) {
                    new_visited.push(next.clone());
                }

                // Recurse
                count += count_paths_dfs(
                    next.clone(),
                    new_visited,
                    required,
                    map,
                    path,
                    memo,
                );

                // Backtrack
                path.remove(next);
            }
        }
    }

    // Cache result
    memo.insert(state, count);
    count
}

fn parse(input: &str) -> HashMap<Device, Vec<Device>> {
    let mut map = HashMap::new();

    for line in input.trim().lines() {
        let (device, connections_string) = line.split_once(": ").unwrap();
        let connections = connections_string
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        map.insert(device.to_string(), connections);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_EXAMPLE_INPUT: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    const PART_2_EXAMPLE_INPUT: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_part1_with_example_input() {
        assert_eq!(part1(PART_1_EXAMPLE_INPUT), 5);
    }

    #[test]
    fn test_part2_with_example_input() {
        assert_eq!(part2(PART_2_EXAMPLE_INPUT), 2);
    }
}
