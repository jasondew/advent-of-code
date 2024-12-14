use std::collections::HashMap;

type Plant = char;
type Position = (usize, usize);
type PlotMap = HashMap<Plant, Vec<Position>>;
type PositionMap = HashMap<Position, Plant>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (plot_map, position_map) = parse(input);
    let contiguous_plots: Vec<(char, Vec<Position>)> = segment(&plot_map);

    contiguous_plots
        .into_iter()
        .map(|(plant, plant_positions)| {
            let perimeter = perimeter(&plant, &plant_positions, &position_map);
            let area = plant_positions.len();

            perimeter * area
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (plot_map, position_map) = parse(input);
    let contiguous_plots: Vec<(char, Vec<Position>)> = segment(&plot_map);

    contiguous_plots
        .into_iter()
        .map(|(plant, plant_positions)| {
            let sides = sides(&plant, &plant_positions, &position_map);
            let area = plant_positions.len();

            sides * area
        })
        .sum()
}

fn sides(
    plant: &Plant,
    plant_positions: &Vec<Position>,
    position_map: &PositionMap,
) -> usize {
    let mut sides: usize = 0;

    for position in plant_positions {
        sides += neighboring_plants(position, position_map)
            .into_iter()
            .filter(|neighbor_plant| *neighbor_plant != Some(*plant))
            .count();
    }

    sides
}

fn segment(plot_map: &PlotMap) -> Vec<(char, Vec<Position>)> {
    let mut segmented_plots = Vec::new();

    for (plant, plot_positions) in plot_map {
        for segmented_positions in
            segment_plot(plot_positions.clone(), Vec::new())
        {
            segmented_plots.push((*plant, segmented_positions));
        }
    }

    segmented_plots
}

fn segment_plot(
    mut positions: Vec<Position>,
    mut plots: Vec<Vec<Position>>,
) -> Vec<Vec<Position>> {
    if positions.is_empty() {
        plots
    } else {
        let mut plot: Vec<Position> = Vec::new();
        let mut frontier: Vec<Position> = vec![positions.pop().unwrap()];
        let mut next_frontier: Vec<Position>;

        while !&frontier.is_empty() {
            next_frontier = Vec::new();
            for position in &frontier {
                plot.push(*position);
                positions.retain(|other_position| {
                    if is_neighboring(&position, other_position) {
                        next_frontier.push(*other_position);
                        false
                    } else {
                        true
                    }
                })
            }

            frontier = next_frontier;
        }
        plots.push(plot);

        segment_plot(positions, plots)
    }
}

fn is_neighboring(a: &Position, b: &Position) -> bool {
    let row_neighbor = a.0 == b.0;
    let col_neighbor = a.1 == b.1;

    (row_neighbor && a.1.abs_diff(b.1) == 1)
        || (col_neighbor && a.0.abs_diff(b.0) == 1)
}

fn perimeter(
    plant: &Plant,
    plant_positions: &Vec<Position>,
    position_map: &PositionMap,
) -> usize {
    let mut perimeter: usize = 0;

    for position in plant_positions {
        perimeter += neighboring_plants(position, position_map)
            .into_iter()
            .filter(|neighbor_plant| *neighbor_plant != Some(*plant))
            .count();
    }

    perimeter
}

fn neighboring_plants(
    position: &Position,
    position_map: &PositionMap,
) -> Vec<Option<Plant>> {
    let (row, col) = position;
    let neighboring_positions: Vec<Option<Position>> = vec![
        row.checked_sub(1).map(|r| (r, *col)),
        col.checked_sub(1).map(|c| (*row, c)),
        Some((row + 1, *col)),
        Some((*row, col + 1)),
    ];

    neighboring_positions
        .into_iter()
        .map(|maybe_position| {
            maybe_position.and_then(|position| {
                position_map.get(&position).map(|plant| *plant)
            })
        })
        .collect()
}

fn parse(input: &str) -> (PlotMap, PositionMap) {
    let mut plots: HashMap<Plant, Vec<Position>> = HashMap::new();
    let mut positions: HashMap<Position, Plant> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, plant) in line.chars().enumerate() {
            plots.entry(plant).or_insert(Vec::new()).push((row, col));
            positions.insert((row, col), plant);
        }
    }

    (plots, positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_input() -> &'static str {
        "\
AAAA
BBCD
BBCC
EEEC"
    }

    fn enclosed_input() -> &'static str {
        "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
    }

    fn large_input() -> &'static str {
        "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
    }

    fn e_input() -> &'static str {
        "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
    }

    fn mobius_input() -> &'static str {
        "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
    }

    #[test]
    fn is_neighboring_test() {
        assert!(is_neighboring(&(0, 0), &(0, 1)));
        assert!(!is_neighboring(&(1, 1), &(1, 3)));
        assert!(!is_neighboring(&(0, 6), &(4, 7)));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(small_input()), 140);
        assert_eq!(part1(enclosed_input()), 772);
        assert_eq!(part1(large_input()), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(small_input()), 80);
        assert_eq!(part2(enclosed_input()), 436);
        assert_eq!(part2(e_input()), 236);
        assert_eq!(part2(mobius_input()), 368);
        assert_eq!(part2(large_input()), 1206);
    }
}
