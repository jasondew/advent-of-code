input = "01110110101001000"
output = input
desired_size = 35651584

def flip(string)
  string.each_char.map {|char| char == "0" ? "1" : "0" }.join
end

while output.size <= desired_size do
  output = output + "0" + flip(output.reverse)
end

final_output = output[0..(desired_size - 1)]

def checksum(string)
  output = string

  while output.size % 2 == 0 do
    output = output.each_char.each_slice(2).map {|chars| chars.uniq.size == 1 ? "1" : "0" }.join
  end

  output
end

puts "final_output size: #{final_output.size}"
puts "checksum = #{checksum final_output}"
