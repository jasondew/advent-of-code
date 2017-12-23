require "ap"

Carrier = Struct.new(:position, :direction) do
  attr_accessor :infection_count

  def initialize(*)
    super
    self.infection_count = 0
  end

  def infection
    self.infection_count += 1
  end

  def move_forward
    x, y = position
    self.position = case direction
                    when :north then [     x, y - 1 ]
                    when :east  then [ x + 1,     y ]
                    when :south then [     x, y + 1 ]
                    when :west  then [ x - 1,     y ]
                    end
  end

  def turn_left
    self.direction = case direction
                     when :north then :west
                     when :east  then :north
                     when :south then :east
                     when :west  then :south
                     end
  end

  def turn_right
    self.direction = case direction
                     when :north then :east
                     when :east  then :south
                     when :south then :west
                     when :west  then :north
                     end
  end

  def turn_around
    self.direction = case direction
                     when :north then :south
                     when :east  then :west
                     when :south then :north
                     when :west  then :east
                     end
  end
end

def part_one(input)
  infected_list, starting_position = parse(input)
  carrier = Carrier.new(starting_position, :north)

  10_000.times { part_one_burst(infected_list, carrier) }

  puts "Infections: #{carrier.infection_count}"
end

def part_one_burst(infected_list, carrier)
  if infected_list.include?(carrier.position)
    carrier.turn_right
    infected_list.delete(carrier.position)
  else
    carrier.turn_left
    carrier.infection
    infected_list << carrier.position
  end

  carrier.move_forward
end

def part_two(input)
  infected_list, starting_position = parse(input)
  grid = infected_list.each_with_object({}) do |infected_position, hash|
    hash[infected_position] = :infected
  end
  carrier = Carrier.new(starting_position, :north)

  10_000_000.times { part_two_burst(grid, carrier) }

  puts "Infections: #{carrier.infection_count}"
end

def part_two_burst(grid, carrier)
  case grid[carrier.position]
  when nil then
    grid[carrier.position] = :weakend
    carrier.turn_left
  when :weakend then
    grid[carrier.position] = :infected
    carrier.infection
  when :infected then
    grid[carrier.position] = :flagged
    carrier.turn_right
  when :flagged then
    grid.delete(carrier.position)
    carrier.turn_around
  end

  carrier.move_forward
end

def parse(input)
  lines = input.split(/\n/)
  infected_list = lines.each.with_index.with_object([]) do |(line, row), infected|
    line.chars.each.with_index do |cell, column|
      infected << [column, row] if cell == "#"
    end
  end
  middle = (lines.size / 2.0).floor
  starting_position = [middle, middle]

  [infected_list, starting_position]
end

example_input = <<-EOT
..#
#..
...
EOT
puzzle_input = File.read("../inputs/22").chomp

part_one(puzzle_input)
part_two(puzzle_input)
