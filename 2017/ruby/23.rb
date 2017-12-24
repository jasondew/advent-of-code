require "ap"
require "prime"

Set = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] = evaluate(registers, value)
    registers[:pc] += 1
  end
end

Sub = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] -= evaluate(registers, value)
    if register == :h
      puts registers[:h]
    end
    registers[:pc] += 1
  end
end

Multiply = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] *= evaluate(registers, value)
    registers[:mul_count] += 1
    registers[:pc] += 1
  end
end

Jump = Struct.new(:register_or_value, :value) do
  def execute(registers)
    if evaluate(registers, register_or_value) != 0
      registers[:pc] += evaluate(registers, value)
    else
      registers[:pc] += 1
    end
  end
end

def parse(input)
  input.split(/\n/).map {|line| parse_instruction(line) }
end

def parse_instruction(line)
  parts = line.split(/ /)
  case parts[0]
  when "set" then Set.new(parts[1].to_sym, parts[2])
  when "sub" then Sub.new(parts[1].to_sym, parts[2])
  when "mul" then Multiply.new(parts[1].to_sym, parts[2])
  when "jnz" then Jump.new(parts[1], parts[2])
  else raise "Unrecognized instruction: #{parts.inspect}"
  end
end

def initial_registers
  %i{a b c d e f g h pc}.
    map {|register| [register, 0] }.
    to_h.
    merge(mul_count: 0)
end

def evaluate(registers, value_or_register)
  if value_or_register =~ /-?\d+/
    Integer(value_or_register)
  else
    if value = registers[value_or_register.to_sym]
      value
    else
      raise "Invalid value_or_register: #{value_or_register.inspect}"
    end
  end
end

def execute(instructions, registers=initial_registers)
  loop do
    execute_instruction(instructions, registers)
    break if halt_condition?(registers, instructions)
  end

  registers
end

def execute_instruction(instructions, registers)
  instruction = instructions[registers[:pc]]

  if $debug
    puts "#{registers[:pc]}: #{instruction.inspect}"
    puts "-"*80
  end

  instruction.execute(registers)

  if $debug
    p registers
    puts
  end
end

def halt_condition?(registers, instructions)
  registers[:pc] >= instructions.size || registers[:pc] < 0
end

def part_one(input)
  instructions = parse(input)
  registers = execute(instructions)
  puts "mul count: #{registers[:mul_count]}"
end

def part_two(input)
  puts (106_500..123_500).step(17).count {|b| not Prime.prime?(b) }
end

puzzle_input = File.read("../inputs/23").chomp

$debug = true
#part_one(puzzle_input)
part_two(puzzle_input)

__END__

 0: b = 65
 1: c = b
 2: if a != 0 goto 4
 3: goto 5
 4: b *= 100
 5: b += 100_000
 6: c = b
 7: c += 17_000
 8: f = 1
 9: d = 2
10: e = 2
11: g = d
12: g *= e
13: g -= b
14: if g != 0 goto 16
15: f = 0
16: e += 1
17: g = e
18: g -= b
19: if g != 0 goto 11
20: d += 1
21: g = d
22: g -= b
23: if g != 0 goto 10
24: if f != 0 goto 26
25: h += 1
26: g = b
27: g -= c
28: if g != 0 goto 30
29: halt
30: b += 17
31: goto 8

---

b, c = 65

if a == 0
  b += 100_000
else
  b = b * 100 + 100_000
end

c = b + 17_000

loop do
  f = 1
  d = 2

  do
    e = 2

    do
      if d * e - b == 0
        f = 0
      end

      e += 1
    until e == b

    d += 1
  until d == b

  h += 1 if f == 0

  halt if b == c
  b += 17
end

---

assume a=1

b = 106_500
c = 123_500

for(b=106_500; b != c; b+=17) {
  f = 1
  d = 2

  loop {
    e = 2

    loop {
      if b == d * e { f = 0 }
      e += 1
    } until e == b

    d += 1
  } until d == b

  h += 1 if f == 0
}

---


for b from 106_500 to 123_500 by 17 do
  f = 1
  for d from 2 to b do
    for e from 2 to b do
      if b == d * e then f = 0
    end
  end
  if f == 0 then h += 1
end
