require "ap"

Play = Struct.new(:register) do
  def execute(registers)
    registers[:lp] = registers[register]
    registers[:ip] += 1
  end
end

Set = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] = value
    registers[:ip] += 1
  end
end

Add = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] += value
    registers[:ip] += 1
  end
end

Multiply = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] *= value
    registers[:ip] += 1
  end
end

Modulo = Struct.new(:register, :value) do
  def execute(registers)
    registers[register] = registers[register] % value
    registers[:ip] += 1
  end
end

Recover = Struct.new(:register) do
  def execute(registers)
    last_played = registers[:lp]
    puts last_played unless last_played.zero?
    registers[:ip] += 1
  end
end

Jump = Struct.new(:register, :offset) do
  def execute(registers)
    if registers[register] > 0
      registers[:ip] += offset
    else
      registers[:ip] += 1
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
  when "set" then Set.new(parts[1].to_sym, parts[2].to_i)
  when "add" then Add.new(parts[1].to_sym, parts[2].to_i)
  when "mul" then Multiply.new(parts[1].to_sym, parts[2].to_i)
  when "mod" then Modulo.new(parts[1].to_sym, parts[2].to_i)
  when "rcv" then Recover.new(parts[1].to_sym)
  when "jgz" then Jump.new(parts[1].to_sym, parts[2].to_i)
  end
end

def initial_registers
  %i{a b c d e f g h i j k l m n o p ip lp}.map {|register| [register, 0] }.to_h
end

def execute(instructions)
  registers = initial_registers

  loop do
    instruction = instructions[registers[:ip]]
    p instruction
    instruction.execute(registers)
    p registers[:ip]
    break if registers[:ip] > instructions.size
  end
end

def part_one(input)
  instructions = parse(input)
  ap instructions
  execute(instructions)
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

part_one(example_input)
