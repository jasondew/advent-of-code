type Number = Vec<(usize, usize)>;

fn parse(input: &str) -> Number {
    let mut depth: usize = 0;
    let mut number: Number = Vec::new();

    for ch in input.chars() {
        match ch {
            '[' => depth += 1,
            ',' => {}
            ']' => depth -= 1,
            value => number.push((value as usize - 48, depth)),
        }
    }
    number
}

fn add(mut a: Number, mut b: Number) -> Number {
    a.append(&mut b);
    reduce(
        a.iter()
            .map(|(value, depth)| (*value, *depth + 1))
            .collect(),
    )
}

fn reduce(mut number: Number) -> Number {
    if let Some(index) = number.iter().position(|(_value, depth)| depth > &4) {
        let (explode_left, _depth) = number[index];
        let (explode_right, _depth) = number[index + 1];

        let mut new_number: Number = number[0..index].into();
        new_number.push((0, 4));
        new_number.append(&mut number[index + 2..].into());

        if index > 0 {
            if let Some((value, depth)) = new_number.get(index - 1) {
                new_number[index - 1] = (value + explode_left, *depth);
            }
        }

        if let Some((value, depth)) = new_number.get(index + 1) {
            new_number[index + 1] = (value + explode_right, *depth);
        }

        reduce(new_number)
    } else if let Some(index) = number.iter().position(|(value, _depth)| value > &9) {
        let (value, depth) = number[index];
        let left = value / 2;
        let right = (value as f32 / 2.0).ceil() as usize;

        number[index] = (right, depth + 1);
        number.insert(index, (left, depth + 1));

        reduce(number)
    } else {
        number
    }
}

fn magnitude(mut number: Number) -> usize {
    for reduce_depth in (1..=4).rev() {
        loop {
            if let Some(index) = number
                .iter()
                .position(|(_value, depth)| depth == &reduce_depth)
            {
                let (left, _depth) = number.remove(index);
                let (right, _depth) = number[index];
                let magnitude = 3 * left + 2 * right;
                number[index] = (magnitude, reduce_depth - 1);
            } else {
                break;
            }
        }
    }
    if let (magnitude, 0) = number[0] {
        magnitude
    } else {
        panic!("magnitude calculation failed")
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let numbers: Vec<Number> = input.lines().map(|line| parse(line)).collect();
    let final_number: Number = numbers.into_iter().reduce(|a, b| add(a, b)).unwrap();

    magnitude(final_number)
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let numbers: Vec<Number> = input.lines().map(|line| parse(line)).collect();
    let mut max_magnitude: usize = 0;

    for a in numbers.iter() {
        for b in numbers.iter().skip(1) {
            max_magnitude = max_magnitude
                .max(magnitude(add(a.clone(), b.clone())))
                .max(magnitude(add(b.clone(), a.clone())))
        }
    }

    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_tests() {
        assert_eq!(
            reduce(parse("[[[[[9,8],1],2],3],4]")),
            parse("[[[[0,9],2],3],4]")
        );
        assert_eq!(
            reduce(parse("[7,[6,[5,[4,[3,2]]]]]")),
            parse("[7,[6,[5,[7,0]]]]")
        );
        assert_eq!(
            reduce(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]")),
            parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn add_tests() {
        assert_eq!(
            add(parse("[1,2]"), parse("[[3,4],5]")),
            parse("[[1,2],[[3,4],5]]")
        );

        assert_eq!(
            add(parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse("[1,1]")),
            parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn magnitude_tests() {
        assert_eq!(magnitude(parse("[9,1]")), 29);
        assert_eq!(magnitude(parse("[[9,1],[1,9]]")), 129);
        assert_eq!(
            magnitude(parse(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\n"
            ),
            4140
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\n"
            ),
            3993
        );
    }
}
