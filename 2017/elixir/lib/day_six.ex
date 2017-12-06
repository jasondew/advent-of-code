defmodule DaySix do
  @doc ~S"""
  Part 1 solver

  Given 4 memory banks and an initial configuration of 0, 2, 7, 0, a loop
  is detected in 5 steps.

    iex> DaySix.partOne("0 2 7 0")
    5
  """
  def partOne(input) do
    blocks =
      input
      |> String.split()
      |> Enum.map(&String.to_integer/1)

    run_until_loop_detected(blocks, 0, [])
  end

  defp run_until_loop_detected(blocks, cycles, previous_states) do
  end
end
