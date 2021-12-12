use std::collections::HashMap;
use std::collections::HashSet;

type Cave = String;

#[must_use]
pub fn part1(input: &str) -> usize {
    let connections = parse(input);
    let small_caves_visitable_once = |next_cave: &Cave, path: &Vec<Cave>| -> bool {
        large_cave(next_cave) || !path.iter().any(|cave| cave == next_cave)
    };

    let all_paths: Vec<Vec<Cave>> = traverse(
        &connections,
        &small_caves_visitable_once,
        "start".into(),
        vec![],
    )
    .into_iter()
    .filter(|path| path[path.len() - 1] == "end")
    .collect();

    all_paths.len()
}

fn traverse(
    connections: &HashMap<Cave, Vec<Cave>>,
    visitable: &dyn Fn(&Cave, &Vec<Cave>) -> bool,
    current_cave: Cave,
    mut path: Vec<Cave>,
) -> Vec<Vec<Cave>> {
    path.push(current_cave.clone());

    if current_cave == "end" {
        return vec![path.clone()];
    }

    connections
        .get(&current_cave)
        .unwrap()
        .iter()
        .flat_map(|next_cave| {
            if visitable(next_cave, &path) {
                traverse(connections, visitable, next_cave.clone(), path.clone())
            } else {
                vec![path.clone()]
            }
        })
        .collect()
}

fn large_cave(cave: &Cave) -> bool {
    cave.chars().any(|c| c.is_uppercase())
}

fn cave_not_seen(cave: &Cave, path: &Vec<Cave>) -> bool {
    !path.iter().any(|c| c == cave)
}

fn small_cave_visited_twice(path: &Vec<Cave>) -> bool {
    let mut sorted_path: Vec<Cave> = path
        .iter()
        .filter(|cave| !large_cave(cave))
        .cloned()
        .collect();
    sorted_path.sort_unstable();
    sorted_path
        .iter()
        .zip(sorted_path.iter().skip(1))
        .any(|(a, b)| a == b)
}

fn parse(input: &str) -> HashMap<Cave, Vec<Cave>> {
    let mut caves: HashSet<Cave> = HashSet::new();
    let mut paths: Vec<(Cave, Cave)> = Vec::new();
    let mut connections: HashMap<Cave, Vec<Cave>> = HashMap::new();

    for line in input.trim_end().lines() {
        let (left, right) = line.split_once("-").unwrap();
        let left_cave: Cave = left.into();
        let right_cave: Cave = right.into();

        caves.insert(left_cave.clone());
        caves.insert(right_cave.clone());
        paths.push((left_cave.clone(), right_cave.clone()));
        paths.push((right_cave, left_cave));
    }

    for cave in caves {
        let next_caves = paths
            .iter()
            .filter_map(|(from, to)| if *from == cave { Some(to) } else { None })
            .cloned()
            .collect();
        connections.insert(cave, next_caves);
    }

    connections
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let connections = parse(input);
    let small_caves_visitable_twice = |next_cave: &Cave, path: &Vec<Cave>| -> bool {
        next_cave != "start"
            && (large_cave(next_cave)
                || cave_not_seen(next_cave, &path)
                || !small_cave_visited_twice(&path))
    };

    let all_paths: Vec<Vec<Cave>> = traverse(
        &connections,
        &small_caves_visitable_twice,
        "start".into(),
        vec![],
    )
    .into_iter()
    .filter(|path| path[path.len() - 1] == "end")
    .collect();

    //    for path in &all_paths {
    //        println!("{}", path.join(","));
    //    }

    all_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_input() -> &'static str {
        "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end\n"
    }

    fn larger_input() -> &'static str {
        "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc\n"
    }

    fn large_input() -> &'static str {
        "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(small_input()), 10);
        assert_eq!(part1(larger_input()), 19);
        assert_eq!(part1(large_input()), 226);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(small_input()), 36);
        assert_eq!(part2(larger_input()), 103);
        assert_eq!(part2(large_input()), 3509);
    }
}
