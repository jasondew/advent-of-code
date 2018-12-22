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

  def part_two(_input), do: ""

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
