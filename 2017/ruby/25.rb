require "ap"

class Machine
  TRANSITIONS = {
    a: [[1, :right, :b], [0, :left,  :c]],
    b: [[1, :left,  :a], [1, :left,  :d]],
    c: [[1, :right, :d], [0, :right, :c]],
    d: [[0, :left,  :b], [0, :right, :e]],
    e: [[1, :right, :c], [1, :left,  :f]],
    f: [[1, :left,  :e], [1, :right, :a]]
  }

  attr_accessor :tape, :position, :state

  def initialize
    @tape = Hash.new {|h, k| h[k] = 0 }
    @position = 0
    @state = :a
  end

  def step
    value_to_write, move_direction, new_state = TRANSITIONS[state][read]

    write(value_to_write)
    move(move_direction)
    self.state = new_state
  end

  def read
    tape[position]
  end

  def write(value)
    tape[position] = value
  end

  def move(direction)
    if direction == :left
      self.position -= 1
    else
      self.position += 1
    end
  end
end

machine = Machine.new

12_656_374.times do
  machine.step
end

puts machine.tape.values.sum
