def repeating_sum(string, offset: 1)
  chars = string.chars
  length = string.length

  chars.each.with_index.inject(0) do |sum, (char, index)|
    if char == chars[(index + offset) % length]
      sum + char.to_i
    else
      sum
    end
  end
end

input = File.read("input").chomp
puts repeating_sum(input)
puts repeating_sum(input, offset: input.length / 2)
