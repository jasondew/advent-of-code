require "ap"
require "pry"

Component = Struct.new(:id, :port_a, :port_b) do
  def inspect
    "#{id}:#{port_a}/#{port_b}"
  end

  def matches?(port)
    port_a == port or port_b == port
  end

  def size
    port_a + port_b
  end

  def swapped
    self.class.new self.id, self.port_b, self.port_a
  end
end

def part_one_and_two(input)
  components = parse(input)

  starters = components.select {|component| component.matches?(0) }

  chains = starters.map do |starter|
    find_longest_bridge([starter.id], starter.port_b, components)
  end.flatten

  strongest_chain = chains.max_by {|chain| chain[:strength] }
  longest_length = chains.map {|chain| chain[:ids].size }.max
  strongest_longest_chain = chains.
                              select {|chain| chain[:ids].size == longest_length }.
                              max_by {|chain| chain[:strength] }

  puts "Strongest chain: #{strongest_chain[:ids]}"
  puts "Strength: #{strongest_chain[:strength]}"
  puts "Strongest and longest chain: #{strongest_longest_chain[:ids]}"
  puts "Strength: #{strongest_longest_chain[:strength]}"
end

def find_longest_bridge(ids, connector, components)
  matching_components = components.select do |component|
    component.matches?(connector) && !ids.include?(component.id)
  end

  if matching_components.empty?
    {ids: ids, strength: components.select {|component| ids.include?(component.id) }.sum(&:size) }
  else
    matching_components.map do |component|
      next_connector = if component.port_a == connector
                         component.port_b
                       else
                         component.port_a
                       end

      find_longest_bridge(ids + [component.id], next_connector, components)
    end
  end
end

def parse(input)
  input.split(/\n/).map.with_index do |line, index|
    Component.new(index + 1, *line.split("/").map(&:to_i))
  end
end

example_input = <<-EOT
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
EOT
puzzle_input = File.read("../inputs/24").chomp

#part_one_and_two(example_input)
part_one_and_two(puzzle_input)
