defmodule DaySeven do
  @doc """
    Part 1

    iex> DaySeven.part_one("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
    43210

    iex> DaySeven.part_one("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
    54321

    iex> DaySeven.part_one("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")
    65210
  """
  def part_one(input) do
    intcode = Conversions.to_integers(input, ",")

    0..4
    |> Enum.into([])
    |> permutations()
    |> Enum.map(&run_with_phases(intcode, &1))
    |> Enum.max()
  end

  @doc """
    Part 2

    iex> DaySeven.part_two("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")
    139629729

    iex> DaySeven.part_two("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")
    18216
  """
  def part_two(input) do
    intcode = Conversions.to_integers(input, ",")

    5..9
    |> Enum.into([])
    |> permutations()
    |> Enum.map(&run_with_phases_and_feedback(intcode, &1))
    |> Enum.max()
  end

  ## PRIVATE FUNCTIONS ##

  defp run_with_phases(intcode, phases) do
    Enum.reduce(phases, 0, fn phase, input ->
      IntcodeVM.run_for_output(intcode, [phase, input])
    end)
  end

  defp run_with_phases_and_feedback(intcode, phases) do
    phases
    |> Enum.map(fn phase -> IntcodeVM.run(intcode, [phase]) end)
    |> run_loop(0)
  end

  defp run_loop(amps, input) do
    {output, processed_amps} =
      Enum.reduce(amps, {input, []}, fn amp, {input, processed_amps} ->
        %{outputs: [output]} =
          processed_amp = IntcodeVM.run(amp.intcode, [input], amp.ip)

        {output, [processed_amp | processed_amps]}
      end)

    case List.last(processed_amps) do
      %{state: :waiting_for_input} ->
        run_loop(Enum.reverse(processed_amps), output)

      %{state: :done} ->
        output
    end
  end

  defp permutations([value | []]), do: [[value]]

  defp permutations(values) do
    Enum.flat_map(values, fn value ->
      values
      |> List.delete(value)
      |> permutations()
      |> Enum.map(&[value | &1])
    end)
  end
end
