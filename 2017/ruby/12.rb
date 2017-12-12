require "ap"
require "set"

def part_one(input)
  connections = parse(input)

  puts "connected to 0: #{connected_to(connections, 0).size}"
end

def part_two(input)
  connections = parse(input)

  puts "total groups: #{groups(connections).size}"
end

def groups(connections)
  ids(connections).each_with_object({}) do |id, hash|
    next if hash.values.any? {|ids| ids.member?(id) }
    hash[id] = connected_to(connections, id)
  end
end

def ids(connections)
   connections.each_with_object([]) do |(from, tos), ids|
    ids << from
    ids += tos
   end.sort.uniq
end

def connected_to(connections, id, seen=Set.new)
  connections[id].each do |connected_id|
    next if seen.member?(connected_id)
    seen << connected_id
    connected_to(connections, connected_id, seen)
  end
  seen
end

def parse(input)
   input.split(/\n/).each_with_object({}) do |line, connections|
    from, tos = line.split(" <-> ")
    connections[from.to_i] = tos.split(", ").map(&:to_i)
  end
end

example_input = <<-EOT.chomp
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
EOT

puzzle_input = File.read("../inputs/12").chomp

part_one(example_input)
part_one(puzzle_input)

part_two(example_input)
part_two(puzzle_input)
