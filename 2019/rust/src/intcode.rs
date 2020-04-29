pub type Word = i32;
pub type Tape = Vec<Word>;
pub type Index = usize;

#[derive(Clone, Debug)]
pub struct Machine {
    pub tape: Tape,
    pub ip: Index,
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
}

#[derive(Debug)]
enum Operation {
    Add(Param, Param, Index),
    Mul(Param, Param, Index),
    In(Index),
    Out(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Index),
    Equal(Param, Param, Index),
    Done,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            tape: vec![],
            ip: 0,
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

        Ok(Self {
            tape: tape,
            ip: 0,
            state: State::NotStarted,
            input: vec![],
            output: vec![],
        })
    }

    pub fn run(self: &mut Self) -> &mut Self {
        loop {
            let operation = self.parse_operation();

            match operation {
                Operation::Add(a, b, to) => {
                    self.tape[to] = self.value(a) + self.value(b);
                    self.ip += 4
                }
                Operation::Mul(a, b, to) => {
                    self.tape[to] = self.value(a) * self.value(b);
                    self.ip += 4
                }
                Operation::In(to) => {
                    if let Some(value) = self.input.pop() {
                        self.tape[to] = value;
                        self.ip += 2
                    } else {
                        self.state = State::Waiting;
                        break self;
                    }
                }
                Operation::Out(from) => {
                    self.output.push(self.value(from));
                    self.ip += 2
                }
                Operation::JumpIfTrue(predicate, jump_to) => {
                    if self.value(predicate) != 0 {
                        self.ip = self.value(jump_to) as Index
                    } else {
                        self.ip += 3
                    }
                }
                Operation::JumpIfFalse(predicate, jump_to) => {
                    if self.value(predicate) == 0 {
                        self.ip = self.value(jump_to) as Index
                    } else {
                        self.ip += 3
                    }
                }
                Operation::LessThan(a, b, output) => {
                    if self.value(a) < self.value(b) {
                        self.tape[output] = 1
                    } else {
                        self.tape[output] = 0
                    }
                    self.ip += 4
                }
                Operation::Equal(a, b, output) => {
                    if self.value(a) == self.value(b) {
                        self.tape[output] = 1
                    } else {
                        self.tape[output] = 0
                    }
                    self.ip += 4
                }
                Operation::Done => {
                    self.state = State::Done;
                    break self;
                }
            }
        }
    }

    fn parse_operation(self: &Self) -> Operation {
        let string = format!("{:05}", self.tape[self.ip])
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
                self.parse_param(param_modes, 0),
                self.parse_param(param_modes, 1),
                self.tape[self.ip + 3] as Index,
            ),
            2 => Operation::Mul(
                self.parse_param(param_modes, 0),
                self.parse_param(param_modes, 1),
                self.tape[self.ip + 3] as Index,
            ),
            3 => Operation::In(self.tape[self.ip + 1] as Index),
            4 => Operation::Out(self.parse_param(param_modes, 0)),
            5 => Operation::JumpIfTrue(
                self.parse_param(param_modes, 0),
                self.parse_param(param_modes, 1),
            ),
            6 => Operation::JumpIfFalse(
                self.parse_param(param_modes, 0),
                self.parse_param(param_modes, 1),
            ),
            7 => Operation::LessThan(
                self.parse_param(param_modes, 0),
                self.parse_param(param_modes, 1),
                self.tape[self.ip + 3] as Index,
            ),
            8 => Operation::Equal(
                self.parse_param(param_modes, 0),
                self.parse_param(param_modes, 1),
                self.tape[self.ip + 3] as Index,
            ),
            99 => Operation::Done,
            _ => panic!("invalid opcode: {}", opcode),
        }
    }

    fn parse_param(self: &Self, modes: &str, index: Index) -> Param {
        let value = self.tape[self.ip + index + 1];
        let mode = modes.chars().nth(index).unwrap();

        match mode {
            '0' => Param::Position(value as Index),
            '1' => Param::Immediate(value),
            _ => panic!("invalid param mode: {}", mode),
        }
    }

    fn value(self: &Self, param: Param) -> Word {
        match param {
            Param::Immediate(value) => value,
            Param::Position(location) => self.tape[location],
        }
    }
}
