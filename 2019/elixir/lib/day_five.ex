defmodule DayFive do
  @doc """
    Part 1
  """
  def part_one(input) do
    input
    |> Conversions.to_integers(",")
    |> run(0, [1])
    |> IO.inspect()
  end

  @doc """
    iex> DayFive.run([1,9,10,3,2,3,11,0,99,30,40,50], 0, [], [])
    {[3500], []}
  """
  def run(tape, ip, inputs, outputs \\ []) do
    instruction = parse_instruction(tape, ip) |> IO.inspect()

    case instruction.opcode do
      1 ->
        a = get_value(tape, Enum.at(instruction.parameters, 0))
        b = get_value(tape, Enum.at(instruction.parameters, 1))
        result_position = Enum.at(instruction.parameters, 2)

        tape
        |> set_value(a + b, result_position)
        |> run(ip + 4, inputs, outputs)

      2 ->
        a = get_value(tape, Enum.at(instruction.parameters, 0))
        b = get_value(tape, Enum.at(instruction.parameters, 1))
        result_position = Enum.at(instruction.parameters, 2)

        tape
        |> set_value(a - b, result_position)
        |> run(ip + 4, inputs, outputs)

      3 ->
        {:position, position} = Enum.at(instruction.parameters, 0)
        {input, inputs} = List.pop_at(inputs, 0)

        tape
        |> List.replace_at(position, input)
        |> run(ip + 2, inputs, outputs)

      4 ->
        output = get_value(tape, Enum.at(instruction.parameters, 0))
        run(tape, ip + 2, inputs, [output | outputs])

      99 ->
        {tape, outputs}
    end
  end

  defp get_value(tape, {:position, position}), do: Enum.at(tape, position)
  defp get_value(_tape, {:immediate, value}), do: value

  defp set_value(tape, value, {:position, position}) do
    List.replace_at(tape, position, value)
  end

  defp parse_instruction(tape, position) do
    instruction = Enum.at(tape, position)

    {parameter_modes_string, opcode_string} =
      instruction
      |> Integer.to_string()
      |> String.split_at(-2)

    opcode = String.to_integer(opcode_string)

    parameter_modes =
      parameter_modes_string
      |> String.reverse()
      |> String.graphemes()
      |> Enum.map(&String.to_integer/1)

    parameters =
      tape
      |> Enum.slice(position + 1, parameter_count(opcode))
      |> Enum.with_index()
      |> Enum.map(fn {parameter, index} ->
        case Enum.at(parameter_modes, index, 0) do
          0 -> {:position, parameter}
          1 -> {:immediate, parameter}
        end
      end)

    %{
      opcode: opcode,
      parameters: parameters
    }
  end

  defp parameter_count(1), do: 3
  defp parameter_count(2), do: 3
  defp parameter_count(3), do: 1
  defp parameter_count(4), do: 1
  defp parameter_count(99), do: 0
  defp parameter_count(opcode), do: raise("invalid opcode: #{opcode}")
end
