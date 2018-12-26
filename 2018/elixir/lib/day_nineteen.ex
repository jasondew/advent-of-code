defmodule ElixirSolutions.DayNineteen do
  require ElixirSolutions.SantaWatch, as: SantaWatch

  @doc ~S"""
    iex> part_one("#ip 0
    ...>seti 5 0 1
    ...>seti 6 0 2
    ...>addi 0 1 0
    ...>addr 1 2 3
    ...>setr 1 0 0
    ...>seti 8 0 4
    ...>seti 9 0 5")
    6
  """
  def part_one(input) do
    {ip_register, instructions} = parse(input)
    instruction_count = Enum.count(instructions)
    initial_registers = [0, 0, 0, 0, 0, 0]

    Enum.reduce_while(1..10_000_000, initial_registers, fn _counter, registers ->
      ip = SantaWatch.get(registers, ip_register)
      instruction = Enum.at(instructions, ip)

      SantaWatch.set(registers, ip_register, ip)
      updated_registers = SantaWatch.execute(registers, instruction)
      updated_ip = Enum.at(updated_registers, ip_register) + 1

      #      IO.puts(
      #        "ip=#{Integer.to_string(ip)} #{inspect(registers)} #{inspect(instruction)} #{
      #          inspect(updated_registers)
      #        }"
      #      )

      if updated_ip >= 0 and updated_ip < instruction_count do
        updated_registers = SantaWatch.set(updated_registers, ip_register, updated_ip)

        {:cont, updated_registers}
      else
        {:halt, SantaWatch.get(registers, 0)}
      end
    end)
  end

  def part_two(input) do
    {ip_register, instructions} = parse(input)
    instruction_count = Enum.count(instructions)
    initial_registers = [1, 0, 0, 0, 0, 0]
    max_iterations = 100_000_000_000

    Enum.reduce_while(1..max_iterations, {initial_registers, %{}}, fn _counter,
                                                                      {registers, trace} ->
      ip = SantaWatch.get(registers, ip_register)
      instruction = Enum.at(instructions, ip)

      {updated_ip, updated_registers} =
        if ip == 3 do
          #  3 mulr 5 2 3 => r[c] = r[a] * r[b]            =>  3: r3 = r5 * r2  => if ((r3 = r5*r2) == r4) => r0 = r5 * (r4 - r2)
          #  4 eqrr 3 4 3 => r[c] = if(r[a] == r[b], 1, 0) =>  4: r3 = r3 == r4 =>   r0 += r5              =>
          #  5 addr 3 1 1 => r[c] = r[a] + r[b]            =>  5: ip += r3      => r2 += 1                 =>
          #  6 addi 1 1 1 => r[c] = r[a] + b               =>  6: ip += 1       => loop until r2 > r4      =>
          #  7 addr 5 0 0 => r[c] = r[a] + r[b]            =>  7: r0 += r5      =>
          #  8 addi 2 1 2 => r[c] = r[a] + b               =>  8: r2 += 1       =>
          #  9 gtrr 2 4 3 => r[c] = if(r[a] > r[b], 1, 0)  =>  9: r3 = r2 > r4  =>
          # 10 addr 1 3 1 => r[c] = r[a] + r[b]            => 10: ip += r3      =>
          # 11 seti 2 8 1 => r[c] = a                      => 11: ip = 2        =>
          r2 = SantaWatch.get(registers, 2)
          r4 = SantaWatch.get(registers, 4)
          r5 = SantaWatch.get(registers, 5)
          {3, SantaWatch.set(registers, 0, r5 * (r4 - r2))}
        else
          updated_registers = SantaWatch.execute(registers, instruction)
          {Enum.at(updated_registers, ip_register) + 1, updated_registers}
        end

      #      IO.puts(
      #        "ip=#{Integer.to_string(ip)} #{inspect(registers)} #{inspect(instruction)} #{
      #          inspect(updated_registers)
      #        }"
      #      )

      if updated_ip >= 0 and updated_ip < instruction_count do
        updated_registers = SantaWatch.set(updated_registers, ip_register, updated_ip)
        # updated_trace = Map.update(trace, ip, [updated_registers], &[updated_registers | &1])
        updated_trace = Map.update(trace, ip, 1, &(&1 + 1))

        {:cont, {updated_registers, updated_trace}}
      else
        {:halt, {:done, registers, trace}}
      end
    end)
  end

  def parse(input) do
    [ip_register_line | instruction_lines] = String.split(input, "\n")

    {
      parse_ip_register(ip_register_line),
      Enum.map(instruction_lines, &parse_instruction/1)
    }
  end

  defp parse_ip_register("#ip " <> register), do: String.to_integer(register)

  defp parse_instruction(string) do
    [name | args] = string |> String.split(" ")
    [a, b, c] = args |> Enum.map(&String.to_integer/1)
    {name, a, b, c}
  end
end
