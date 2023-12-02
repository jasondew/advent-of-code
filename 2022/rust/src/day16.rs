use regex::Regex;
use std::collections::{HashMap, HashSet};

type ValveName = String;

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    adjacencies: Vec<ValveName>,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let valves = parse(input);
    let distances: HashMap<ValveName, HashMap<ValveName, usize>> = valves
        .keys()
        .map(|name| (name.clone(), find_distances(name.clone(), &valves)))
        .collect();
    let closed_valves = valves
        .iter()
        .filter(|(_name, valve)| valve.flow_rate > 0)
        .map(|(name, _valve)| name)
        .collect();

    let paths_with_pressure = all_paths_with_pressure(
        &valves,
        &distances,
        closed_valves,
        &"AA".into(),
        vec!["AA".into()],
        30,
        0,
    );

    let (_path, pressure) = paths_with_pressure
        .into_iter()
        .max_by_key(|(_path, pressure)| pressure.clone())
        .unwrap();

    pressure
}

fn all_paths_with_pressure(
    valves: &HashMap<ValveName, Valve>,
    distances: &HashMap<ValveName, HashMap<ValveName, usize>>,
    closed_valves: HashSet<&ValveName>,
    current_name: &ValveName,
    path: Vec<ValveName>,
    time_left: usize,
    pressure_released: usize,
) -> Vec<(Vec<ValveName>, usize)> {
    //    println!(
    //        "at {}, time_left={}, pressure_released={}",
    //        &current_name, &time_left, &pressure_released
    //    );
    if time_left == 0 {
        return vec![(path, pressure_released)];
    }

    closed_valves
        .iter()
        .filter_map(|next_name| {
            let next = &valves[*next_name];
            let time_required = distances[current_name][*next_name] + 1;

            if time_required < time_left {
                let mut new_closed_valves = closed_valves.clone();
                new_closed_valves.remove(next_name);
                let mut new_path = path.clone();
                new_path.push(next_name.to_string());

                Some(all_paths_with_pressure(
                    valves,
                    distances,
                    new_closed_valves,
                    next_name,
                    new_path,
                    time_left - time_required,
                    pressure_released
                        + (time_left - time_required) * next.flow_rate,
                ))
            } else {
                None
            }
        })
        .collect()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let valves = parse(input);
    let distances: HashMap<ValveName, HashMap<ValveName, usize>> = valves
        .keys()
        .map(|name| (name.clone(), find_distances(name.clone(), &valves)))
        .collect();
    let closed_valves: HashSet<&ValveName> = valves
        .iter()
        .filter(|(_name, valve)| valve.flow_rate > 0)
        .map(|(name, _valve)| name)
        .collect();
    let start_valve_name: String = "AA".into();

    //    let (path1, pressure_released1) = maximal_pressure(
    //        &valves,
    //        &distances,
    //        closed_valves.clone(),
    //        &start_valve_name,
    //        vec![],
    //        26,
    //        0,
    //    );
    //
    //    dbg!(path1, pressure_released1);
    //
    //    let (path2, pressure_released2) = maximal_pressure(
    //        &valves,
    //        &distances,
    //        closed_valves,
    //        &start_valve_name,
    //        vec![],
    //        26,
    //        0,
    //    );
    //
    //    dbg!(path2, pressure_released2);
    //
    //    pressure_released1 + pressure_released2
    0
}

fn find_distances(
    start: ValveName,
    valves: &HashMap<ValveName, Valve>,
) -> HashMap<ValveName, usize> {
    let mut distances = HashMap::new();
    let mut distance = 0usize;
    let mut frontier: Vec<&ValveName> = vec![&start];

    while !&frontier.is_empty() {
        let mut new_frontier = vec![];
        for &name in &frontier {
            distances.insert(name.clone(), distance);

            for next in &valves[name].adjacencies {
                if !distances.contains_key(next) {
                    new_frontier.push(next);
                }
            }
        }

        frontier.clear();
        for name in new_frontier {
            frontier.push(name);
        }

        distance += 1;
    }

    distances
}

fn parse(input: &str) -> HashMap<ValveName, Valve> {
    let valve_regex = Regex::new(r"Valve (\w+) has flow rate=(\d+)").unwrap();
    let tunnel_regex = Regex::new(r"tunnels? leads? to valves? (.*)").unwrap();

    input
        .lines()
        .map(|line| {
            let (valve_string, tunnels_string) = line.split_once("; ").unwrap();
            let captures = valve_regex.captures(valve_string).unwrap();
            let name = captures[1].to_owned();
            let flow_rate = captures[2].parse::<usize>().unwrap();
            let adjacencies: Vec<String> =
                tunnel_regex.captures(tunnels_string).unwrap()[1]
                    .split(", ")
                    .map(|s| s.to_owned())
                    .collect();

            (
                name.clone(),
                Valve {
                    flow_rate,
                    adjacencies,
                },
            )
        })
        .collect::<HashMap<ValveName, Valve>>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 1651);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 1707)
    }
}
