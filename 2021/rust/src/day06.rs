struct Lanternfish {
    timer: usize,
}

impl Default for Lanternfish {
    fn default() -> Self {
        Self { timer: 8 }
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    spawn_count(input, 80)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    spawn_count(input, 256)
}

fn spawn_count(input: &str, days: usize) -> usize {
    let lanternfish = parse(input);
    let mut lanternfish_by_age = [0usize; 9];
    let mut spawns;

    for fish in lanternfish {
        lanternfish_by_age[fish.timer] += 1
    }

    for _day in 1..=days {
        spawns = lanternfish_by_age[0];

        for age in 1..=8 {
            lanternfish_by_age[age - 1] = lanternfish_by_age[age]
        }

        lanternfish_by_age[6] += spawns;
        lanternfish_by_age[8] = spawns;
    }

    lanternfish_by_age.iter().sum()
}

#[allow(dead_code)]
fn spawn_count_brute_force(input: &str, days: usize) -> usize {
    let mut lanternfish = parse(input);
    let mut spawns = 0;

    for _day in 0..days {
        for mut fish in &mut lanternfish {
            if fish.timer == 0 {
                fish.timer = 6;
                spawns += 1;
            } else {
                fish.timer -= 1;
            }
        }

        for _ in 1..=spawns {
            lanternfish.push(Lanternfish::default());
        }

        spawns = 0;
    }

    lanternfish.len()
}

fn parse(input: &str) -> Vec<Lanternfish> {
    input
        .trim_end()
        .split(",")
        .map(|string| Lanternfish {
            timer: string.parse().unwrap(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "3,4,3,1,2\n"
    }

    #[test]
    fn examples() {
        assert_eq!(spawn_count(input(), 18), 26);
        assert_eq!(spawn_count(input(), 80), 5934);
        assert_eq!(spawn_count(input(), 256), 26984457539);
    }
}
