input = <<-EOT
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
EOT

class Node
  attr_accessor :name, :weight, :children

  def initialize(name, weight, children)
    @name, @weight, @children = name, weight.to_i, children
  end

  def inspect
    "#{name} (#{weight} / #{total_weight})"
  end

  def total_weight
    @total_weight ||= weight + (children.map(&:total_weight).inject(:+) || 0)
  end
end

def find_bottom(nodes)
  candidates = nodes.reject {|node| node.children.empty? }
  candidates.detect do |node|
    not candidates.detect do |other_node|
      other_node.name != node.name and other_node.children.include?(node.name)
    end
  end
end

def find_wrong_weight(nodes, bottom)
  tree = build_tree(bottom, nodes)
#  print_tree(tree)

  find_unmatched(tree, tree.total_weight)
end

def print_tree(node, index=0)
  print_node(node, index)
  node.children.each {|child| print_tree(child, index + 1) }
end

def print_node(node, index)
  puts "#{"\t" * index}#{node.inspect}"
end

def find_unmatched(node, expected_total_weight)
  return if node.children.empty?

  total_weights = node.children.map(&:total_weight)

  if total_weights.uniq.size > 1
    weight_hash = node.children.each_with_object(Hash.new {|h, k| h[k] = [] }) do |child, hash|
      hash[child.total_weight] << child 
    end
    correct_total_weight = weight_hash.detect {|weight, children| children.size > 1 }.first
    _, odd_child = weight_hash.detect {|weight, children| children.size == 1 }

    find_unmatched(odd_child.first, correct_total_weight)
  else
    correct_weight = node.weight - (node.total_weight - expected_total_weight)
    puts "#{node.inspect} should have weight: #{correct_weight}"
  end
end

def build_tree(parent, nodes)
  parent.children = parent.children.map do |child_name|
    child = nodes.detect {|node| node.name == child_name }
    build_tree(child, nodes)
  end

  parent
end

def parse_nodes(input)
  input.lines.map do |line|
    name, weight, children = parse_node(line.strip)
    Node.new(name, weight, children)
  end
end

def parse_node(line)
  name_and_weight, child_list = line.split("->")
  name, weight = name_and_weight.split(/[ ()]+/)

  if child_list
    children = child_list.strip.split(", ")

    [name, weight, children]
  else
    [name, weight, []]
  end
end

require "ap"

puts "Part 1:\n#{'-'*80}"
nodes = parse_nodes(input.chomp)
bottom = find_bottom(nodes)

puts "bottom: #{bottom.name}"
find_wrong_weight(nodes, bottom)

puts "\n"

puts "Part 2:\n#{'-'*80}"
input = File.read("../inputs/07").chomp
nodes = parse_nodes(input)
bottom = find_bottom(nodes)

puts "bottom: #{bottom.name}"
find_wrong_weight(nodes, bottom)
