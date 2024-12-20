use std::{
    collections::HashMap,
    ops::{BitXor, Shl, Shr},
};

#[derive(Clone, Debug)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    memory: Vec<u8>,
    instruction_pointer: usize,
    outputs: Vec<u8>,
    running: bool,
    debug: bool,
}

impl Default for Computer {
    fn default() -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            memory: Vec::new(),
            instruction_pointer: 0,
            outputs: Vec::new(),
            running: true,
            debug: false,
        }
    }
}

impl Computer {
    fn run(&mut self) {
        while self.running {
            self.step()
        }
    }

    fn step(&mut self) {
        match self.next_instruction() {
            Some(0) => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A
                // register. The denominator is found by raising 2 to the power of the instruction's combo
                // operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A
                // by 2^B.) The result of the division operation is truncated to an integer and then
                // written to the A register.

                let instruction = self.next_instruction().expect("operand");
                let (operand_name, operand_value) =
                    self.decode_operand(instruction);
                let result = self.register_a.shr(operand_value);

                if self.debug {
                    println!(
                        "[{}] adv :: A = A / 2^{} :: {} / 2^{} = {}",
                        self.instruction_pointer - 2,
                        operand_name,
                        self.register_a,
                        operand_value,
                        result
                    )
                }

                self.register_a = result;
            }
            Some(1) => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
                // instruction's literal operand, then stores the result in register B.

                let literal = self.next_instruction().expect("operand");
                let result = self.register_b.bitxor(literal as usize);

                if self.debug {
                    println!(
                        "[{}] bxl :: B = xor(B, {}) = {}",
                        self.instruction_pointer - 2,
                        literal,
                        result
                    )
                }

                self.register_b = result;
            }
            Some(2) => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
                // (thereby keeping only its lowest 3 bits), then writes that value to the B register.

                let instruction = self.next_instruction().expect("operand");
                let (operand_name, operand_value) =
                    self.decode_operand(instruction);
                let result = operand_value % 8;

                if self.debug {
                    println!(
                        "[{}] bst :: B = {} % 8 = {} % 8 = {}",
                        self.instruction_pointer - 2,
                        operand_name,
                        operand_value,
                        result
                    )
                }

