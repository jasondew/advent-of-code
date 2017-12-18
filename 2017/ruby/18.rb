require "ap"
require "pry"

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

Jump = Struct.new(:register_or_value, :value) do
  def execute(registers)
    if evaluate(registers, register_or_value) > 0
      registers[:pc] += evaluate(registers, value)
    else
      registers[:pc] += 1
    end
  end
end

Play = Struct.new(:register) do
  def execute(registers)
    registers[:outgoing] << registers.fetch(register)
    registers[:pc] += 1
  end
end

Recover = Struct.new(:register) do
  def execute(registers)
    if registers[register].nonzero? && registers[:outgoing].size.nonzero?
      puts "Recovered #{registers[:outgoing].last}"
      registers[:pc] = -1
    else
      registers[:pc] += 1
    end
  end
end

Send = Struct.new(:register) do
  def execute(registers)
    registers[:outgoing] << evaluate(registers, register)
    registers[:outgoing_count] += 1
    registers[:pc] += 1
  end
end

Receive = Struct.new(:register) do
  def execute(registers)
    if registers[:incoming].size.nonzero?
      registers[:waiting] = false
      registers[register] = registers[:incoming].shift
      registers[:pc] += 1
    else
      registers[:waiting] = true
    end
  end
end

def parse(input, send_klass, receive_klass)
  input.split(/\n/).map do |line|
    parse_instruction(line, send_klass, receive_klass)
  end
end

def parse_instruction(line, send_klass, receive_klass)
  parts = line.split(/ /)
  case parts[0]
  when "set" then Set.new(parts[1].to_sym, parts[2])
  when "add" then Add.new(parts[1].to_sym, parts[2])
  when "mul" then Multiply.new(parts[1].to_sym, parts[2])
  when "mod" then Modulo.new(parts[1].to_sym, parts[2])
  when "jgz" then Jump.new(parts[1], parts[2])
  when "snd" then send_klass.new(parts[1].to_sym)
  when "rcv" then receive_klass.new(parts[1].to_sym)
  else raise "Unrecognized instruction: #{parts.inspect}"
  end
end

def initial_registers
  %i{a b c d e f g h i j k l m n o p pc}.
    map {|register| [register, 0] }.
    to_h.
    merge(incoming: [], outgoing: [], outgoing_count: 0, waiting: false)
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
  instructions = parse(input, Play, Recover)
  registers = execute(instructions)
end

def part_two(input)
  instructions = parse(input, Send, Receive)

  program_zero = initial_registers.merge(p: 0)
  program_one = initial_registers.merge(p: 1)

  1_000_000.times do
    execute_instruction(instructions, program_zero)
    execute_instruction(instructions, program_one)

    if program_zero[:outgoing].size.nonzero?
      program_one[:incoming] += program_zero[:outgoing]
      program_zero[:outgoing] = []
    end

    if program_one[:outgoing].size.nonzero?
      program_zero[:incoming] += program_one[:outgoing]
      program_one[:outgoing] = []
    end

    break if (halt_condition?(program_zero, instructions) && halt_condition?(program_one, instructions)) ||
             (program_zero[:waiting] && program_one[:waiting])
  end

  puts "Program 1 sends: #{program_one[:outgoing_count]}"
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
part_one(example_input)
part_one(puzzle_input)
part_two(puzzle_input)
