require "ap"
require "pry"

Play = Struct.new(:register) do
  def execute(registers)
    registers[:lp] = registers.fetch(register)
    registers[:pc] += 1
  end
end

Set = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] = evaluate(registers, value)
    registers[:pc] += 1
  end
end

Add = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] += evaluate(registers, value)
    registers[:pc] += 1
  end
end

Multiply = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] *= evaluate(registers, value)
    registers[:pc] += 1
  end
end

Modulo = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] %= evaluate(registers, value)
    registers[:pc] += 1
  end
end

Recover = Struct.new(:register) do
  def execute(registers)
    if registers[register].nonzero? && registers[:lp].nonzero?
      puts "Recovered #{registers[:lp]}"
      registers[:pc] = -1
    else
      registers[:pc] += 1
    end
  end
end

Jump = Struct.new(:register_or_value, :value) do
  def execute(registers)
    if evaluate(registers, register_or_value) > 0
      registers[:pc] += evaluate(registers, value)
    else
      registers[:pc] += 1
    end
  end
end

def parse(input)
  input.split(/\n/).map(&method(:parse_instruction))
end

def parse_instruction(input)
  parts = input.split(/ /)
  case parts[0]
  when "snd" then Play.new(parts[1].to_sym)
  when "set" then Set.new(parts[1].to_sym, parts[2])
  when "add" then Add.new(parts[1].to_sym, parts[2])
  when "mul" then Multiply.new(parts[1].to_sym, parts[2])
  when "mod" then Modulo.new(parts[1].to_sym, parts[2])
  when "rcv" then Recover.new(parts[1].to_sym)
  when "jgz" then Jump.new(parts[1], parts[2])
  else raise "Unrecognized instruction: #{parts.inspect}"
  end
end

def initial_registers
  %i{a b c d e f g h i j k l m n o p pc lp}.map {|register| [register, 0] }.to_h
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

def execute(instructions)
  registers = initial_registers

  loop do
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

    break if halt_condition?(registers, instructions)
  end

  registers
end

def halt_condition?(registers, instructions)
  registers[:pc] >= instructions.size || registers[:pc] < 0
end

def part_one(input)
  instructions = parse(input)
  registers = execute(instructions)
end

example_input = <<-EOT
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
EOT
puzzle_input = File.read("../inputs/18").chomp

$debug = false
#part_one(example_input)
part_one(puzzle_input)
