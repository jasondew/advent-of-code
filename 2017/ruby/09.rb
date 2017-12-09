# Ruby doesn't have TCO, so have to `export RUBY_THREAD_VM_STACK_SIZE=10000000`

require "ap"

Group = Struct.new(:score)

def score(input)
  groups = parse(input, 0, :normal, 1, [])
  groups.inject(0) {|sum, group| sum + group.score }
end

def parse(input, index, state, level, groups)
  return groups if index >= input.size

  if input[index] == "!" then
    parse(input, index + 2, state, level, groups)
  else
    if state == :garbage then
      if input[index] == ">" then
        parse(input, index + 1, :normal, level, groups)
      else
        parse(input, index + 1, state, level, groups)
      end
    else
      case input[index]
      when "{" then parse(input, index + 1, state, level + 1, groups << Group.new(level))
      when "}" then parse(input, index + 1, state, level - 1, groups)
      when "<" then parse(input, index + 1, :garbage, level, groups)
      when "," then parse(input, index + 1, state, level, groups)
      else
        puts "Skipping unrecognized character: #{input[index].inspect}"
        parse(input, index + 1, state, level, groups)
      end
    end
  end
end

def garbage_size(input)
  parse_garbage(input, 0, :normal, 0)
end

def parse_garbage(input, index, state, garbage_count)
  return garbage_count if index >= input.size

  if input[index] == "!" then
    parse_garbage(input, index + 2, state, garbage_count)
  else
    if state == :garbage then
      if input[index] == ">" then
        parse_garbage(input, index + 1, :normal, garbage_count)
      else
        parse_garbage(input, index + 1, state, garbage_count + 1)
      end
    else
      if input[index] == "<" then
        parse_garbage(input, index + 1, :garbage, garbage_count)
      else
        parse_garbage(input, index + 1, state, garbage_count)
      end
    end
  end
end

def test(input, expected_output, method=:score)
  actual_output = send(method, input)
  puts "expecting #{method}(#{input}) == #{expected_output} and it is #{actual_output} #{expected_output == actual_output ? '✅' : '❌'}"
end

puts "Examples:\n#{'-'*80}"
test("{}", 1)
test("{{{}}}", 6)
test("{{},{}}", 5)
test("{{{},{},{{}}}}", 16)
test("{<{},{},{{}}>}", 1)
test("{<a>,<a>,<a>,<a>}", 1)
test("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9)
test("{{<!!>},{<!!>},{<!!>},{<!!>}},", 9)
test("{{<a!>},{<a!>},{<a!>},{<ab>}},", 3)

test("<>", 0, :garbage_size)
test("<random characters>", 17, :garbage_size)
test("<<<<>", 3, :garbage_size)
test("<{!>}>", 2, :garbage_size)
test("<!!>", 0, :garbage_size)
test("<!!!>>", 0, :garbage_size)
test('<{o"i!a,<{i<a>', 10, :garbage_size)

puts "\n"

puts "Puzzle:\n#{'-'*80}"
input = File.read("../inputs/09").chomp
puts "score: #{score input}"
puts "garbage: #{garbage_size input}"
