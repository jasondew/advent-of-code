require Logger

defmodule ElixirSolutions.DayOne do
  @doc ~S"""
    Part 1

    +0 +1 produces a sum of 1

    iex> ElixirSolutions.DayOne.part_one("+0\n+1")
    1

    +1 -1 produces a sum of 0

    iex> ElixirSolutions.DayOne.part_one("+1\n-1")
    0
  """

  def part_one(input) do
    input
    |> String.split()
    |> List.foldl(0, fn x, total -> run(x, total) end)
  end

  @doc ~S"""
    Part 2

    +1, -1 first reaches 0 twice.
    iex> ElixirSolutions.DayOne.part_two("+1 -1")
    0

    +3, +3, +4, -2, -4 first reaches 10 twice.
    iex> ElixirSolutions.DayOne.part_two("+3 +3 +4 -2 -4")
    10

    -6, +3, +8, +5, -6 first reaches 5 twice.
    iex> ElixirSolutions.DayOne.part_two("-6 +3 +8 +5 -6")
    5

    +7, +7, -2, -7, -4 first reaches 14 twice.
    iex> ElixirSolutions.DayOne.part_two("+7 +7 -2 -7 -4")
    14
  """

  def part_two(input) do
    instruction_stream =
      input
      |> String.split()
      |> Stream.cycle()

    instruction_stream
    |> Stream.scan(0, &run/2)
    |> Stream.transform(MapSet.new([0]), fn total, set ->
      if MapSet.member?(set, total) do
        {[total], set}
      else
        {[], MapSet.put(set, total)}
      end
    end)
    |> Enum.take(1)
    |> List.first()
  end

  defp run("+" <> number_string, total) do
    total + String.to_integer(number_string)
  end

  defp run("-" <> number_string, total) do
    total - String.to_integer(number_string)
  end
end
