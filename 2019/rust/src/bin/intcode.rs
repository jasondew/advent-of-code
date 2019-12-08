#[derive(Debug)]
pub struct Configuration {
    pub intcode: Vec<i32>,
    pub ip: usize,
    pub state: State,
    pub output: Option<i32>,
}

#[derive(Debug)]
pub enum State {
    Waiting,
    Done,
}

#[derive(Debug)]
enum Param {
    Immediate(i32),
    Position(usize),
}

#[derive(Debug)]
enum Operation {
    Add(Param, Param, usize),
    Mul(Param, Param, usize),
    In(usize),
    Out(usize),
    Done,
}

pub fn run(intcode: &mut Vec<i32>, inputs: &mut Vec<i32>) -> Configuration {
    let mut ip: usize = 0;
    let mut output: Option<i32> = None;

    loop {
        let operation = parse_operation(intcode, ip);

        match operation {
            Operation::Add(a, b, to) => {
                intcode[to] = value(intcode, a) + value(intcode, b);
                ip += 4
            }
            Operation::Mul(a, b, to) => {
                intcode[to] = value(intcode, a) * value(intcode, b);
                ip += 4
            }
            Operation::In(to) => {
                if inputs.len() > 0 {
                    intcode[to] = inputs.remove(0);
                    ip += 2
                } else {
                    break Configuration {
                        intcode: intcode.to_vec(),
                        ip: ip,
                        state: State::Waiting,
                        output: output,
                    };
                }
            }
            Operation::Out(from) => {
                output = Some(intcode[from]);
                ip += 2
            }
            Operation::Done => {
                break Configuration {
                    intcode: intcode.to_vec(),
                    ip: ip,
                    state: State::Done,
                    output: output,
                }
            }
        }
    }
}

fn value(intcode: &Vec<i32>, param: Param) -> i32 {
    match param {
        Param::Immediate(value) => value,
        Param::Position(location) => intcode[location],
    }
}

fn parse_operation(intcode: &Vec<i32>, ip: usize) -> Operation {
    let string = format!("{:05}", intcode[ip])
        .chars()
        .rev()
        .collect::<String>();
    let (reversed_opcode, param_modes): (&str, &str) = string.split_at(2);
    let opcode: u32 = reversed_opcode
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();

    match opcode {
        1 => Operation::Add(
            parse_param(intcode, ip, param_modes, 0),
            parse_param(intcode, ip, param_modes, 1),
            intcode[ip + 3] as usize,
        ),
        2 => Operation::Mul(
            parse_param(intcode, ip, param_modes, 0),
            parse_param(intcode, ip, param_modes, 1),
            intcode[ip + 3] as usize,
        ),
        3 => Operation::In(intcode[ip + 1] as usize),
        4 => Operation::Out(intcode[ip + 1] as usize),
        99 => Operation::Done,
        _ => panic!("invalid opcode: {}", opcode),
    }
}

fn parse_param(intcode: &Vec<i32>, ip: usize, modes: &str, index: usize) -> Param {
    let value = intcode[ip + index + 1];
    let mode = modes.chars().nth(index).unwrap();

    match mode {
        '0' => Param::Position(value as usize),
        '1' => Param::Immediate(value),
        _ => panic!("invalid param mode: {}", mode),
    }
}
