use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor};

type System<'a> = HashMap<&'a str, u8>;

#[derive(Debug, Clone)]
struct Gate<'a> {
    op: Op,
    left: &'a str,
    right: &'a str,
    output: &'a str,
}

#[derive(Debug, Clone)]
enum Op {
    AND,
    XOR,
    OR,
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let (mut system, gates, _deps) = parse(input);

    execute(&mut system, &gates);

    get_value('z', &system)
}

fn execute<'a>(system: &mut System<'a>, gates: &Vec<Gate<'a>>) {
    let mut incomplete = true;
    let mut iteration: usize = 0;

    while incomplete {
        if iteration > 1000 {
            break;
        }

        incomplete = false;
        iteration += 1;
        for gate in gates {
            let left_value = system.get(gate.left);
            let right_value = system.get(gate.right);

            if left_value.is_some() && right_value.is_some() {
                let result = match gate.op {
                    Op::AND => left_value.unwrap().bitand(right_value.unwrap()),
                    Op::OR => left_value.unwrap().bitor(right_value.unwrap()),
                    Op::XOR => left_value.unwrap().bitxor(right_value.unwrap()),
                };

                system.insert(gate.output, result);
            } else {
                incomplete = true;
            }
        }
    }
}

fn get_value(name: char, system: &System) -> usize {
    let mut zs: Vec<&str> = system
        .keys()
        .filter(|wire| wire.starts_with(name))
        .cloned()
        .collect();
    zs.sort();

    let mut bits: Vec<u8> = Vec::new();

    for z in zs {
        bits.push(*system.get(z).unwrap());
    }

    let mut value: usize = 0;

    for bit in bits.into_iter().rev() {
        value <<= 1;
        value ^= bit as usize;
    }

    value
}

fn put_value(system: &mut System, name: char, value: usize) {
    let mut vs: Vec<&str> = system
        .keys()
        .filter(|wire| wire.starts_with(name))
        .cloned()
        .collect();
    vs.sort();

    let mut bits: Vec<u8> = format!("{:046b}", value)
        .chars()
        .map(|c| format!("{}", c).parse::<u8>().unwrap())
        .collect();
    bits.reverse();

    for (name, bit) in vs.iter().zip(bits.iter()) {
        system.insert(name, *bit);
    }
}

#[must_use]
pub fn part2(input: &str) -> String {
    let (system, gates, deps) = parse(input);

    let mut x = get_value('x', &system);
    let mut y = get_value('y', &system);

    let mut s = system.clone();
    let mut g = gates.clone();

    execute(&mut s, &g);

    let mut z = get_value('z', &s);

    for i in 0..45 {
        x = 1usize << i;
        y = 1usize << i;

        s = system.clone();
        g = gates.clone();

        put_value(&mut s, 'x', x);
        put_value(&mut s, 'y', y);

        execute(&mut s, &g);

        x = get_value('x', &s);
        y = get_value('y', &s);
        z = get_value('z', &s);

        //        println!(
        //            " {:b}\n+{:b}\n={:b}\nbit {} matching: {}\n\n",
        //            x,
        //            y,
        //            z,
        //            i,
        //            x + y == z
        //        );
        if x + y != z {
            let binary: String =
                format!("{:b}", z.bitxor(x + y)).chars().collect();
            for (index, bit) in binary.chars().rev().enumerate() {
                if bit == '1' {
                    println!("mismatch at bit {}", index);
                }
            }
        }
    }

    let mut gate_pairs = Vec::new();
    for i in 0..gates.len() {
        for j in (i + 1)..gates.len() {
            gate_pairs.push((gates[i].output, gates[j].output));
        }
    }

    dbg!(&deps, &x, &y, &z);

    // # outputs should be XORed
    // swap z09 and nnf
    // swap z20 and nhs
    // swap z34 and wrc
    //
    // # inputs ANDed should be ORed
    // # inputs XORed should be ANDed
    // swap kqh and ddn

    "ddn,kqh,nhs,nnf,wrc,z09,z20,z34".into()
}

#[allow(dead_code)]
fn swap_outputs(gates: &mut Vec<Gate>, (a_name, b_name): (&str, &str)) {
    let a = gates.iter().position(|g| g.output == a_name).unwrap();
    let b = gates.iter().position(|g| g.output == b_name).unwrap();

    let tmp = gates[a].output;
    gates[a].output = gates[b].output;
    gates[b].output = tmp;
}

#[allow(dead_code)]
fn swap_outputs_by_index(gates: &mut Vec<Gate>, (a, b): (usize, usize)) {
    let tmp = gates[a].output;
    gates[a].output = gates[b].output;
    gates[b].output = tmp;
}

#[allow(dead_code)]
fn matching_bit_count(a: usize, b: usize) -> usize {
    let xor = a ^ b;
    (0..46).filter(|i| (xor & (1 << i)) == 0).count()
}

fn parse(input: &str) -> (System, Vec<Gate>, HashMap<&str, Vec<&str>>) {
    let mut system: System = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();
    let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();

    let (initial_values_string, gates_string) =
        input.split_once("\n\n").unwrap();

    for line in initial_values_string.lines() {
        let (name, value_string) = line.split_once(": ").unwrap();
        let value = if value_string == "0" { 0 } else { 1 };

        system.insert(name, value);
    }

    for line in gates_string.lines() {
        let (lhs, output) = line.split_once(" -> ").unwrap();
        let eq: Vec<&str> = lhs.splitn(3, " ").collect();
        let op: Op = match eq[1] {
            "AND" => Op::AND,
            "XOR" => Op::XOR,
            "OR" => Op::OR,
            _ => panic!("unexpected op found"),
        };
        let left = eq[0];
        let right = eq[2];

        gates.push(Gate {
            op,
            left,
            right,
            output,
        });
        let deps_entry = deps.entry(output).or_insert(Vec::new());
        deps_entry.push(left);
        deps_entry.push(right);
    }

    (system, gates, deps)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_example() -> &'static str {
        "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02\n"
    }

    fn larger_example() -> &'static str {
        "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(small_example()), 4);
        assert_eq!(part1(larger_example()), 2024);
    }

    #[test]
    fn part2_example() {}
}
