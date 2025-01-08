use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashMap};

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type Path = Vec<Position>;
type Map = HashMap<Position, usize>;

#[must_use]
pub fn part1(input: &str) -> usize {
    part1_with_params(input, 70, 1024)
}

pub fn part1_with_params(input: &str, bounds: usize, time: usize) -> usize {
    let map: Map = parse(input);
    let start_position = Position { x: 0, y: 0 };
    let target_position = Position {
        x: bounds,
        y: bounds,
    };
    let path =
        shortest_path(&start_position, &target_position, &map, bounds, time);

    //    print(&map, &path, bounds, time);

    path.len() - 1
}

#[must_use]
pub fn part2(input: &str) -> String {
    part2_with_params(input, 70)
}

pub fn part2_with_params(input: &str, bounds: usize) -> String {
    let map: Map = parse(input);
    let start_position = Position { x: 0, y: 0 };
    let target_position = Position {
        x: bounds,
        y: bounds,
    };

    let blocked_time: usize = find_blocked_time(
        &start_position,
        &target_position,
        &map,
        bounds,
        0,
        map.len(),
    );

    let (position, _time) = map
        .into_iter()
        .find(|(_position, fall_time)| *fall_time == blocked_time - 1)
        .unwrap();

    format!("{},{}", position.x, position.y)
}

fn find_blocked_time(
    start_position: &Position,
    target_position: &Position,
    map: &Map,
    bounds: usize,
    low_time: usize,
    high_time: usize,
) -> usize {
    if high_time - low_time <= 1 {
        return high_time;
    }

    let time: usize = (high_time + low_time) / 2;
    let path =
        shortest_path(&start_position, &target_position, &map, bounds, time);

    if path.first() != Some(&start_position) {
        find_blocked_time(
            start_position,
            target_position,
            map,
            bounds,
            low_time,
            time,
        )
    } else {
        find_blocked_time(
            start_position,
            target_position,
            map,
            bounds,
            time,
            high_time,
        )
    }
}

fn shortest_path(
    start_position: &Position,
    target_position: &Position,
    map: &Map,
    bounds: usize,
    time: usize,
) -> Path {
    let mut frontier: PriorityQueue<Position, Reverse<usize>> =
        PriorityQueue::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut cost_so_far: HashMap<Position, usize> = HashMap::new();

    frontier.push(start_position.clone(), Reverse(0));
    cost_so_far.insert(start_position.clone(), 0);

    while !frontier.is_empty() {
        let (current, _priority) = frontier.pop().unwrap();
        //        let time = *cost_so_far.get(&current).unwrap();

        if current == *target_position {
            break;
        }

        //    let path = calculate_path(&came_from, &current);
        //    print(&map, &path, bounds, time);

        for next in neighbors(&current, map, bounds, time) {
            let cost = cost_so_far.get(&current).unwrap();
            let new_cost = cost + 1;
            let maybe_cost = cost_so_far.get(&next);

            if maybe_cost.is_none()
                || maybe_cost.map_or(false, |cost| new_cost < *cost)
            {
                cost_so_far.insert(next.clone(), new_cost);
                frontier.push(next.clone(), Reverse(new_cost));
                came_from.insert(next.clone(), current.clone());
            }
        }
    }

    calculate_path(&came_from, &target_position)
}

fn calculate_path(
    came_from: &HashMap<Position, Position>,
    target_position: &Position,
) -> Vec<Position> {
    let mut path: Vec<Position> = vec![target_position.clone()];
    let mut position = target_position;

    while let Some(previous_position) = came_from.get(&position) {
        path.push(previous_position.clone());
        position = previous_position;
    }

    path.into_iter().rev().collect()
}

fn neighbors(
    position: &Position,
    map: &Map,
    bounds: usize,
    time: usize,
) -> Vec<Position> {
    vec![
        position.x.checked_sub(1).map(|x| (x, position.y)),
        position.y.checked_sub(1).map(|y| (position.x, y)),
        Some((position.x + 1, position.y)),
        Some((position.x, position.y + 1)),
    ]
    .into_iter()
    .filter_map(|maybe_position| maybe_position.map(|(x, y)| Position { x, y }))
    .filter(|position| {
        position.x <= bounds
            && position.y <= bounds
            && map
                .get(&position)
                .map_or(true, |fall_time| time <= *fall_time)
    })
    .collect()
}

#[allow(dead_code)]
fn print(map: &Map, path: &Path, bounds: usize, time: usize) {
    println!("time={}", time);

    for y in 0..=bounds {
        for x in 0..=bounds {
            if path.contains(&Position { x, y }) {
                print!(" () ")
            } else if let Some(fall_time) = map.get(&Position { x, y }) {
                if time < *fall_time {
                    print!(" .. ")
                } else {
                    print!("{:03} ", fall_time)
                }
            } else {
                print!(" .. ")
            }
        }
        println!();
    }
}

fn parse(input: &str) -> Map {
    let mut map: Map = HashMap::new();

    for (time, line) in input.trim_end().lines().enumerate() {
        let (x_string, y_string) =
            line.split_once(",").expect("x,y coordinate");
        let x = x_string.parse::<usize>().expect("integral x coordinate");
        let y = y_string.parse::<usize>().expect("integral x coordinate");

        map.insert(Position { x, y }, time);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1_with_params(input(), 6, 12), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_with_params(input(), 6), "6,1");
    }
}
