require "ap"

def part_one(input)
  scanners = parse(input)
  severity = caught_in(scanners).inject(0) do |sum, (depth, range)|
    sum + depth * range
  end
end

def part_two(input)
  scanners = parse(input)
  (0..10_000_000).detect do |delay|
    not caught?(scanners, delay)
  end
end

def caught?(scanners, delay)
  scanners.detect do |depth, range|
    position(depth + delay, range) == 0
  end
end

def caught_in(scanners)
  scanners.select do |depth, range|
    position(depth, range) == 0
  end
end

def parse(input)
  input.split(/\n/).map {|line| line.split(": ").map(&:to_i) }
end

def position(time, range)
  modulus = range - 1
  if (x = time % (modulus * 2)) < modulus
    x
  else
    (modulus * 2) - x
  end
end

example_input = "0: 3\n1: 2\n4: 4\n6: 4"
puzzle_input = File.read("../inputs/13").chomp

puts "Example part 1: severity = #{part_one(example_input)}"
puts "Puzzle part 1: severity = #{part_one(puzzle_input)}"

puts "Example part 2: delay = #{part_two(example_input)}"
puts "Puzzle part 2: delay = #{part_two(puzzle_input)}"
