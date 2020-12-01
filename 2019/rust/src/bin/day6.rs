use core::cell::Cell;
use std::collections::HashMap;

type Label = String;
type Orbits = Vec<(Label, Label)>;

#[derive(Debug)]
struct OrbitData<'a> {
    orbiters: Vec<&'a Label>,
    depth: Cell<i32>,
}
type Map<'a> = HashMap<&'a Label, OrbitData<'a>>;

impl<'a> OrbitData<'a> {
    fn new(orbiters: Vec<&'a Label>) -> Self {
        Self {
            orbiters,
            depth: Cell::new(0),
        }
    }
}

fn part1(orbits: &Orbits) -> i32 {
    let mut map: Map = HashMap::new();

    for (orbitee, orbiter) in orbits {
        match map.get_mut(orbitee) {
            Some(OrbitData { orbiters, depth: _ }) => orbiters.push(orbiter),
            None => {
                map.insert(orbitee, OrbitData::new(vec![orbiter]));
                ()
            }
        }
    }

    compute_depth(&map);

    let mut count: i32 = 0;
    for (_orbitee, orbit_data) in &map {
        count += orbit_data.depth.get()
    }

    count
}

fn compute_depth(map: &Map) {
    let com: &OrbitData = map
        .get(&String::from("COM"))
        .expect("COM has to be present in valid inputs");

    update_depth(map, com);
}

fn update_depth(_map: &Map, orbit_data: &OrbitData) {
    orbit_data.depth.set(orbit_data.depth.get() + 1)
}

fn part2(_input: &Orbits) -> i32 {
    42
}

fn parse(input: &String) -> Vec<(Label, Label)> {
    input
        .lines()
        .map(|line| {
            let vec: Vec<&str> = line.split(")").collect();
            assert!(vec.len() == 2);
            (vec[0].to_owned(), vec[1].to_owned())
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let input: String = std::fs::read_to_string("../inputs/06")?;
    let orbits: Orbits = parse(&input);

    println!("Part 1: {}", part1(&orbits));
    println!("Part 2: {}", part2(&orbits));

    Ok(())
}
