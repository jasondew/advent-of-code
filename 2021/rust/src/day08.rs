use std::collections::HashMap;

#[must_use]
pub fn part1(input: &str) -> usize {
    let pairs = parse(input);
    let mut count: usize = 0;

    for (_, output) in pairs {
        for segments in output {
            match segments.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }

    count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let pairs = parse(input);
    let mut total: usize = 0;

    for (pattern, output) in pairs {
        total += decode(pattern, output)
    }

    total
}

// segment: digits required for
// a: 0,    2, 3,    5, 6, 7, 8, 9
// b: 0,          4, 5, 6,    8, 9
// c: 0, 1, 2, 3, 4,       7, 8, 9
// d:       2, 3, 4, 5, 6,    8, 9
// e: 0,    2,          6,    8
// f: 0, 1,    3, 4, 5, 6, 7, 8, 9
// g: 0,    2, 3,    5, 6,    8, 9
//
// digits by segment count
// 2: 1
// 3: 7
// 4: 4
// 5: 2, 3, 5
// 6: 0, 6, 9
// 7: 8
fn decode(pattern: Vec<Vec<char>>, output: Vec<Vec<char>>) -> usize {
    let mut digit_map: HashMap<&Vec<char>, u8> = HashMap::new();
    let mut segment_map: HashMap<char, char> = HashMap::new();

    let one = &mut find_by_length(&pattern, 2)[0];
    one.sort();
    let four = &mut find_by_length(&pattern, 4)[0];
    four.sort();
    let seven = &mut find_by_length(&pattern, 3)[0];
    seven.sort();
    let eight = &mut find_by_length(&pattern, 7)[0];
    eight.sort();
    let length_fives = find_by_length(&pattern, 5);

    digit_map.insert(&one, 1);
    digit_map.insert(&seven, 7);
    digit_map.insert(&four, 4);
    digit_map.insert(&eight, 8);

    let a = minus(&seven, &one)[0];
    segment_map.insert('a', a);

    let f = find_char_missing_in_only_one(&pattern);
    segment_map.insert('f', f);

    let c = minus(&one, &vec![f])[0];
    segment_map.insert('c', c);

    let be = find_chars_missing_in_all_but_one(&length_fives);
    let bd = minus(&four, &one);

    let b = first_common(&be, &bd);
    segment_map.insert('b', b);

    let d = minus(&bd, &vec![b])[0];
    segment_map.insert('d', d);

    let e = minus(&be, &vec![b])[0];
    segment_map.insert('e', e);

    let eg = minus(&minus(&eight, &four), &seven);
    let g = minus(&eg, &vec![e])[0];
    segment_map.insert('g', g);

    let mut zero = vec![a, b, c, e, f, g];
    zero.sort();
    digit_map.insert(&zero, 0);

    let mut two = vec![a, c, d, e, g];
    two.sort();
    digit_map.insert(&two, 2);

    let mut three = vec![a, c, d, f, g];
    three.sort();
    digit_map.insert(&three, 3);

    let mut five = vec![a, b, d, f, g];
    five.sort();
    digit_map.insert(&five, 5);

    let mut six = vec![a, b, d, e, f, g];
    six.sort();
    digit_map.insert(&six, 6);

    let mut nine = vec![a, b, c, d, f, g];
    nine.sort();
    digit_map.insert(&nine, 9);

    let numbers: Vec<String> = output
        .into_iter()
        .map(|mut digit| {
            digit.sort();
            digit_map.get(&digit).unwrap().to_string()
        })
        .collect();

    usize::from_str_radix(&numbers.join(""), 10).unwrap()
}

fn minus(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    a.iter()
        .filter(|&aa| b.iter().find(|&bb| aa == bb).is_none())
        .cloned()
        .collect()
}

fn first_common(a: &Vec<char>, b: &Vec<char>) -> char {
    for &aa in a.iter() {
        for &bb in b.iter() {
            if aa == bb {
                return aa;
            }
        }
    }
    panic!("would've been nice to find one...")
}

fn find_char_missing_in_only_one(pattern: &Vec<Vec<char>>) -> char {
    for target in "abcdefg".chars() {
        if pattern
            .iter()
            .filter(|chars| chars.iter().find(|&&ch| ch == target).is_some())
            .count()
            == (pattern.len() - 1)
        {
            return target;
        }
    }
    panic!("really should've found one...")
}

fn find_chars_missing_in_all_but_one(pattern: &Vec<Vec<char>>) -> Vec<char> {
    let mut singles: Vec<char> = vec![];
    for target in "abcdefg".chars() {
        if pattern
            .iter()
            .filter(|chars| chars.iter().find(|&&ch| ch == target).is_some())
            .count()
            == 1
        {
            singles.push(target);
        }
    }
    singles
}

fn find_by_length(pattern: &Vec<Vec<char>>, target: usize) -> Vec<Vec<char>> {
    pattern
        .iter()
        .filter(|chars| chars.len() == target)
        .cloned()
        .collect()
}

fn parse(input: &str) -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(" | ")
                .unwrap_or_else(|| panic!("invalid line: {}", line));

            (
                left.split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect(),
                right
                    .split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 26)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n"), 5353);
        assert_eq!(part2(input()), 61229)
    }
}
