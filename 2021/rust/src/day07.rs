const MAX_STEPS: usize = 1000;

#[must_use]
pub fn part1(input: &str) -> usize {
    let positions = parse(input);
    let median: usize = positions[positions.len() / 2];
    let linear_loss = |a: usize, b: usize| -> usize { (a as isize - b as isize).abs() as usize };

    total_fuel_burn(&positions, median, linear_loss)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let positions = parse(input);
    let quadratic_loss = |a: usize, b: usize| -> usize {
        let distance = (a as isize - b as isize).abs() as usize;
        (distance * distance + distance) / 2
    };
    let min_loss_position = minimize(&positions, quadratic_loss);

    total_fuel_burn(&positions, min_loss_position, quadratic_loss)
}

fn parse(input: &str) -> Vec<usize> {
    let mut positions: Vec<usize> = input
        .trim_end()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    positions.sort_unstable();
    positions
}

fn total_fuel_burn<F>(positions: &Vec<usize>, destination: usize, loss_function: F) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    positions
        .iter()
        .map(|&position| loss_function(position, destination))
        .sum()
}

fn minimize<F>(positions: &Vec<usize>, loss_function: F) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    let initial_position: usize = positions[positions.len() / 2];
    let initial_cost: usize = total_fuel_burn(positions, initial_position, &loss_function);

    minimize_step(
        positions,
        loss_function,
        initial_position,
        initial_cost,
        1,
        0,
    )
}

fn minimize_step<F>(
    positions: &Vec<usize>,
    loss_function: F,
    current_position: usize,
    current_cost: usize,
    delta: usize,
    steps: usize,
) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    if steps > MAX_STEPS {
        return 0;
    }

    let candidate_position: usize = current_position + delta;
    let candidate_cost: usize = total_fuel_burn(positions, candidate_position, &loss_function);

    println!(
        "step: {}  current: {}=>{}  candidate: {}=>{}",
        steps, candidate_position, current_cost, candidate_position, candidate_cost
    );

    if candidate_cost < current_cost {
        minimize_step(
            positions,
            loss_function,
            candidate_position,
            candidate_cost,
            1,
            steps + 1,
        )
    } else {
        current_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 37)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 168)
    }
}
