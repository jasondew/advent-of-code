require "ap"

def part_one(input, size)
  lengths = input.strip.split(",").map(&:to_i)
  p lengths
  initial_list = (0..size-1).to_a
  context = { list: initial_list, position: 0, skip_size: 0 }

  final_context = lengths.each_with_object(context) do |length, context|
    puts "length: #{length}"

    list = context[:list]
    list_length = list.length
    from = context[:position]
    to = context[:position] + length - 1

    if to >= list_length
      to_end = list_length - 1
      to_wrapped = to % list_length
      new_sequence = (list[from..to_end] + list[0..to_wrapped]).reverse

      list[from..to_end] = new_sequence[0..(to_end-from)]
      list[0..to_wrapped] = new_sequence[(to_end-from+1)..(new_sequence.length-1)]
    else
      list[from..to] = list[from..to].reverse
    end

    context[:position] = (context[:position] + length + context[:skip_size]) % list_length
    context[:skip_size] += 1
  end

  puts "final list: #{final_context[:list]}"

  final_context[:list].first(2).inject(:*)
end

input = "3,4,1,5"
puts "multiplied first two: #{part_one(input, 5)}"

input = File.read("../inputs/10").chomp
puts "multiplied first two: #{part_one(input, 256)}"
