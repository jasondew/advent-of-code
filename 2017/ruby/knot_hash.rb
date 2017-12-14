require "active_support/core_ext/array"

class KnotHash
  attr_accessor :input

  def self.run(input)
    new(input).run
  end

  def initialize(input)
    @input = input
  end

  def run
    lengths = input.unpack("C*") + [17, 31, 73, 47, 23]
    initial_context = { list: (0..255).to_a, position: 0, skip_size: 0 }

    final_context = 64.times.each_with_object(initial_context) do |_, context|
      knot_hash_round(lengths, context)
    end

    final_context[:list].in_groups_of(16).map do |group|
      group.inject(:^).to_s(16).rjust(2, "0")
    end.join
  end

  private

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
end