                self.register_b = result;
            }
            Some(3) => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A
                // register is not zero, it jumps by setting the instruction pointer to the value of its
                // literal operand; if this instruction jumps, the instruction pointer is not increased by
                // 2 after this instruction.

                let literal = self.next_instruction().expect("operand");

                if self.debug {
                    println!(
                        "[{}] jnz :: A = {} => {}",
                        self.instruction_pointer - 2,
                        self.register_a,
                        if self.register_a == 0 {
                            "noop".into()
                        } else {
                            format!("jump {}", literal)
                        }
                    )
                }

                if self.register_a != 0 {
                    self.instruction_pointer = literal as usize;
                }
            }
            Some(4) => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
                // then stores the result in register B. (For legacy reasons, this instruction reads an
                // operand but ignores it.)

                let _ignored = self.next_instruction().expect("operand");
                let result = self.register_b.bitxor(self.register_c);

                if self.debug {
                    println!(
                        "[{}] bxc :: B = xor(B, C) :: xor({}, {}) = {}",
                        self.instruction_pointer - 2,
                        self.register_b,
                        self.register_c,
                        result
                    )
                }

                self.register_b = result;
            }
            Some(5) => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then
                // outputs that value. (If a program outputs multiple values, they are separated by
                // commas.)

                let operand = self.next_instruction().expect("operand");
                let (operand_name, operand_value) =
                    self.decode_operand(operand);
                let result = operand_value % 8;

                if self.debug {
                    println!(
                        "[{}] out :: out({}) => {}",
                        self.instruction_pointer - 2,
                        operand_name,
                        result
                    )
                }
                self.outputs.push(result as u8);
            }
            Some(6) => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the
                // result is stored in the B register. (The numerator is still read from the A register.)

                let instruction = self.next_instruction().expect("operand");
                let (operand_name, operand_value) =
                    self.decode_operand(instruction);
                let result = self.register_a.shr(operand_value);

                if self.debug {
                    println!(
                        "[{}] bdv :: B = A / 2^{} :: {} / 2^{} = {}",
                        self.instruction_pointer - 2,
                        operand_name,
                        self.register_a,
                        operand_value,
                        result
                    )
                }

                self.register_b = result;
            }
            Some(7) => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the
                // result is stored in the C register. (The numerator is still read from the A register.)

                let instruction = self.next_instruction().expect("operand");
                let (operand_name, operand_value) =
                    self.decode_operand(instruction);
                let result = self.register_a.shr(operand_value);

                if self.debug {
                    println!(
                        "[{}] cdv :: C = A / 2^{} :: {} / 2^{} = {}",
                        self.instruction_pointer - 2,
                        operand_name,
                        self.register_a,
                        operand_value,
                        result
                    )
                }

                self.register_c = result;
            }
            Some(invalid) => panic!("invalid instruction: {}", invalid),
            None => {
                self.running = false;
            }
        }
    }

    fn decode_operand(&self, instruction: u8) -> (&str, usize) {
        match instruction {
            literal @ 0..=3 => ("literal", literal as usize),
            4 => ("A", self.register_a),
            5 => ("B", self.register_b),
            6 => ("C", self.register_c),
            7 => panic!("reserved operand found!"),
            invalid => panic!("invalid operand: {}", invalid),
        }
    }

    fn next_instruction(&mut self) -> Option<u8> {
        self.memory
            .get(self.instruction_pointer)
            .map(|instruction| {
                self.instruction_pointer += 1;
                *instruction
            })
    }

    #[allow(dead_code)]
    fn memory_string(&self) -> String {
        self.memory
            .iter()
            .map(|output| format!("{}", output))
            .collect::<Vec<String>>()
            .join(",")
    }

    fn output_string(&self) -> String {
        self.outputs
            .iter()
            .map(|output| format!("{}", output))
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[must_use]
pub fn part1(input: &str) -> String {
    let mut computer = parse(input);
    computer.run();

    computer.output_string()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    // [0] bst :: B = A % 8 = 63687530 % 8 = 2
    // [2] bxl :: B = xor(B, 3) = 1
    // [4] cdv :: C = A / 2^B :: 63687530 / 2^1 = 31843765
    // [6] adv :: A = A / 2^literal :: 63687530 / 3 = 7960941
    // [8] bxl :: B = xor(B, 5) = 4
    //[10] bxc :: B = xor(B, C) :: xor(4, 31843765) = 31843761
    //[12] out :: out(B) => 1
    //[14] jnz :: A = 7960941 => jump 0
    //
    //B = xor(A % 8, 3)
    //C = A / 2^B
    //A = A / 8
    //B = xor(xor(B, 5), C)
    //out(B)
    //jnz(A)
    //
    //A % 8 #=> 0bxx
    //xor(0b00, 3) = 0b11 = 3
    //xor(0b01, 3) = 0b10 = 2
    //xor(0b10, 3) = 0b01 = 1
    //xor(0b11, 3) = 0b00 = 0
    //
    //while A != 0 {
    //  B = (A % 8) ^ 3   # take last 3 bits and flip last two of A, resulting in 0..=7
    //  C = A / 2**B      # shift A right B bits and store in C
    //
    //  output(B ^ 0b101 ^ C)
    //
    //  A = A / 8
    //}
    //
    //A = 35184378340031 0bxxxxxxxxxxxxx111
    //B = 4              0b0000000000000100
    //C = 2199023646251  0bxxxxxxxxxxxxx011
    //output 0b100 ^ 0b101 ^ 0b011 = 0b010
    //
    //0bxxxxxxxx011x111 => 2
    //0bxxxxx   x   xxx => 4

    let computer = parse(input);
    let mut start: usize = 0b000;

    for size in 1..=16 {
        let end: usize = start.shl(size) + 0b111usize;
        let mut a: usize = start;

        let target: Vec<u8> = computer
            .memory
            .iter()
            .rev()
            .take(size)
            .cloned()
            .rev()
            .collect();

        loop {
            if a > end {
                break;
            }

            let mut c = computer.clone();
            c.register_a = a;
            c.run();

            if c.outputs == target {
                //                println!("a={} {:b} output={}", a, a, c.output_string());
                start = a.shl(3);
                break;
            }

            a += 1;
        }
    }

    start.shr(3)
}

fn parse(input: &str) -> Computer {
    let (computer_string, program_string) =
        input.trim_end().split_once("\n\n").unwrap();
    let mut registers: HashMap<&str, usize> = HashMap::new();

    for line in computer_string.lines() {
        let (register_string, value_string) = line.split_once(": ").unwrap();
        let (_, name) = register_string.split_once(" ").unwrap();

        registers.insert(name, value_string.parse::<usize>().unwrap());
    }

    let (_program, instructions_string) =
        program_string.split_once(": ").unwrap();
    let instructions: Vec<u8> = instructions_string
        .split(",")
        .map(|s| s.parse::<u8>().expect("instruction"))
        .collect();

    Computer {
        register_a: *registers.get("A").unwrap(),
        register_b: *registers.get("B").unwrap(),
        register_c: *registers.get("C").unwrap(),
        memory: instructions,
        instruction_pointer: 0,
        outputs: Vec::new(),
        running: true,
        debug: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
    }

    #[test]
    fn part1_examples() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut computer = Computer {
            register_c: 9,
            memory: vec![2, 6],
            ..Default::default()
        };
        computer.run();
        assert_eq!(computer.register_b, 1);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        computer = Computer {
            register_a: 10,
            memory: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };
        computer.run();
        assert_eq!(computer.outputs, vec![0, 1, 2]);

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0
        // and leave 0 in register A.
        computer = Computer {
            register_a: 2024,
            memory: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };
        computer.run();
        assert_eq!(computer.register_a, 0);
        assert_eq!(computer.outputs, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);

        // If register B contains 29, the program 1,7 would set register B to 26.
        computer = Computer {
            register_b: 29,
            memory: vec![1, 7],
            ..Default::default()
        };
        computer.run();
        assert_eq!(computer.register_b, 26);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set
        // register B to 44354.
        computer = Computer {
            register_b: 2024,
            register_c: 43690,
            memory: vec![4, 0],
            ..Default::default()
        };
        computer.run();
        assert_eq!(computer.register_b, 44354);

        assert_eq!(part1(input()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 117440);
    }
}
