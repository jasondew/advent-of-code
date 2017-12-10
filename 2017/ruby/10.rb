require "ap"

def part_one(input, size)
  lengths = input.strip.split(" ").map(&:to_i)
  initial_list = (0..size-1).to_a
  context = { list: initial_list, position: 0, skip_size: 0 }

  final_context = lengths.each_with_object(context) do |length, context|
    puts "length: #{length} context: #{context}"
    list_length = context[:list].length
    from = context[:position]
    to = context[:position] + length

    puts "from: #{from} to: #{to} list_length: #{list_length}"

    if to == from + 1
      # no-op
    elsif to >= list_length
      to_end = list_length - 1
      to_wrapped = to % list_length

      puts "(#{from}, #{to_end}) -> (0, #{to_wrapped})"
      puts "(0, #{to_wrapped}) -> (0, #{to_wrapped})"

      context[:list][0, to_wrapped], context[:list][from, to_end] =
        context[:list][from, to_end].reverse, context[:list][0, to_wrapped].reverse
    else
      context[:list][from, to] = context[:list][from, to].reverse
    end

    context[:position] = (context[:position] + length + context[:skip_size]) % list_length
    context[:skip_size] += 1
  end

  puts final_context[:list].inspect

  final_context[:list].first(2).inject(:*)
end

input = "3 4 1 5"
puts "multiplied first two: #{part_one(input, 5)}"

#input = File.read("../inputs/10").chomp
#puts "multiplied first two: #{part_one(input, 256)}"
