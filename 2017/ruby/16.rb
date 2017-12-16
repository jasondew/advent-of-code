require "ap"

Spin = Struct.new(:count) do
  def execute(programs)
    size = programs.size
    programs[(size - count)..size] + programs[0..(size - count - 1)]
  end
end

Exchange = Struct.new(:position_a, :position_b) do
  def execute(programs)
    programs[position_a], programs[position_b] =
      programs[position_b], programs[position_a]
    programs
  end
end

Partner = Struct.new(:program_a, :program_b) do
  def execute(programs)
    Exchange.new(programs.index(program_a), programs.index(program_b)).
      execute(programs)
  end
end

def part_one(input, size)
  moves = parse(input)

  final_state = moves.inject(initial_state size) do |state, move|
#    puts "move = #{move}"
#    puts state.inspect
    move.execute(state) #.tap {|updated_state| puts updated_state.inspect }
  end

  puts final_state.join
end

def part_two(input, size)
  moves = parse(input)

  final_state = 1_000_000_000.times.inject(initial_state size) do |state|
    moves.inject(state) {|state, move| move.execute(state) }
  end

  puts final_state.join
end

def initial_state(size)
  (97..(97+size-1)).map(&:chr)
end

def parse(input)
  input.split(",").map do |move|
    case move[0]
    when "s" then Spin.new(move[1..-1].to_i)
    when "x" then Exchange.new(*move[1..-1].split("/").map(&:to_i))
    when "p" then Partner.new(move[1], move[3])
    else raise "Unknown move found: #{move.inspect}"
    end
  end
end

example_input = "s1,x3/4,pe/b"
puzzle_input = File.read("../inputs/16").chomp

part_one(example_input, 5)
part_one(puzzle_input, 16)

part_two(puzzle_input, 16)
