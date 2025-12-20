use itertools::Itertools;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize};

#[derive(Clone, Debug, PartialEq)]
enum State {
    Off,
    On,
}

type Button = Vec<usize>;

#[derive(Debug)]
struct Machine {
    target_state: Vec<State>,
    buttons: Vec<Button>,
    target_joltage: Vec<usize>,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let machines = parse(input);

    machines
        .iter()
        .map(|machine| {
            find_minimum_button_presses(&machine.target_state, &machine.buttons)
                .len()
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let machines = parse(input);

    machines
        .iter()
        .map(|machine| {
            minimum_presses_z3(&machine.target_joltage, &machine.buttons)
        })
        .sum()
}

fn minimum_presses_z3(target: &[usize], buttons: &[Vec<usize>]) -> usize {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    let button_vars: Vec<Int> = buttons
        .iter()
        .map(|button| Int::new_const(&ctx, format!("{button:?}")))
        .collect();

    for var in &button_vars {
        opt.assert(&var.ge(&Int::from_i64(&ctx, 0)));
    }

    for (position, &target_value) in target.iter().enumerate() {
        let mut sum = Int::from_i64(&ctx, 0);

        for (index, button) in buttons.iter().enumerate() {
            if button.contains(&position) {
                sum += &button_vars[index];
            }
        }

        opt.assert(&sum._eq(&Int::from_u64(&ctx, target_value as u64)));
    }

    // Minimize the total number of button presses
    let total: Int = button_vars
        .iter()
        .fold(Int::from_i64(&ctx, 0), |total, count| total + count);
    opt.minimize(&total);

    // Solve
    opt.check(&[]);

    // Extract solution
    let model = opt.get_model().unwrap();
    button_vars
        .iter()
        .map(|var| {
            usize::try_from(model.eval(var, true).unwrap().as_u64().unwrap())
                .unwrap()
        })
        .sum()
}

fn find_minimum_button_presses<'a>(
    target_state: &'a [State],
    buttons: &'a [Button],
) -> Vec<&'a Button> {
    let target_value =
        target_state
            .iter()
            .enumerate()
            .fold(0usize, |acc, (i, state)| match state {
                State::On => acc | (1 << i),
                State::Off => acc,
            });
    let button_values = buttons
        .iter()
        .map(|button| {
            button.iter().fold(0usize, |acc, &index| acc | (1 << index))
        })
        .collect::<Vec<usize>>();
    let mut optimal_button_presses = Vec::new();

    let n = buttons.len();
    for combination in 1..(1 << n) {
        let mut combined_value = 0usize;
        let mut button_presses = Vec::new();

        for bit_position in 0..n {
            if (combination & (1 << bit_position)) != 0 {
                button_presses.push(&buttons[bit_position]);
                combined_value ^= button_values[bit_position];
            }
        }
        if combined_value == target_value
            && (optimal_button_presses.is_empty()
                || button_presses.len() < optimal_button_presses.len())
        {
            optimal_button_presses.clone_from(&button_presses);
        }
    }

    optimal_button_presses
}

// Solve part 2 via brute-force search (too slow)
#[allow(dead_code)]
fn minimum_presses(target_joltages: &[usize], button_list: &[Button]) -> usize {
    let n = target_joltages.len();
    let button_values = button_list
        .iter()
        .map(|button| {
            let mut value = vec![0usize; n];
            for index in button {
                value[*index] = 1;
            }
            value
        })
        .collect::<Vec<Vec<usize>>>();

    let multiple_button_values: Vec<Vec<Vec<usize>>> = button_values
        .iter()
        .map(|button_value| {
            let max_multiples = target_joltages
                .iter()
                .zip(button_value.iter())
                .map(|(&target, &button)| {
                    if button == 0 {
                        usize::MAX
                    } else {
                        target / button
                    }
                })
                .min()
                .unwrap();

            (0..=max_multiples)
                .rev()
                .map(|multiple| {
                    button_value
                        .iter()
                        .map(|&v| v * multiple)
                        .collect::<Vec<usize>>()
                })
                .collect()
        })
        .collect();

    let mut minimum_presses = usize::MAX;

    for combination in multiple_button_values.iter().multi_cartesian_product() {
        let sum = sum_joltages(&combination);

        if sum == target_joltages {
            let presses = combination
                .iter()
                .map(|value| value.iter().max().unwrap())
                .sum();
            if presses < minimum_presses {
                minimum_presses = presses;
            }
        }
    }

    minimum_presses
}

#[allow(dead_code)]
fn sum_joltages(joltages: &[&Vec<usize>]) -> Vec<usize> {
    let length = joltages[0].len();

    joltages
        .iter()
        .fold(vec![0usize; length], |joltages, button| {
            joltages
                .iter()
                .zip(button.iter())
                .map(|(&a, &b)| a + b)
                .collect::<Vec<usize>>()
        })
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let lights_str = parts[0];
        let buttons_strs = &parts[1..parts.len() - 1];
        let joltages_str = parts.last().unwrap();

        let target_state: Vec<State> = lights_str
            .trim_matches(&['[', ']'][..])
            .chars()
            .map(|c| if c == '#' { State::On } else { State::Off })
            .collect();

        let buttons: Vec<Vec<usize>> = buttons_strs
            .iter()
            .map(|&s| {
                s.trim_matches(&['(', ')'][..])
                    .split(',')
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let target_joltage: Vec<usize> = joltages_str
            .trim_matches(&['{', '}'][..])
            .split(',')
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();

        machines.push(Machine {
            target_state,
            buttons,
            target_joltage,
        });
    }

    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";

    #[test]
    fn test_example_input() {
        let machine1 = Machine {
            target_state: vec![State::Off, State::On, State::On, State::Off],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            target_joltage: vec![3, 5, 4, 7],
        };
        let machine2 = Machine {
            target_state: vec![
                State::Off,
                State::Off,
                State::Off,
                State::On,
                State::Off,
            ],
            buttons: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
            target_joltage: vec![7, 5, 12, 7, 2],
        };
        let machine3 = Machine {
            target_state: vec![
                State::Off,
                State::On,
                State::On,
                State::On,
                State::Off,
                State::On,
            ],
            buttons: vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 3, 4],
                vec![0, 1, 2, 4, 5],
                vec![1, 2],
            ],
            target_joltage: vec![10, 11, 11, 5, 10, 5],
        };

        assert_eq!(
            find_minimum_button_presses(
                &machine1.target_state,
                &machine1.buttons
            )
            .len(),
            2
        );
        assert_eq!(
            find_minimum_button_presses(
                &machine2.target_state,
                &machine2.buttons
            )
            .len(),
            3
        );
        assert_eq!(
            find_minimum_button_presses(
                &machine3.target_state,
                &machine3.buttons
            )
            .len(),
            2
        );

        assert_eq!(part1(EXAMPLE_INPUT), 7);

        assert_eq!(
            minimum_presses(&machine1.target_joltage, &machine1.buttons),
            10
        );
        assert_eq!(
            minimum_presses(&machine2.target_joltage, &machine2.buttons),
            12
        );
        assert_eq!(
            minimum_presses(&machine3.target_joltage, &machine3.buttons),
            11
        );

        assert_eq!(part2(EXAMPLE_INPUT), 33);
    }

    #[test]
    fn test_second_real_input() {
        let input = "\
[###.#...] (3,4,6) (1,2,3,4,5,7) (0,1,2,4) (0,1,3,5,7) (0,1,4,5,6,7) (0,2,7) (4,6) (7) (0,6) (1,4,5,6,7) {214,221,33,36,226,204,203,217}
        ";
        assert_eq!(part2(input), 252);
    }
}
