use std::fmt::Debug;

struct Position(usize, usize);

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Game {
    a: Position,
    b: Position,
    prize: Position,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let games = parse(input, 0);

    games
        .iter()
        .map(|game| match minimum_tokens_to_win(&game) {
            Some(tokens) => tokens,
            None => 0,
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let games = parse(input, 10000000000000);

    games
        .iter()
        .map(|game| match minimum_tokens_to_win(&game) {
            Some(tokens) => tokens,
            None => 0,
        })
        .sum()
}

fn minimum_tokens_to_win(game: &Game) -> Option<usize> {
    // (1) PX = AX*a + BX*b
    // (2) PY = AY*a + BY*b
    //
    // want: min[3a + b]

    let ax = game.a.0 as isize;
    let ay = game.a.1 as isize;
    let bx = game.b.0 as isize;
    let by = game.b.1 as isize;
    let px = game.prize.0 as isize;
    let py = game.prize.1 as isize;

    let b = (ay * px - ax * py) / (bx * ay - by * ax);
    let a = (px - b * bx) / ax;

    let px_computed = a * ax + b * bx;
    let py_computed = a * ay + b * by;

    if (px as isize == px_computed) && (py as isize == py_computed) {
        Some((3 * a + b) as usize)
    } else {
        None
    }
}

fn parse(input: &str, add: usize) -> Vec<Game> {
    input
        .split("\n\n")
        .map(|game_string| {
            let mut lines = game_string.lines();
            Game {
                a: parse_position(lines.next().unwrap(), 0),
                b: parse_position(lines.next().unwrap(), 0),
                prize: parse_position(lines.next().unwrap(), add),
            }
        })
        .collect()
}

fn parse_position(input: &str, add: usize) -> Position {
    let position_string = input.split(": ").skip(1).next().unwrap();
    let mut iter = position_string.split(", ");

    Position(
        parse_coordinate(iter.next().unwrap()) + add,
        parse_coordinate(iter.next().unwrap()) + add,
    )
}

fn parse_coordinate(input: &str) -> usize {
    parse_usize(input.split(&['+', '=']).skip(1).next().unwrap())
}

fn parse_usize(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 480)
    }
}
