#[derive(Debug)]
enum ParseResult {
    Valid,
    Incomplete(Vec<char>),
    Corrupted(char),
}

#[must_use]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            use ParseResult::*;
            let parse_result: ParseResult = parse(line);
            match parse_result {
                Valid | Incomplete(_) => 0,
                Corrupted(')') => 3,
                Corrupted(']') => 57,
                Corrupted('}') => 1197,
                Corrupted('>') => 25137,
                Corrupted(_) => panic!("invalid character"),
            }
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .filter_map(|line| autocomplete_score(parse(line)))
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn autocomplete_score(parse_result: ParseResult) -> Option<usize> {
    use ParseResult::*;
    match parse_result {
        Valid | Corrupted(_) => None,
        Incomplete(missing) => {
            let mut score: usize = 0;

            for &ch in missing.iter().rev() {
                score = score * 5 + value(ch)
            }

            Some(score)
        }
    }
}

fn value(ch: char) -> usize {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("invalid character: {}", ch),
    }
}

fn parse(line: &str) -> ParseResult {
    let mut stack: Vec<char> = vec![];

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(opening) => {
                    if !matching(opening, ch) {
                        return ParseResult::Corrupted(ch);
                    }
                }
                None => return ParseResult::Incomplete(stack),
            },
            _ => panic!("invalid input"),
        }
    }

    if stack.len() == 0 {
        ParseResult::Valid
    } else {
        ParseResult::Incomplete(stack)
    }
}

fn matching(opening: char, closing: char) -> bool {
    match (opening, closing) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 26397)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 288957)
    }
}
