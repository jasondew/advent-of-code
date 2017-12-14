require "ap"
require_relative "knot_hash"

def part_one(input)
  grid(input).map(&:join).join.count("1")
end

def part_two(input)
  grid = grid(input)
  regions = 0

  grid.each_with_index do |row, y|
    row.each_with_index do |cell, x|
      next if cell.zero?

      adjacent_region = 0

      if x > 0
        adjacent_region = grid[y][x-1]
      end

      if adjacent_region.zero? && y > 0
        adjacent_region = grid[y-1][x]
      end

      if adjacent_region.zero?
        adjacent_region = (regions += 1)
      end

      grid[y][x] = adjacent_region
    end
  end

  loop do
    print "."
    break if unalias(grid).zero?
  end
  puts

#  grid[0..7].each {|row| row[0..20].each {|cell| print ("%3d " % cell) }; puts }

  grid.flatten.sort.uniq.size - 1
end

def unalias(grid)
  aliases = {}
  grid.each_with_index do |row, y|
    row.each_with_index do |cell, x|
      next if cell.zero?

      if x > 0 && (adjacent_region = grid[y][x-1]).nonzero? && adjacent_region != cell
        add_alias(aliases, cell, adjacent_region)
      end

      if (adjacent_region = grid[y][x+1]) && adjacent_region.nonzero? && adjacent_region != cell
        add_alias(aliases, cell, adjacent_region)
      end

      if y > 0 && (adjacent_region = grid[y-1][x]).nonzero? && adjacent_region != cell
        add_alias(aliases, cell, adjacent_region)
      end

      if (next_row = grid[y+1]) && (adjacent_region = next_row[x]) && adjacent_region.nonzero? && adjacent_region != cell
        add_alias(aliases, cell, adjacent_region)
      end
    end
  end

  grid.each_with_index do |row, y|
    row.each_with_index do |cell, x|
      if (region = aliases[cell])
        grid[y][x] = region
      end
    end
  end

  aliases.size
end

def add_alias(aliases, region1, region2)
  return if aliases[region2]

  if (region = aliases[region1])
    aliases[region2] = region
  else
    aliases[region2] = region1
  end
end

def grid(input)
  (0..127).map do |row|
    knot_hash("#{input}-#{row}").chars.map do |char|
      char.to_i(16).to_s(2).rjust(4, "0")
    end.join.chars.map(&:to_i)
  end
end

def knot_hash(input)
  KnotHash.run(input)
end


example_input = "flqrgnkx"
puzzle_input = File.read("../inputs/14").chomp

puts "Example part 1: ones = #{part_one(example_input)}"
puts "Puzzle part 1: ones = #{part_one(puzzle_input)}"

puts "Example part 2: regions = #{part_two(example_input)}"
puts "Puzzle part 2: regions = #{part_two(puzzle_input)}"
