pub type Word = i64;
pub type Tape = Vec<Word>;
pub type Index = usize;

#[derive(Clone, Debug)]
pub struct Machine {
    pub tape: Tape,
    pub ip: Index,
    pub bp: Word,
    pub state: State,
    pub input: Vec<Word>,
    pub output: Vec<Word>,
}

#[derive(Clone, Debug)]
pub enum State {
    NotStarted,
    Waiting,
    Done,
}

#[derive(Debug)]
enum Param {
    Immediate(Word),
    Position(Index),
    Relative(Word),
}

impl Param {
    fn parse(machine: &Machine, modes: Word, index: Index) -> Self {
        let value = machine.tape[machine.ip + index + 1];
        let base: Word = 10;
        let mode = modes % base.pow((index + 1) as u32) / base.pow(index as u32);

        match mode {
            0 => Param::Position(value as Index),
            1 => Param::Immediate(value),
            2 => Param::Relative(value),
            _ => panic!("invalid param mode: {:?}", mode),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    In(Param),
    Out(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equal(Param, Param, Param),
    IncrementBasePointer(Param),
    Done,
}

impl Operation {
    fn parse(machine: &Machine) -> Operation {
        let instruction = machine.tape[machine.ip];
        let opcode = instruction % 100;
        let modes = instruction / 100;

        match opcode {
            1 => Operation::Add(
                Param::parse(machine, modes, 0),
                Param::parse(machine, modes, 1),
                Param::parse(machine, modes, 2),
            ),
            2 => Operation::Mul(
                Param::parse(machine, modes, 0),
                Param::parse(machine, modes, 1),
                Param::parse(machine, modes, 2),
            ),
            3 => Operation::In(Param::parse(machine, modes, 0)),
            4 => Operation::Out(Param::parse(machine, modes, 0)),
            5 => Operation::JumpIfTrue(
                Param::parse(machine, modes, 0),
                Param::parse(machine, modes, 1),
            ),
            6 => Operation::JumpIfFalse(
                Param::parse(machine, modes, 0),
                Param::parse(machine, modes, 1),
            ),
            7 => Operation::LessThan(
                Param::parse(machine, modes, 0),
                Param::parse(machine, modes, 1),
                Param::parse(machine, modes, 2),
            ),
            8 => Operation::Equal(
                Param::parse(machine, modes, 0),
                Param::parse(machine, modes, 1),
                Param::parse(machine, modes, 2),
            ),
            9 => Operation::IncrementBasePointer(Param::parse(machine, modes, 0)),
            99 => Operation::Done,
            _ => panic!("invalid opcode: {:?}", opcode),
        }
    }
}

impl Machine {
    pub fn new() -> Self {
        Self::new_with_tape(&vec![])
    }

    pub fn new_with_tape(tape: &Tape) -> Self {
        Self {
            tape: tape.clone(),
            ip: 0,
            bp: 0,
            state: State::NotStarted,
            input: vec![],
            output: vec![],
        }
    }

    pub fn new_from_file(path: &str) -> std::io::Result<Self> {
        let input = std::fs::read_to_string(path)?;
        let tape = input
            .trim()
            .split(",")
            .map(|str| str.parse().unwrap())
            .collect();

        Ok(Self::new_with_tape(&tape))
    }

    pub fn run(self: &mut Self) -> &mut Self {
        loop {
            match Operation::parse(self) {
                Operation::Add(a, b, to) => {
                    let a = self.param_value(a);
                    let b = self.param_value(b);
                    self.set_tape_value(to, a + b);
                    self.ip += 4
                }
                Operation::Mul(a, b, to) => {
                    let a = self.param_value(a);
                    let b = self.param_value(b);
                    self.set_tape_value(to, a * b);
                    self.ip += 4
                }
                Operation::In(to) => {
                    if let Some(value) = self.input.pop() {
                        self.set_tape_value(to, value);
                        self.ip += 2
                    } else {
                        self.state = State::Waiting;
                        break self;
                    }
                }
                Operation::Out(from) => {
                    let from = self.param_value(from);
                    self.output.push(from);
                    self.ip += 2
                }
                Operation::JumpIfTrue(predicate, jump_to) => {
                    if self.param_value(predicate) != 0 {
                        self.ip = self.param_value(jump_to) as Index
                    } else {
                        self.ip += 3
                    }
                }
                Operation::JumpIfFalse(predicate, jump_to) => {
                    if self.param_value(predicate) == 0 {
                        self.ip = self.param_value(jump_to) as Index
                    } else {
                        self.ip += 3
                    }
                }
                Operation::LessThan(a, b, output) => {
                    if self.param_value(a) < self.param_value(b) {
                        self.set_tape_value(output, 1)
                    } else {
                        self.set_tape_value(output, 0)
                    }
                    self.ip += 4
                }
                Operation::Equal(a, b, output) => {
                    if self.param_value(a) == self.param_value(b) {
                        self.set_tape_value(output, 1)
                    } else {
                        self.set_tape_value(output, 0)
                    }
                    self.ip += 4
                }
                Operation::IncrementBasePointer(x) => {
                    self.bp += self.param_value(x);
                    self.ip += 2
                }
                Operation::Done => {
                    self.state = State::Done;
                    break self;
                }
            }
        }
    }

    fn param_value(self: &mut Self, param: Param) -> Word {
        match param {
            Param::Immediate(value) => value,
            Param::Position(location) => self.get_tape_value(location),
            Param::Relative(offset) => self.get_tape_value((self.bp + offset) as Index),
        }
    }

    fn get_tape_value(self: &mut Self, index: Index) -> Word {
        if index < self.tape.len() {
            self.tape[index]
        } else {
            0
        }
    }

    fn set_tape_value(self: &mut Self, at: Param, value: Word) {
        let index = match at {
            Param::Immediate(_) => panic!("invalid parameter (Immediate) for In given"),
            Param::Position(location) => location,
            Param::Relative(offset) => (self.bp + offset) as Index,
        };

        if index >= self.tape.len() {
            self.tape.resize(index + 1, 0)
        }

        self.tape[index] = value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let tape = vec![
            4, 3, 101, 72, 14, 3, 101, 1, 4, 4, 5, 3, 16, 99, 29, 7, 0, 3, -67, -12, 87, -8, 3, -6,
            -8, -67, -23, -10,
        ];
        let hello_world = "Hello, world!\n"
            .chars()
            .map(|c| c as Word)
            .collect::<Tape>();

        assert_eq!(hello_world, run(tape))
    }

    #[test]
    fn supports_large_numbers() {
        let tape = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        assert_eq!(vec![1_219_070_632_396_864], run(tape))
    }

    #[test]
    fn quine() {
        let tape = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let machine = &mut Machine::new_with_tape(&tape);

        machine.run();

        assert_eq!(tape, machine.output)
    }

    #[allow(dead_code)]
    fn run(tape: Tape) -> Tape {
        let machine = &mut Machine::new_with_tape(&tape);
        machine.run();

        return machine.output.clone();
    }
}
