require "ap"

Instruction = Struct.new(:command, :conditional) do
  def evaluate(context)
    command.evaluate(context) if conditional.evaluate(context)
  end
end

Command = Struct.new(:register, :operation, :amount) do
  def evaluate(context)
    case operation
    when "inc" then context[register] += amount
    when "dec" then context[register] -= amount
    else raise "unknown command operation: #{operation}"
    end
  end
end

Conditional = Struct.new(:register, :comparator, :value) do
  def evaluate(context)
    case comparator
    when ">" then context[register] > value
    when ">=" then context[register] >= value
    when "<" then context[register] < value
    when "<=" then context[register] <= value
    when "==" then context[register] == value
    when "!=" then context[register] != value
    else raise "unknown conditional comaparator: #{comparator}"
    end
  end
end

class Context < Hash
  attr_accessor :maximum_amount

  def initialize(*)
    super
    self.maximum_amount = 0
  end

  def []=(register, amount)
    super
    self.maximum_amount = [maximum_amount, self[register]].compact.max
  end
end

def parse(input)
  input.lines.map do |line|
    tokens = line.split(/ /, 7)

    tokens[3] == "if" or raise "unknown conditional found: #{conditional}"
    command = Command.new(tokens[0], tokens[1], tokens[2].to_i)
    conditional = Conditional.new(tokens[4], tokens[5], tokens[6].to_i)

    Instruction.new(command, conditional)
  end
end

def evaluate(instructions)
  context = Context.new {|h, k| h[k] = 0 }

  instructions.each_with_object(context) do |instruction, context|
    instruction.evaluate(context)
  end
end


puts "Example:\n#{'-'*80}"
input = <<-EOT
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
EOT
instructions = parse(input.chomp)
ending_context = evaluate(instructions)
ap ending_context
puts "Maximum final register value: #{ending_context.values.max}"
puts "Maximum register value: #{ending_context.maximum_amount}"

puts "\n"

puts "Puzzle:\n#{'-'*80}"
input = File.read("../inputs/08").chomp
instructions = parse(input)
ending_context = evaluate(instructions)
ap ending_context.sort_by {|k, v| k }.to_h
puts "Maximum final register value: #{ending_context.values.max}"
puts "Maximum register value: #{ending_context.maximum_amount}"
