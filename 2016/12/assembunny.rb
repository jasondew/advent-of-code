@registers = {"a" => 0, "b" => 0, "c" => 1, "d" => 0}
@instructions = DATA.each_line.map {|line| line.chomp.split(/ /) }
@ip = 0

def cpy(value, to)
  @registers[to] = @registers[value] || value.to_i
  @ip += 1
end

def inc(register)
  @registers[register] += 1
  @ip += 1
end

def dec(register)
  if @registers[register] > 0
    @registers[register] -= 1
  end
  @ip += 1
end

def jnz(register, offset)
  if @registers[register] != 0
    @ip += offset.to_i
  else
    @ip += 1
  end
end

iteration = 0
while (instruction = @instructions[@ip]) do
  command, *args = instruction
  send command, *args
#  puts "IP: #{@ip}, #{command}#{args.inspect} => #{@registers.inspect}"
#  break if iteration >= 25
  iteration+=1
end
puts @registers.inspect

__END__
cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 16 c
cpy 17 d
inc a
dec d
jnz d -2
dec c
jnz c -5
