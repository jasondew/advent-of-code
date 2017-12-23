require "ap"
require "pry"
require "active_support/core_ext/array"

def part_one(input)
  rules = parse(input)
  pattern = initial_pattern

  18.times do
    size = pattern.split("/").first.size
    puts "\n"
    puts "size = #{size}"
    puts "="*80

    if (size % 2).zero?
      submatrices = split(pattern, 2)
    elsif (size % 3).zero?
      submatrices = split(pattern, 3)
    else
      raise "Pattern not divisble by 2 or 3"
    end

    submatrices.map! {|submatrix| evaluate(submatrix, rules) }

    if submatrices.size == 1
      pattern = submatrices.first
    else
      new_size = Math.sqrt(submatrices.join.delete("/").size).to_i
      groups = new_size / submatrices.first.split("/").first.size
      puts "new_size=#{new_size} submatrices.size=#{submatrices.size} groups=#{groups}"
      pattern = submatrices.in_groups_of(groups).map do |gs|
        x = unpack(gs.shift)
        gs.each {|g| x = x.zip(unpack(g)) }
        x.map(&:join).join("/")
      end.join("/")
    end

#    puts pattern
  end

  puts "\n"
  puts "sum: #{pattern.count("#")}"
end

def split(pattern, size)
  matrix = unpack(pattern)
  matrix.each_with_object(Hash.new {|h, k| h[k] = [] }).with_index do |(row, hash), row_index|
    row.map.with_index do |cell, col_index|
      hash[[row_index / size, col_index / size]] << cell
    end
  end.map do |id, cells|
    pack(cells.in_groups_of(size))
  end
end

def unpack(pattern)
  pattern.split("/").map {|row| row.split(//) }
end

def pack(matrix)
  matrix.map(&:join).join("/")
end

def evaluate(pattern, rules)
  hashes = pattern.count("#")

  rules.detect do |from, to|
    next unless from.size == pattern.size
    next unless from.count("#") == hashes

    return to if from == pattern ||
      from == rotate(pattern) ||
      from == rotate(rotate(pattern)) ||
      from == rotate(rotate(rotate(pattern)))

    pattern = flip_vertically(pattern)

    return to if from == pattern ||
      from == rotate(pattern) ||
      from == rotate(rotate(pattern)) ||
      from == rotate(rotate(rotate(pattern)))
  end

  raise("No matching rule found")
end

def flip_horizontally(pattern)
  pattern.split("/").map(&:reverse).join("/")
end

def flip_vertically(pattern)
  pattern.split("/").reverse.join("/")
end

def rotate(pattern)
  matrix = unpack(pattern)

  if matrix.first.size == 2
    row1, row2 = matrix
    rotated_matrix = [
      [ row2[0], row1[0] ],
      [ row2[1], row1[1] ]
    ]
  elsif matrix.first.size == 3
    row1, row2, row3 = matrix
    rotated_matrix = [
      [ row3[0], row2[0], row1[0] ],
      [ row3[1], row2[1], row1[1] ],
      [ row3[2], row2[2], row1[2] ]
    ]
  else
    raise "unsupported matrix size for rotation"
  end

  pack(rotated_matrix)
end

def initial_pattern
  ".#./..#/###"
end

def parse(input)
  input.split(/\n/).each_with_object({}) do |line, book|
    from, to = line.split(" => ")
    book[from] = to
  end
end

puzzle_rules = File.read("../inputs/21").chomp

part_one(puzzle_rules)
