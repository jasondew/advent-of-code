def part_one(input)
  initial_state = {buffer: [0], buffer_size: 1, current_position: 0}
  buffer = (1..2017).each.with_object(initial_state) do |index, state|
    insert_index = ((input + state[:current_position]) % state[:buffer_size]) + 1

    if insert_index > state[:buffer_size]
      state[:buffer] << index
    else
      state[:buffer].insert(insert_index, index)
    end

    state[:current_position] = insert_index
    state[:buffer_size] += 1
  end[:buffer]
  puts "After 2017: #{buffer[buffer.index(2017)+1]}"
end

def part_two(input)
  initial_state = {buffer: [0], buffer_size: 1, current_position: 0}
  after_zeros = []

  (1..50_000_000).each.with_object(initial_state) do |index, state|
    insert_index = ((input + state[:current_position]) % state[:buffer_size]) + 1
    after_zeros << index if insert_index == 1
    state[:current_position] = insert_index
    state[:buffer_size] += 1
  end[:buffer]

  puts "After zero: #{after_zeros.last}"
end

example_input = 3
puzzle_input = 324

part_one(example_input)
part_one(puzzle_input)
part_two(puzzle_input)
