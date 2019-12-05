defmodule DayFive do
  @doc """
    Part 1
  """
  def part_one(input) do
    tape = Conversions.to_integers(input, ",")
    outputs = run(tape, 0, [1])
  end

  def run(tape, ip, inputs, outputs \\ []) do
    [operation, arg1_position, arg2_position, result_position] = Enum.slice(tape, ip, 4)

    arg1 = Enum.at(tape, arg1_position)
    arg2 = Enum.at(tape, arg2_position)

    case operation do
      1 ->
        tape
        |> List.replace_at(result_position, arg1 + arg2)
        |> run(ip + 4, inputs, outputs)

      2 ->
        tape
        |> List.replace_at(result_position, arg1 * arg2)
        |> run(ip + 4, inputs, outputs)

      3 ->
        tape

      99 ->
        tape
    end
  end

  defp parse_instruction(tape, position) do
    instruction = Enum.at(tape, position)
    [parameter_modes_string, opcode_string] = String.split_at(instruction, -2)
    opcode = String.to_integer(opcode_string)
    parameter_count = parameter_count(opcode)
    parameter_modes = List.duplicate(0, parameter_count)

    parameter_modes_string
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.each(fn {mode_string, index} ->
      List.replace_at(parameter_modes, index, String.to_integer(mode_string))
    end)

    %{
      opcode: opcode,
      parameter_count: parameter_count,
      parameter_modes: parameter_modes
    }
  end

  defp parameter_count(1), do: 3
  defp parameter_count(2), do: 3
  defp parameter_count(3), do: 1
  defp parameter_count(4), do: 1
  defp parameter_count(99), do: 0
  defp parameter_count(opcode), do: raise("invalid opcode: #{opcode}")
end
