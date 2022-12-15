use std::{cmp::Ordering, iter::Peekable};

#[derive(PartialEq)]
enum Packet {
    Scalar(usize),
    List(Vec<Packet>),
}

impl std::fmt::Debug for Packet {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            Packet::List(list) => {
                let delimited: String = list
                    .iter()
                    .map(|packet| format!("{:?}", packet))
                    .collect::<Vec<String>>()
                    .join(",");
                write!(formatter, "[{}]", delimited)?;
            }
            Packet::Scalar(value) => {
                write!(formatter, "{}", value)?;
            }
        }
        Ok(())
    }
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let packet_pairs = parse(input);
    let mut right_order_indices = vec![];

    for (index, (left, right)) in packet_pairs.iter().enumerate() {
        //        dbg!(left, right);
        if rightly_ordered(left, right).unwrap() {
            //            println!("{} is RIGHT ORDERED!", index + 1);
            right_order_indices.push(index + 1);
        } else {
            //            println!("{} is NOT RIGHT ORDERED!", index + 1);
        }
    }

    right_order_indices.iter().sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let packet_pairs = parse(input);
    let mut packets: Vec<Packet> = packet_pairs
        .into_iter()
        .flat_map(|(left, right)| vec![left, right])
        .collect();
    let divider_packets = &[
        Packet::List(vec![Packet::List(vec![Packet::Scalar(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Scalar(6)])]),
    ];
    packets.push(Packet::List(vec![Packet::List(vec![Packet::Scalar(2)])]));
    packets.push(Packet::List(vec![Packet::List(vec![Packet::Scalar(6)])]));

    packets.sort_unstable_by(|a, b| {
        if let Some(true) = rightly_ordered(a, b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if divider_packets.iter().any(|dp| dp == packet) {
                Some(index + 1)
            } else {
                None
            }
        })
        .product()
}

fn rightly_ordered(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Packet::Scalar(lv), Packet::Scalar(rv)) => match lv.cmp(rv) {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => None,
        },
        (ll @ Packet::List(_), Packet::Scalar(rv)) => {
            rightly_ordered(ll, &Packet::List(vec![Packet::Scalar(*rv)]))
        }
        (Packet::Scalar(lv), rl @ Packet::List(_)) => {
            rightly_ordered(&Packet::List(vec![Packet::Scalar(*lv)]), rl)
        }
        (Packet::List(ll), Packet::List(rl)) => ll
            .iter()
            .zip(rl.iter())
            .find_map(|(l, r)| rightly_ordered(l, r))
            .or(match ll.len().cmp(&rl.len()) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            }),
    }
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|packet_lines| {
            let (left, right) = packet_lines.trim().split_once('\n').unwrap();
            (parse_packet_line(left), parse_packet_line(right))
        })
        .collect()
}

fn parse_packet_line(line: &str) -> Packet {
    parse_list(&mut line.chars().peekable())
}

fn parse_list<T>(chars: &mut Peekable<T>) -> Packet
where
    T: Iterator<Item = char>,
{
    let mut list = vec![];
    consume(chars, '[');

    loop {
        match chars.peek() {
            Some('[') => list.push(parse_list(chars)),
            Some(']') => break,
            Some(',') => {
                consume(chars, ',');
                continue;
            }
            Some(_) => list.push(parse_digit(chars)),
            None => panic!("unexpected end of input"),
        }
    }

    consume(chars, ']');
    Packet::List(list)
}

fn parse_digit<T>(chars: &mut Peekable<T>) -> Packet
where
    T: Iterator<Item = char>,
{
    let mut value = 0usize;
    let mut multipler = 1usize;

    loop {
        let digit = chars.next().unwrap().to_string().parse::<usize>().unwrap();
        value = value * multipler + digit;
        multipler *= 10;

        match chars.peek() {
            Some(',' | ']') | None => break,
            Some(_) => continue,
        }
    }

    Packet::Scalar(value)
}

fn consume<T>(chars: &mut T, expected: char)
where
    T: Iterator<Item = char>,
{
    match chars.next() {
        Some(ch) => {
            assert!(ch == expected, "expected: {} got: {}", expected, ch);
        }
        None => panic!("expected: {}, got EOF", expected),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 13)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 140)
    }
}
