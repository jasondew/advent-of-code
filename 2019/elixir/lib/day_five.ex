defmodule DayFive do
  @doc """
    Part 1
  """
  def part_one(input) do
    tape = Conversions.to_integers(input, ",")
    outputs = run(tape, 0, [1])
  end

  def run(tape, ip, inputs, outputs \\ []) do
    [operation, arg1_position, arg2_position, result_position] =
      Enum.slice(tape, ip, 4)

    arg1 = Enum.at(tape, arg1_position)
    arg2 = Enum.at(tape, arg2_position)

    case operation do
      1 ->
        tape
        |> List.replace_at(result_position, arg1 + arg2)
        |> step(ip + 4, inputs, outputs)

      2 ->
        tape
        |> List.replace_at(result_position, arg1 * arg2)
        |> step(ip + 4, inputs, outputs)

      3 ->
        tape

      99 ->
        tape
    end
  end

  def parse_instruction(tape, position) do
    instruction = Enum.at(tape, position)
    parameter_modes, opcode = Enum.split_at(instruction, -2)

    %{
      opcode: String.to_integer(opcode),
    }
  end
end
