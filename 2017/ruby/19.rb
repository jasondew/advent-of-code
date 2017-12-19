require "ap"
require "pry"

Maze = Struct.new(:grid) do
  def entry_point
    column = grid.first.index("|")
    [0, column]
  end

  def at(row, column)
    grid.fetch(row, [])[column]
  end

  def next_location(location, direction)
    row, column = location

    new_location = case direction
    when :north then [row - 1, column]
    when :south then [row + 1, column]
    when :east  then [row    , column + 1]
    when :west  then [row    , column - 1]
    end

    [new_location, at(*new_location)]
  end

  def next_direction(location, direction)
    case direction
    when :north, :south
      possible_location, contents = next_location(location, :east)

      if [" ", nil].include?(contents)
        :west
      else
        :east
      end
    when :east, :west
      possible_location, contents = next_location(location, :north)

      if [" ", nil].include?(contents)
        :south
      else
        :north
      end
    end
  end
end

def part_one(input)
  maze = parse(input)
  location = maze.entry_point
  direction = :south
  letters = []
  steps = 0

  (1..100_000).each do |index|
    location, contents = maze.next_location(location, direction)
    steps += 1

    if $debug
      puts "i: #{index} at: #{location.inspect} contents=#{contents.inspect}"
    end

    case contents
    when " ", nil then break
    when /[A-Z]/  then letters << contents
    when "+"      then direction = maze.next_direction(location, direction)
    end
  end

  puts "Letters: #{letters.join} Steps: #{steps}"
end

def parse(input)
  Maze.new(input.split(/\n/).map {|line| line.split(//) })
end

example_input = <<-EOT
     |         
     |  +--+   
     A  |  C   
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
EOT

puzzle_input = File.read("../inputs/19").chomp

$debug=false
part_one(example_input)
part_one(puzzle_input)
