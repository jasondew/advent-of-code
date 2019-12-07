defmodule DayFive do
  @doc """
    Part 1
  """
  def part_one(input) do
    input
    |> Conversions.to_integers(",")
    |> IntcodeVM.run_for_output([1])
  end

  @doc """
    Part 2
  """
  def part_two(input) do
    input
    |> Conversions.to_integers(",")
    |> IntcodeVM.run_for_output([5])
  end
end
