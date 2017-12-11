require "ap"

def part_one(input)
  final_context = input.split(",").each_with_object({cell: [0,0,0], max_distance: 0}) do |direction, context|
    x, y, z = context[:cell]

    context[:cell] = case direction
    when "n"  then [    x, y + 1, z - 1]
    when "ne" then [x + 1,     y, z - 1]
    when "se" then [x + 1, y - 1,     z]
    when "s"  then [    x, y - 1, z + 1]
    when "sw" then [x - 1,     y, z + 1]
    when "nw" then [x - 1, y + 1,     z]
    else raise "Invalid direction: #{direction}"
    end

    context[:max_distance] = [context[:max_distance], distance(context[:cell])].max
  end

  puts "final_cell: #{final_context[:cell]}"
  puts "distance: #{distance(final_context[:cell])}"
  puts "max_distance: #{final_context[:max_distance]}"
end

def distance(cell)
  x, y, z = cell
  (x.abs + y.abs + z.abs) / 2
end

part_one("ne,ne,ne")
part_one("ne,ne,sw,sw")
part_one("ne,ne,s,s")
part_one("se,sw,se,sw,sw")
part_one(File.read("../inputs/11").strip)
