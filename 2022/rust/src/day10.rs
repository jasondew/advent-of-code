#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    fn cycle_count(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }

    fn execute(&self, x: isize) -> isize {
        match self {
            Instruction::Noop => x,
            Instruction::Addx(amount) => x + amount,
        }
    }
}

#[must_use]
pub fn part1(input: &str) -> isize {
    let instructions = parse(input);
    let mut signal_strengths: Vec<isize> = Vec::new();
    let mut cycle: usize = 1;
    let mut x: isize = 1;

    for instruction in instructions {
        for _ in 0..instruction.cycle_count() {
            if record_signal(cycle) {
                signal_strengths.push(cycle as isize * x);
            }
            cycle += 1;
        }

        x = instruction.execute(x);
    }

    signal_strengths.iter().sum()
}

#[must_use]
pub fn part2(input: &str) -> Vec<String> {
    let instructions = parse(input);
    let mut cycle: usize = 1;
    let mut x: isize = 1;
    let mut crt: Vec<Vec<char>> =
        vec![vec![], vec![], vec![], vec![], vec![], vec![]];

    for instruction in instructions {
        for _ in 0..instruction.cycle_count() {
            let row = (cycle - 1) / 40;
            let sprite = ((cycle - 1) % 40) as isize;
            crt[row].push(if (sprite - x).abs() <= 1 { '#' } else { '.' });

            cycle += 1;
        }

        x = instruction.execute(x);
    }

    crt.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect()
}

fn record_signal(cycle: usize) -> bool {
    if cycle < 30 {
        cycle == 20
    } else {
        (cycle - 20) % 40 == 0
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => Instruction::Noop,
            ["addx", value_string] => {
                Instruction::Addx(value_string.parse::<isize>().unwrap())
            }
            _ => panic!("invalid instruction"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 13140)
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(input()),
            vec![
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
        )
    }
}
