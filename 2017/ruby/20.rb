require "ap"

Vector = Struct.new(:x, :y, :z) do
  def to_s
    "(%2d,%2d,%2d)" % [x, y, z]
  end
end

Particle = Struct.new(:index, :position, :velocity, :acceleration) do
  attr_accessor :destroyed

  def to_s
    "[#{index}] p=#{position} v=#{velocity} a=#{acceleration}"
  end

  def distance
    position[:x].abs + position[:y].abs + position[:z].abs
  end

  def destroy
    self.destroyed = true
  end

  def step
    velocity[:x] += acceleration[:x]
    velocity[:y] += acceleration[:y]
    velocity[:z] += acceleration[:z]
    position[:x] += velocity[:x]
    position[:y] += velocity[:y]
    position[:z] += velocity[:z]
  end
end

def part_one(input)
  particles = parse(input)

  best_index, best_distance = nil, nil

  loop do
    1000.times { particles.map(&:step) }

    min_particle = particles.min_by(&:distance)
    min_distance = min_particle.distance

    if best_distance.nil? || min_distance < best_distance
      best_distance = min_distance
      best_index = min_particle.index
    else
      break
    end
  end

  puts best_index
end

def part_two(input)
  particles = parse(input)

  1000.times do
    particles.map(&:step)

    particles.group_by(&:position).each do |position, particles|
      particles.each(&:destroy) if particles.size > 1
    end
  end

  p particles.reject(&:destroyed).size
end

def parse(input)
  input.split(/\n/).map.with_index do |line, index|
    vectors = line.scan(/=< *([\-0-9]+), *([\-0-9]+), *([\-0-9]+)>/).map do |triplet|
      Vector.new(*triplet.map(&:to_i))
    end
    Particle.new(index, *vectors)
  end
end

puzzle_input = File.read("../inputs/20").chomp

#part_one(puzzle_input)
part_two(puzzle_input)
