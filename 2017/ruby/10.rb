require "ap"
require "active_support/core_ext/array"

def knot_hash_round(lengths, context)
  lengths.each_with_object(context) do |length, context|
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
end

def part_one(input, size)
  lengths = input.strip.split(",").map(&:to_i)
  initial_context = { list: (0..size-1).to_a, position: 0, skip_size: 0 }
  knot_hash = knot_hash_round(lengths, initial_context)

  puts "lengths: #{lengths}"
  puts "final list: #{knot_hash[:list]}"
  puts "multiplied first two: #{knot_hash[:list].first(2).inject(:*)}"
end

def part_two(input)
  lengths = input.unpack("C*") + [17, 31, 73, 47, 23]
  initial_context = { list: (0..255).to_a, position: 0, skip_size: 0 }

  final_context = 64.times.each_with_object(initial_context) do |_, context|
    knot_hash_round(lengths, context)
  end

  hash = final_context[:list].in_groups_of(16).map do |group|
    group.inject(:^).to_s(16).rjust(2, "0")
  end.join

  puts "#{input.inspect} -> #{hash}"
end

part_one("3,4,1,5", 5)
part_one(File.read("../inputs/10").strip, 256)

part_two("")
part_two("AoC 2017")
part_two("1,2,3")
part_two("1,2,4")
part_two(File.read("../inputs/10").strip)
