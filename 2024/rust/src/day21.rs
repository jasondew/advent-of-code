use std::{collections::HashMap, io::Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum KeypadButton {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    ZERO,
    APPLY,
}

impl std::fmt::Display for KeypadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use KeypadButton::*;

        match self {
            ONE => write!(f, "1"),
            TWO => write!(f, "2"),
            THREE => write!(f, "3"),
            FOUR => write!(f, "4"),
            FIVE => write!(f, "5"),
            SIX => write!(f, "6"),
            SEVEN => write!(f, "7"),
            EIGHT => write!(f, "8"),
            NINE => write!(f, "9"),
            ZERO => write!(f, "0"),
            APPLY => write!(f, "A"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct KeypadSequence(Vec<KeypadButton>);

impl From<String> for KeypadSequence {
    fn from(input: String) -> Self {
        Self(input.chars().map(|ch| KeypadButton::from(ch)).collect())
    }
}

impl From<&str> for KeypadSequence {
    fn from(input: &str) -> Self {
        Self(input.chars().map(|ch| KeypadButton::from(ch)).collect())
    }
}

impl Iterator for KeypadSequence {
    type Item = KeypadButton;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
}

impl From<char> for KeypadButton {
    fn from(ch: char) -> Self {
        match ch {
            '0' => KeypadButton::ZERO,
            '1' => KeypadButton::ONE,
            '2' => KeypadButton::TWO,
            '3' => KeypadButton::THREE,
            '4' => KeypadButton::FOUR,
            '5' => KeypadButton::FIVE,
            '6' => KeypadButton::SIX,
            '7' => KeypadButton::SEVEN,
            '8' => KeypadButton::EIGHT,
            '9' => KeypadButton::NINE,
            'A' => KeypadButton::APPLY,
            _ => panic!("Invalid keypad button: {}", ch),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DirectionButton {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    APPLY,
}

impl std::fmt::Display for DirectionButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DirectionButton::*;

        match self {
            UP => write!(f, "^"),
            DOWN => write!(f, "v"),
            LEFT => write!(f, "<"),
            RIGHT => write!(f, ">"),
            APPLY => write!(f, "A"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct DirectionSequence(Vec<DirectionButton>);

impl std::fmt::Display for DirectionSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|direction_button| format!("{}", direction_button))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl From<&str> for DirectionSequence {
    fn from(input: &str) -> Self {
        Self(
            input
                .chars()
                .map(|ch| match ch {
                    '^' => DirectionButton::UP,
                    'v' => DirectionButton::DOWN,
                    '<' => DirectionButton::LEFT,
                    '>' => DirectionButton::RIGHT,
                    'A' => DirectionButton::APPLY,
                    _ => panic!("Invalid direction button: {}", ch),
                })
                .collect(),
        )
    }
}

impl From<KeypadSequence> for DirectionSequence {
    fn from(keypad_sequence: KeypadSequence) -> Self {
        let mut current_button = KeypadButton::APPLY;
        let mut moves: Vec<DirectionButton> = Vec::new();

        for keypad_button in keypad_sequence.into_iter() {
            moves.append(&mut keypad_path(&current_button, &keypad_button));
            moves.push(DirectionButton::APPLY);
            current_button = keypad_button;
        }

        Self(moves)
    }
}

impl Iterator for DirectionSequence {
    type Item = DirectionButton;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
}

impl DirectionSequence {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let codes: Vec<String> = parse(input);

    //    let buttons = [
    //        KeypadButton::ZERO,
    //        KeypadButton::ONE,
    //        KeypadButton::TWO,
    //        KeypadButton::THREE,
    //        KeypadButton::FOUR,
    //        KeypadButton::FIVE,
    //        KeypadButton::SIX,
    //        KeypadButton::SEVEN,
    //        KeypadButton::EIGHT,
    //        KeypadButton::NINE,
    //        KeypadButton::APPLY,
    //    ];

    //    use itertools::Itertools;
    //
    //    for from_button in &buttons {
    //        for to_button in &buttons {
    //            let path = keypad_path(from_button, to_button);
    //            let baseline = foo(path.clone());
    //
    //            for iteration in path.iter().permutations(path.len()) {
    //                let test =
    //                    foo(iteration.clone().into_iter().cloned().collect());
    //                if test < baseline {
    //                    println!(
    //                        "{:?} to {:?} is shorter via {:?} ({} vs {})",
    //                        from_button, to_button, iteration, baseline, test
    //                    );
    //                }
    //            }
    //        }
    //    }

    codes
        .iter()
        .map(|code| {
            let encoded = encode(code);
            encoded.len() * code[0..3].parse::<usize>().unwrap()
        })
        .sum()
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[allow(dead_code)]
fn foo(mut moves: Vec<DirectionButton>, times: usize) -> usize {
    moves.push(DirectionButton::APPLY);
    let mut iteration = DirectionSequence(moves);

    for _ in 0..times {
        iteration = iterate(iteration);
    }

    iteration.len()
}

fn encode(code: &str) -> DirectionSequence {
    //    println!("encode: {}", code);

    let keypad_sequence = KeypadSequence::from(code);
    //    println!("keypad_sequence = {:?}", &keypad_sequence);

    let first_iteration = DirectionSequence::from(keypad_sequence);
    //    println!("first_iteration = {}", &first_iteration);

    let second_iteration = iterate(first_iteration);
    //    println!("second_iteration = {}", &second_iteration);

    let final_iteration = iterate(second_iteration);
    //    println!("final_iteration = {}", &final_iteration);

    final_iteration
}

fn encode_extra(
    code: &str,
    cache: &HashMap<KeypadSequence, usize>,
) -> DirectionSequence {
    let keypad_sequence = KeypadSequence::from(code);
    let mut iteration = DirectionSequence::from(keypad_sequence);

    for _ in 0..25 {
        iteration = iterate(iteration);
    }

    iteration
}

fn iterate(direction_sequence: DirectionSequence) -> DirectionSequence {
    let mut current_button = DirectionButton::APPLY;
    let mut moves: Vec<DirectionButton> = Vec::new();

    //    println!("{:?}\n", direction_sequence);

    for direction_button in direction_sequence.into_iter() {
        let mut path = direction_path(&current_button, &direction_button);

        //        println!("{:?} => {:?}: {:?}", current_button, direction_button, path);

        moves.append(&mut path);
        moves.push(DirectionButton::APPLY);

        current_button = direction_button;
    }

    //    println!("\n");

    DirectionSequence(moves)
}

fn keypad_path(from: &KeypadButton, to: &KeypadButton) -> Vec<DirectionButton> {
    use DirectionButton::{DOWN, LEFT, RIGHT, UP};
    use KeypadButton::*;

    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    match (from, to) {
        (ZERO, APPLY) => vec![RIGHT],
        (ZERO, ONE) => vec![UP, LEFT],
        (ZERO, TWO) => vec![UP],
        (ZERO, THREE) => vec![UP, RIGHT],
        (ZERO, FOUR) => vec![UP, UP, LEFT],
        (ZERO, FIVE) => vec![UP, UP],
        (ZERO, SIX) => vec![UP, UP, RIGHT],
        (ZERO, SEVEN) => vec![UP, UP, UP, LEFT],
        (ZERO, EIGHT) => vec![UP, UP, UP],
        (ZERO, NINE) => vec![UP, UP, UP, RIGHT],

        (APPLY, ZERO) => vec![LEFT],
        (APPLY, ONE) => vec![UP, LEFT, LEFT],
        (APPLY, TWO) => vec![LEFT, UP],
        (APPLY, THREE) => vec![UP],
        (APPLY, FOUR) => vec![UP, UP, LEFT, LEFT],
        (APPLY, FIVE) => vec![LEFT, UP, UP],
        (APPLY, SIX) => vec![UP, UP],
        (APPLY, SEVEN) => vec![UP, UP, UP, LEFT, LEFT],
        (APPLY, EIGHT) => vec![LEFT, UP, UP, UP],
        (APPLY, NINE) => vec![UP, UP, UP],

        (ONE, APPLY) => vec![DOWN, RIGHT, RIGHT],
        (ONE, ZERO) => vec![DOWN, RIGHT],
        (ONE, TWO) => vec![RIGHT],
        (ONE, THREE) => vec![RIGHT, RIGHT],
        (ONE, FOUR) => vec![UP],
        (ONE, FIVE) => vec![UP, RIGHT],
        (ONE, SIX) => vec![UP, RIGHT, RIGHT],
        (ONE, SEVEN) => vec![UP, UP],
        (ONE, EIGHT) => vec![UP, UP, RIGHT],
        (ONE, NINE) => vec![UP, UP, RIGHT, RIGHT],

        (TWO, APPLY) => vec![DOWN, RIGHT],
        (TWO, ZERO) => vec![DOWN],
        (TWO, ONE) => vec![LEFT],
        (TWO, THREE) => vec![RIGHT],
        (TWO, FOUR) => vec![LEFT, UP],
        (TWO, FIVE) => vec![UP],
        (TWO, SIX) => vec![UP, RIGHT],
        (TWO, SEVEN) => vec![LEFT, UP, UP],
        (TWO, EIGHT) => vec![UP, UP],
        (TWO, NINE) => vec![RIGHT, UP, UP],

        (THREE, APPLY) => vec![DOWN],
        (THREE, ZERO) => vec![LEFT, DOWN],
        (THREE, ONE) => vec![LEFT, LEFT],
        (THREE, TWO) => vec![LEFT],
        (THREE, FOUR) => vec![LEFT, LEFT, UP],
        (THREE, FIVE) => vec![LEFT, UP],
        (THREE, SIX) => vec![UP],
        (THREE, SEVEN) => vec![LEFT, LEFT, UP, UP],
        (THREE, EIGHT) => vec![LEFT, UP, UP],
        (THREE, NINE) => vec![UP, UP],

        (FOUR, APPLY) => vec![RIGHT, RIGHT, DOWN, DOWN],
        (FOUR, ZERO) => vec![DOWN, DOWN, RIGHT],
        (FOUR, ONE) => vec![DOWN],
        (FOUR, TWO) => vec![DOWN, RIGHT],
        (FOUR, THREE) => vec![DOWN, RIGHT, RIGHT],
        (FOUR, FIVE) => vec![RIGHT],
        (FOUR, SIX) => vec![RIGHT, RIGHT],
        (FOUR, SEVEN) => vec![UP],
        (FOUR, EIGHT) => vec![UP, RIGHT],
        (FOUR, NINE) => vec![UP, RIGHT, RIGHT],

        (FIVE, APPLY) => vec![DOWN, DOWN, RIGHT],
        (FIVE, ZERO) => vec![DOWN, DOWN],
        (FIVE, ONE) => vec![LEFT, DOWN],
        (FIVE, TWO) => vec![DOWN],
        (FIVE, THREE) => vec![DOWN, RIGHT],
        (FIVE, FOUR) => vec![LEFT],
        (FIVE, SIX) => vec![RIGHT],
        (FIVE, SEVEN) => vec![LEFT, UP],
        (FIVE, EIGHT) => vec![UP],
        (FIVE, NINE) => vec![RIGHT, UP],

        (SIX, APPLY) => vec![DOWN, DOWN],
        (SIX, ZERO) => vec![LEFT, DOWN, DOWN],
        (SIX, ONE) => vec![LEFT, LEFT, DOWN],
        (SIX, TWO) => vec![LEFT, DOWN],
        (SIX, THREE) => vec![DOWN],
        (SIX, FOUR) => vec![LEFT, LEFT],
        (SIX, FIVE) => vec![LEFT],
        (SIX, SEVEN) => vec![LEFT, LEFT, UP],
        (SIX, EIGHT) => vec![LEFT, UP],
        (SIX, NINE) => vec![UP],

        (SEVEN, APPLY) => vec![RIGHT, RIGHT, DOWN, DOWN, DOWN],
        (SEVEN, ZERO) => vec![RIGHT, DOWN, DOWN, DOWN],
        (SEVEN, ONE) => vec![DOWN, DOWN],
        (SEVEN, TWO) => vec![DOWN, DOWN, RIGHT],
        (SEVEN, THREE) => vec![DOWN, DOWN, RIGHT, RIGHT],
        (SEVEN, FOUR) => vec![DOWN],
        (SEVEN, FIVE) => vec![DOWN, RIGHT],
        (SEVEN, SIX) => vec![DOWN, RIGHT, RIGHT],
        (SEVEN, EIGHT) => vec![RIGHT],
        (SEVEN, NINE) => vec![RIGHT, RIGHT],

        (EIGHT, APPLY) => vec![DOWN, DOWN, DOWN, RIGHT],
        (EIGHT, ZERO) => vec![DOWN, DOWN, DOWN],
        (EIGHT, ONE) => vec![LEFT, DOWN, DOWN],
        (EIGHT, TWO) => vec![DOWN, DOWN],
        (EIGHT, THREE) => vec![DOWN, DOWN, RIGHT],
        (EIGHT, FOUR) => vec![LEFT, DOWN],
        (EIGHT, FIVE) => vec![DOWN],
        (EIGHT, SIX) => vec![DOWN, RIGHT],
        (EIGHT, SEVEN) => vec![LEFT],
        (EIGHT, NINE) => vec![RIGHT],

        (NINE, APPLY) => vec![DOWN, DOWN, DOWN],
        (NINE, ZERO) => vec![LEFT, DOWN, DOWN, DOWN],
        (NINE, ONE) => vec![LEFT, LEFT, DOWN, DOWN],
        (NINE, TWO) => vec![LEFT, DOWN, DOWN],
        (NINE, THREE) => vec![DOWN, DOWN],
        (NINE, FOUR) => vec![LEFT, LEFT, DOWN],
        (NINE, FIVE) => vec![LEFT, DOWN],
        (NINE, SIX) => vec![DOWN],
        (NINE, SEVEN) => vec![LEFT, LEFT],
        (NINE, EIGHT) => vec![LEFT],

        (from, to) if from == to => vec![],

        _ => panic!("Invalid transition from {:?} to {:?}", from, to),
    }
}

fn direction_path(
    from: &DirectionButton,
    to: &DirectionButton,
) -> Vec<DirectionButton> {
    use DirectionButton::*;

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+

    match (from, to) {
        (APPLY, UP) => vec![LEFT],
        (APPLY, DOWN) => vec![LEFT, DOWN],
        (APPLY, LEFT) => vec![DOWN, LEFT, LEFT],
        (APPLY, RIGHT) => vec![DOWN],

        (UP, APPLY) => vec![RIGHT],
        (UP, DOWN) => vec![DOWN],
        (UP, LEFT) => vec![DOWN, LEFT],
        (UP, RIGHT) => vec![DOWN, RIGHT],

        (LEFT, APPLY) => vec![RIGHT, RIGHT, UP],
        (LEFT, UP) => vec![RIGHT, UP],
        (LEFT, DOWN) => vec![RIGHT],
        (LEFT, RIGHT) => vec![RIGHT, RIGHT],

        (DOWN, APPLY) => vec![RIGHT, UP],
        (DOWN, UP) => vec![UP],
        (DOWN, LEFT) => vec![LEFT],
        (DOWN, RIGHT) => vec![RIGHT],

        (RIGHT, APPLY) => vec![UP],
        (RIGHT, UP) => vec![LEFT, UP],
        (RIGHT, DOWN) => vec![LEFT],
        (RIGHT, LEFT) => vec![LEFT, LEFT],

        (from, to) if from == to => vec![],

        _ => panic!("Invalid transition from {:?} to {:?}", from, to),
    }
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let codes: Vec<String> = parse(input);

    let buttons = [
        DirectionButton::UP,
        DirectionButton::DOWN,
        DirectionButton::LEFT,
        DirectionButton::RIGHT,
        DirectionButton::APPLY,
    ];

    let mut cache: HashMap<(DirectionButton, DirectionButton), usize> =
        HashMap::new();

    for from_button in &buttons {
        for to_button in &buttons {
            if from_button == to_button {
                continue;
            }

            let mut path = direction_path(from_button, to_button);
            path.push(DirectionButton::APPLY);
            let count = path.len();

            println!("{} -> {}: {} {:?}", from_button, to_button, count, path);
            cache.insert((from_button.clone(), to_button.clone()), count);
        }
    }

    //    codes
    //        .iter()
    //        .map(|code| {
    //            let encoded = encode_extra(code);
    //            encoded.len() * code[0..3].parse::<usize>().unwrap()
    //        })
    //        .sum()
    0
}

fn encoding_length(
    from_button: &DirectionButton,
    to_button: &DirectionButton,
    iterations: usize,
) -> usize {
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
029A
980A
179A
456A
379A\n"
    }

    #[test]
    fn part1_example() {
        //        assert_eq!(part1(input()), 0);
        assert_eq!(
            DirectionSequence::from(KeypadSequence::from("029A")),
            "<A^A>^^AvvvA".into()
        );
        assert_eq!(
            iterate("<A^A>^^AvvvA".into()),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".into()
        );
        assert_eq!(
            iterate(iterate(DirectionSequence::from(KeypadSequence::from("029A")))),
            DirectionSequence::from("<vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A")
        );

        assert_eq!(
            encode("980A"),
            "v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A"
                .into()
        );
        assert_eq!(encode("179A"), "v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A".into());

        assert_eq!(
            encode("456A"),
            "v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A"
                .into()
        );

        assert_eq!(
            encode("379A"),
            "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A"
                .into()
        );

        assert_eq!(part1(input()), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 5);
    }
}
