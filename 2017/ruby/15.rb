A_FACTOR = 16_807
B_FACTOR = 48_271
MODULO = 2_147_483_647
LOWER_16_MASK = 2**16 - 1

def part_one(input)
  a, b = input
  matches = 0

  40_000_000.times do
    a = next_value(a, A_FACTOR)
    b = next_value(b, B_FACTOR)
    matches += 1 if lower_16_bits_match?(a, b)
  end

  puts "matches: #{matches}"
end

def part_two(input)
  a, b = input
  matches = 0

  5_000_000.times do
    a = next_value_multiple(a, A_FACTOR, 4)
    b = next_value_multiple(b, B_FACTOR, 8)
    matches += 1 if lower_16_bits_match?(a, b)
  end

  puts "matches: #{matches}"
end

def next_value(seed, factor)
  (seed * factor) % MODULO
end

def next_value_multiple(seed, factor, multiple)
  value = next_value(seed, factor)

  loop do
    break if value % multiple == 0
    value = next_value(value, factor)
  end

  value
end

def bitstring(value)
  value.to_s(2).rjust(32, "0")
end


def lower_16_bits_match?(a, b)
  (a & LOWER_16_MASK) == (b & LOWER_16_MASK)
end

example_input = [65, 8921]
puzzle_input = [591, 393]

part_one(example_input) #=> 588
part_one(puzzle_input)

part_two(example_input) # => 309
part_two(puzzle_input)
