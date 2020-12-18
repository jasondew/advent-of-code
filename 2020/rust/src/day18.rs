#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Times,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(usize),
    Operator(Op),
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let tokens: Vec<Token> = input
        .chars()
        .filter_map(|ch| match ch {
            '+' => Some(Token::Operator(Op::Plus)),
            '*' => Some(Token::Operator(Op::Times)),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '0' => Some(Token::Number(0)),
            '1' => Some(Token::Number(1)),
            '2' => Some(Token::Number(2)),
            '3' => Some(Token::Number(3)),
            '4' => Some(Token::Number(4)),
            '5' => Some(Token::Number(5)),
            '6' => Some(Token::Number(6)),
            '7' => Some(Token::Number(7)),
            '8' => Some(Token::Number(8)),
            '9' => Some(Token::Number(9)),
            _ => None,
        })
        .collect();

    tokens
}

fn evaluate_math(tokens: &[Token]) -> usize {
    let (_index, result) = evaluate_expression(tokens, 0);
    result
}

fn evaluate_expression(tokens: &[Token], starting_index: usize) -> (usize, usize) {
    let mut index: usize = starting_index;
    let mut op_stack: Vec<&Op> = Vec::new();
    let mut result: Option<usize> = None;

    while let Some(token) = &tokens.get(index) {
        match token {
            Token::LParen => {
                let (new_index, inner_result) = evaluate_expression(&tokens, index + 1);

                index = new_index;
                result = if let Some(op) = op_stack.pop() {
                    Some(evaluate_op(op, result.unwrap(), inner_result))
                } else {
                    Some(inner_result)
                };
            }
            Token::RParen => break,
            Token::Operator(op) => op_stack.push(op),
            Token::Number(number) => match result {
                Some(lhs) => {
                    if let Some(op) = op_stack.pop() {
                        result = Some(evaluate_op(op, lhs, *number))
                    } else {
                        panic!(
                            "number without an operation on the stack! number={} index={}",
                            number, index
                        )
                    }
                }
                None => result = Some(*number),
            },
        }
        index += 1;
    }

    (index, result.unwrap())
}

fn evaluate_op(op: &Op, lhs: usize, rhs: usize) -> usize {
    match op {
        Op::Plus => lhs + rhs,
        Op::Times => lhs * rhs,
    }
}

fn evaluate_advanced_math(tokens: &[Token]) -> usize {
    let (_index, result) = evaluate_advanced_expression(tokens, 0);
    result
}

fn evaluate_advanced_expression(tokens: &[Token], starting_index: usize) -> (usize, usize) {
    let mut index: usize = starting_index;
    let mut stack: Vec<Token> = Vec::new();

    while let Some(token) = &tokens.get(index) {
        match token {
            Token::LParen => {
                let (new_index, inner_result) = evaluate_advanced_expression(&tokens, index + 1);

                stack.push(Token::Number(inner_result));
                index = new_index;
            }
            Token::RParen => break,
            _ => stack.push(*token.clone()),
        }
        index += 1;
    }

    let mut products: Vec<usize> = Vec::new();
    while stack.len() > 0 {
        let token: Token = stack.pop().unwrap();
        match token {
            Token::Number(value) => products.push(value),
            Token::Operator(Op::Times) => {}
            Token::Operator(Op::Plus) => {
                if let Token::Number(rhs) = stack.pop().unwrap() {
                    let lhs = products.pop().unwrap();
                    products.push(lhs + rhs)
                } else {
                    panic!("unexpected non-number")
                }
            }
            _ => panic!("shouldn't have parens at this point"),
        }
    }

    (index, products.iter().product())
}

#[must_use]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let tokens: Vec<Token> = tokenize(line);
            evaluate_math(&tokens)
        })
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let tokens: Vec<Token> = tokenize(line);
            evaluate_advanced_math(&tokens)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    }

    #[test]
    fn part1_example6() {
        assert_eq!(
            part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn part1_multiline_example() {
        assert_eq!(
            part1("1 + 2 * 3 + 4 * 5 + 6\n1 + (2 * 3) + (4 * (5 + 6))"),
            122
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2("1 + 2 * 3 + 4 * 5 + 6"), 231)
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51)
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46)
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3) "), 1445)
    }

    #[test]
    fn part2_example5() {
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060)
    }

    #[test]
    fn part2_example6() {
        assert_eq!(
            part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        )
    }
}
