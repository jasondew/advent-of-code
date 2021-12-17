#[derive(Debug)]
enum Payload {
    Literal(usize),
    Operator(usize, Vec<Packet>),
}

impl PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        use Payload::*;
        match (self, other) {
            (Literal(a), Literal(b)) => a == b,
            (Operator(at, a), Operator(bt, b)) => {
                at == bt && a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    payload: Payload,
}

fn parse(input: &str, mut offset: usize) -> (Packet, usize) {
    //        println!("packet: {:b}", usize::from_str_radix(input, 16).unwrap());
    let version = extract(input, offset, 3);
    let type_id = extract(input, offset + 3, 3);

    match type_id {
        4 => {
            let (literal, new_offset) = parse_literal(input, offset + 6);
            offset = new_offset;

            (
                Packet {
                    version,
                    payload: Payload::Literal(literal),
                },
                offset,
            )
        }
        _ => {
            let length_type_id = extract(input, offset + 6, 1);
            let mut subpackets: Vec<Packet> = Vec::new();

            if length_type_id == 0 {
                let payload_length = extract(input, offset + 7, 15);

                offset += 22;
                let subpackets_end = offset + payload_length;

                loop {
                    let (subpacket, new_offset) = parse(input, offset);

                    offset = new_offset;
                    subpackets.push(subpacket);

                    if offset >= subpackets_end {
                        break;
                    }
                }
            } else {
                let subpacket_count = extract(input, offset + 7, 11);
                offset += 18;

                for _ in 1..=subpacket_count {
                    let (subpacket, new_offset) = parse(input, offset);

                    offset = new_offset;
                    subpackets.push(subpacket);
                }
            }

            (
                Packet {
                    version,
                    payload: Payload::Operator(type_id, subpackets),
                },
                offset,
            )
        }
    }
}

fn parse_literal(input: &str, mut offset: usize) -> (usize, usize) {
    let mut value = 0usize;
    let mut group: usize;

    loop {
        group = extract(input, offset, 5);
        offset += 5;
        value = (value << 4) ^ (group & 0b1111);

        if (group & 0b10000) == 0 {
            break;
        }
    }

    (value, offset)
}

fn extract(input: &str, start_bit: usize, bit_length: usize) -> usize {
    let start_char: usize = start_bit / 4usize;
    let char_length = (bit_length as f32 / 4f32).ceil() as usize;
    let end_char: usize = (start_char + char_length + 1).min(input.len());

    let chars = input.get(start_char..end_char).unwrap();
    let raw_value = usize::from_str_radix(chars, 16).unwrap();

    let source_end_bit = end_char * 4;
    let extra_end_bits = source_end_bit - (start_bit + bit_length);
    let bitmask = (usize::MAX << bit_length) ^ usize::MAX;

    (raw_value >> extra_end_bits) & bitmask
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (packet, _offset) = parse(input.trim_end(), 0);

    version_sum(&packet)
}

fn version_sum(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::Literal(_) => packet.version,
        Payload::Operator(_, subpackets) => {
            subpackets
                .iter()
                .map(|subpacket| version_sum(subpacket))
                .sum::<usize>()
                + packet.version
        }
    }
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (packet, _offset) = parse(input.trim_end(), 0);
    evaluate(&packet)
}

fn evaluate(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::Literal(value) => *value,
        Payload::Operator(type_id, subpackets) => match type_id {
            0 => subpackets.iter().map(|subpacket| evaluate(subpacket)).sum(),
            1 => subpackets
                .iter()
                .map(|subpacket| evaluate(subpacket))
                .product(),
            2 => subpackets
                .iter()
                .map(|subpacket| evaluate(subpacket))
                .min()
                .unwrap(),
            3 => subpackets
                .iter()
                .map(|subpacket| evaluate(subpacket))
                .max()
                .unwrap(),
            5 => {
                let mut iterator = subpackets.iter();
                if evaluate(iterator.next().unwrap()) > evaluate(iterator.next().unwrap()) {
                    1
                } else {
                    0
                }
            }
            6 => {
                let mut iterator = subpackets.iter();
                if evaluate(iterator.next().unwrap()) < evaluate(iterator.next().unwrap()) {
                    1
                } else {
                    0
                }
            }
            7 => {
                let mut iterator = subpackets.iter();
                if evaluate(iterator.next().unwrap()) == evaluate(iterator.next().unwrap()) {
                    1
                } else {
                    0
                }
            }
            _ => panic!("unsupported type ID: {}", type_id),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let payload = "D2FE28";

        assert_eq!(extract(payload, 0, 3), 0b110);
        assert_eq!(extract(payload, 3, 3), 0b100);
        assert_eq!(extract(payload, 6, 5), 0b10111);
        assert_eq!(extract(payload, 11, 5), 0b11110);
        assert_eq!(
            parse(payload, 0),
            (
                Packet {
                    version: 6,
                    payload: Payload::Literal(2021)
                },
                21
            )
        );

        assert_eq!(
            parse("38006F45291200", 0),
            (
                Packet {
                    version: 1,
                    payload: Payload::Operator(
                        6,
                        vec![
                            Packet {
                                version: 6,
                                payload: Payload::Literal(10)
                            },
                            Packet {
                                version: 2,
                                payload: Payload::Literal(20)
                            },
                        ]
                    )
                },
                49
            )
        );

        assert_eq!(
            parse("EE00D40C823060", 0),
            (
                Packet {
                    version: 7,
                    payload: Payload::Operator(
                        3,
                        vec![
                            Packet {
                                version: 2,
                                payload: Payload::Literal(1)
                            },
                            Packet {
                                version: 4,
                                payload: Payload::Literal(2)
                            },
                            Packet {
                                version: 1,
                                payload: Payload::Literal(3)
                            },
                        ]
                    )
                },
                51
            )
        );

        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}
