#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let races = parse(input);
    let mut result = 1;

    for race in races {
        result *= win_count(race.time, race.record);
    }

    result
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let races = parse(input);
    let time: usize = combine(races.iter().map(|r| r.time).collect());
    let record: usize = combine(races.iter().map(|r| r.record).collect());

    win_count(time, record)
}

fn win_count(time: usize, record: usize) -> usize {
    // distance = time*speed - speed^2
    // 0 = speed^2 - time*speed + distance
    // speed = (-b +/- sqrt(b^2 - 4ac)) / 2a
    // speed = (time +/- sqrt(time^2 - 4*distance)) / 2

    let discriminant = ((time * time - 4 * record) as f64).sqrt();
    let low = ((time as f64 - discriminant) / 2f64).floor() as usize;
    let high = ((time as f64 + discriminant) / 2f64).ceil() as usize;

    high - low - 1
}

fn combine(values: Vec<usize>) -> usize {
    values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn parse(input: &str) -> Vec<Race> {
    let mut line_iter = input.lines();

    let times: Vec<usize> = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let records: Vec<usize> = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    times
        .into_iter()
        .zip(records.into_iter())
        .map(|(time, record)| Race { time, record })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Time:      7  15   30
Distance:  9  40  200"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 288)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 71503)
    }
}
