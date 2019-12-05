defmodule DayFive do
  @doc """
    Part 1
  """
  def part_one(input) do
    input
    |> Conversions.to_integers(",")
    |> run(0, [1])
    |> List.first()
  end

  @doc """
    Part 2
  """
  def part_two(input) do
    input
    |> Conversions.to_integers(",")
    |> run(0, [5])
    |> List.first()
  end

  def run(tape, ip, inputs, outputs \\ []) do
    instruction = parse_instruction(tape, ip)

    case instruction.operation do
      :add ->
        a = get_value(tape, param(instruction, 0))
        b = get_value(tape, param(instruction, 1))

        tape
        |> set_value(a + b, param(instruction, 2))
        |> run(ip + instruction.size, inputs, outputs)

      :mul ->
        a = get_value(tape, param(instruction, 0))
        b = get_value(tape, param(instruction, 1))

        tape
        |> set_value(a * b, param(instruction, 2))
        |> run(ip + instruction.size, inputs, outputs)

      :in ->
        {input, inputs} = List.pop_at(inputs, 0)

        tape
        |> set_value(input, param(instruction, 0))
        |> run(ip + instruction.size, inputs, outputs)

      :out ->
        output = get_value(tape, param(instruction, 0))
        run(tape, ip + instruction.size, inputs, [output | outputs])

      :jit ->
        if get_value(tape, param(instruction, 0)) != 0 do
          new_ip = get_value(tape, param(instruction, 1))
          run(tape, new_ip, inputs, outputs)
        else
          run(tape, ip + instruction.size, inputs, outputs)
        end

      :jif ->
        if get_value(tape, param(instruction, 0)) == 0 do
          new_ip = get_value(tape, param(instruction, 1))
          run(tape, new_ip, inputs, outputs)
        else
          run(tape, ip + instruction.size, inputs, outputs)
        end

      :lt ->
        if get_value(tape, param(instruction, 0)) <
             get_value(tape, param(instruction, 1)) do
          set_value(tape, 1, param(instruction, 2))
        else
          set_value(tape, 0, param(instruction, 2))
        end
        |> run(ip + instruction.size, inputs, outputs)

      :eq ->
        if get_value(tape, param(instruction, 0)) ==
             get_value(tape, param(instruction, 1)) do
          set_value(tape, 1, param(instruction, 2))
        else
          set_value(tape, 0, param(instruction, 2))
        end
        |> run(ip + instruction.size, inputs, outputs)

      :exit ->
        outputs
    end
  end

  defp param(instruction, index) do
    Enum.at(instruction.params, index)
  end

  defp get_value(tape, {:position, position}), do: Enum.at(tape, position)
  defp get_value(_tape, {:immediate, value}), do: value

  defp set_value(tape, value, {:position, position}) do
    List.replace_at(tape, position, value)
  end

  defp parse_instruction(tape, position) do
    instruction = Enum.at(tape, position)

    {param_modes_string, opcode_string} =
      instruction
      |> Integer.to_string()
      |> String.split_at(-2)

    opcode = String.to_integer(opcode_string)
    {operation, param_count} = translate_opcode(opcode)

    param_modes =
      param_modes_string
      |> String.reverse()
      |> String.graphemes()
      |> Enum.map(&String.to_integer/1)

    params =
      tape
      |> Enum.slice(position + 1, param_count)
      |> Enum.with_index()
      |> Enum.map(fn {param, index} ->
        case Enum.at(param_modes, index, 0) do
          0 -> {:position, param}
          1 -> {:immediate, param}
        end
      end)

    %{
      operation: operation,
      params: params,
      size: param_count + 1
    }
  end

  defp translate_opcode(1), do: {:add, 3}
  defp translate_opcode(2), do: {:mul, 3}
  defp translate_opcode(3), do: {:in, 1}
  defp translate_opcode(4), do: {:out, 1}
  defp translate_opcode(5), do: {:jit, 2}
  defp translate_opcode(6), do: {:jif, 2}
  defp translate_opcode(7), do: {:lt, 3}
  defp translate_opcode(8), do: {:eq, 3}
  defp translate_opcode(99), do: {:exit, 0}
  defp translate_opcode(opcode), do: raise("invalid opcode: #{opcode}")
end
