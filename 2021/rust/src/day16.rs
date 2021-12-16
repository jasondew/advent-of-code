#[derive(Debug)]
enum Payload {
    Literal(&str),
    Operator(&str)
}

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    payload: Payload
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let packet = parse(input.trim_end());
    0
}

fn parse(input: &str) -> Packet {
    println!( "packet: {:b}", usize::from_str_radix(input, 16).unwrap());

    let version = extract(packet, 0, 3);
    let type_id = extract(packet, 3, 3);

    match type_id {
        4 => parse_literal(packet),
        _ => parse_operator(packet),
    }
}

fn parse_operator(packet: &str) -> Packet {

fn parse_literal(packet: &str) -> Packet {
    let mut value = 0usize;
    let mut packet_offset = 6usize;
    let mut group: usize;

    loop {
        group = extract(packet, packet_offset, 5);
        packet_offset += 5;
        value = (value << 4) ^ (group & 0b1111);

        if (group & 0b10000) == 0 {
            break;
        }
    }

    value
}

fn extract(packet: &str, start_bit: usize, bit_length: usize) -> usize {
    let start_char: usize = start_bit / 4usize;
    let char_length = (bit_length as f32 / 4f32).ceil() as usize;
    let end_char: usize = (start_char + char_length + 1).min(packet.len());

    let chars = packet.get(start_char..end_char).unwrap();
    let raw_value = usize::from_str_radix(chars, 16).unwrap();

    let source_end_bit = end_char * 4;
    let extra_end_bits = source_end_bit - (start_bit + bit_length);
    let bitmask = (usize::MAX << bit_length) ^ usize::MAX;

    (raw_value >> extra_end_bits) & bitmask
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
D2FE28\n"
    }

    #[test]
    fn part1_examples() {
        assert_eq!(extract("D2FE28", 0, 3), 0b110);
        assert_eq!(extract("D2FE28", 3, 3), 0b100);
        assert_eq!(extract("D2FE28", 6, 5), 0b10111);
        assert_eq!(extract("D2FE28", 11, 5), 0b11110);
        assert_eq!(part1(input()), 2021)
    }

    #[test]
    fn part2_example() {
        //        assert_eq!(part2(input()), 16)
    }
}
