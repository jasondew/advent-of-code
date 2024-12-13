use std::collections::HashMap;

type Plant = char;
type Position = (usize, usize);
type PlotMap = HashMap<Plant, Vec<Position>>;
type PositionMap = HashMap<Position, Plant>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (plot_map, position_map) = parse(input);
    let contiguous_plots: Vec<(char, Vec<Position>)> = separate(&plot_map);

    //    dbg!(&contiguous_plots);

    contiguous_plots
        .into_iter()
        .map(|(plant, plant_positions)| {
            let perimeter = perimeter(&plant, &plant_positions, &position_map);
            let area = plant_positions.len();

            dbg!(&plant, perimeter, area);

            perimeter * area
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.lines().count()
}

fn separate(plot_map: &PlotMap) -> Vec<(char, Vec<Position>)> {
    let mut contiguous_plots = Vec::new();

    for (plant, positions) in plot_map {
        let mut plots: Vec<Vec<Position>> = Vec::new();

        for position in positions {
            match plots.iter_mut().find(|plot_positions| {
                plot_positions.iter().any(|plot_position| {
                    is_neighboring(plot_position, position)
                })
            }) {
                Some(plot) => {
                    plot.push(*position);
                }
                None => {
                    plots.push(vec![*position]);
                }
            }
            if *plant == 'I' {
                dbg!(plant, position, &plots);
            }
        }

        if *plant == 'I' {
            dbg!(&plots);
        }

        let mut combined_plots: Vec<Vec<Position>> = Vec::new();
        let mut skip_next = false;
        let mut merging: bool = true;

        while merging {
            merging = false;
            for a_index in 0..plots.len() {
                if skip_next {
                    skip_next = false
                } else {
                    let plot_a = plots.get(a_index).unwrap();
                    if a_index == plots.len() - 1 {
                        combined_plots.push(plot_a.clone());
                    } else {
                        for b_index in a_index + 1..plots.len() {
                            let plot_b = plots.get(b_index).unwrap();

                            if plot_a.iter().any(|a_position| {
                                plot_b.iter().any(|b_position| {
                                    is_neighboring(a_position, b_position)
                                })
                            }) {
                                let mut combined_plot = plot_a.clone();
                                combined_plot.append(&mut plot_b.clone());
                                combined_plots.push(combined_plot);
                                merging = true;
                                skip_next = true;

                                break;
                            }
                        }
                        if !skip_next {
                            combined_plots.push(plot_a.clone());
                        }
                    }
                }
            }
            plots = combined_plots;
            combined_plots = Vec::new();
        }

        if *plant == 'I' {
            dbg!(&plots);
        }

        for plot in plots {
            contiguous_plots.push((*plant, plot))
        }
    }

    contiguous_plots
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

    #[test]
    fn is_neighboring_test() {
        assert!(is_neighboring(&(0, 0), &(0, 1)));
        assert!(!is_neighboring(&(1, 1), &(1, 3)));
        assert!(!is_neighboring(&(0, 6), &(4, 7)));
    }

    #[test]
    fn part1_example() {
        //        assert_eq!(part1(small_input()), 140);
        //        assert_eq!(part1(enclosed_input()), 772);
        assert_eq!(part1(large_input()), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(large_input()), 10);
    }
}
