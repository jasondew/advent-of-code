use std::ops::RangeInclusive;

#[derive(Debug)]
struct Number {
    value: usize,
    x_range: RangeInclusive<usize>,
    y: usize,
}

impl Number {
    fn new(digit: char, x: usize, y: usize) -> Self {
        Self {
            value: Self::parse_digit(digit),
            x_range: x..=x,
            y,
        }
    }

    fn push_digit(&mut self, digit: char) {
        self.value = self.value * 10 + Self::parse_digit(digit);
        self.x_range = *self.x_range.start()..=(self.x_range.end() + 1);
    }

    fn parse_digit(digit: char) -> usize {
        digit.to_digit(10).unwrap() as usize
    }

    fn symbol_adjacent(&self, symbols: &Vec<Symbol>) -> bool {
        symbols.iter().any(|symbol| self.is_adjacent(symbol))
    }

    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        Self::buffer(&self.x_range, 1).contains(&symbol.x)
            && Self::buffer(&(self.y..=self.y), 1).contains(&symbol.y)
    }

    fn buffer(
        range: &RangeInclusive<usize>,
        amount: usize,
    ) -> RangeInclusive<usize> {
        let buffered_start = if range.start() > &0usize {
            range.start() - amount
        } else {
            0
        };
        let buffered_end = range.end() + amount;

        buffered_start..=buffered_end
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    x: usize,
    y: usize,
}

impl Symbol {
    fn number_values_adjacent(&self, numbers: &Vec<Number>) -> Vec<usize> {
        numbers
            .iter()
            .filter(|number| number.is_adjacent(self))
            .map(|number| number.value)
            .collect()
    }
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let schematic = parse(input);
    let mut total: usize = 0;

    for number in schematic.numbers {
        if number.symbol_adjacent(&schematic.symbols) {
            total += number.value;
        }
    }

    total
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let schematic = parse(input);
    let mut total: usize = 0;

    for symbol in schematic.symbols {
        if symbol.value == '*' {
            let values = symbol.number_values_adjacent(&schematic.numbers);

            if values.len() == 2 {
                total += values.iter().product::<usize>();
            }
        }
    }

    total
}

fn parse(input: &str) -> Schematic {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut maybe_number: Option<Number> = None;

        for (x, ch) in line.chars().enumerate() {
            match ch {
                '0'..='9' => {
                    match maybe_number {
                        None => {
                            maybe_number = Some(Number::new(ch, x, y));
                        }
                        Some(ref mut number) => {
                            number.push_digit(ch);
                        }
                    };
                }
                symbol_or_dot => {
                    if symbol_or_dot != '.' {
                        symbols.push(Symbol { value: ch, x, y });
                    }

                    maybe_number.map(|n| numbers.push(n));
                    maybe_number = None;
                }
            }
        }
        maybe_number.map(|n| numbers.push(n));
    }

    Schematic { numbers, symbols }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 4361)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 467835)
    }
}
