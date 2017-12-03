defmodule ElixirSolutions do
  @moduledoc """
  Advent of Code 2017 Elixir Solutions
  """

  def dayOne do
    input = dayInput("01")
    IO.puts("Part 1: #{DayOne.partOne(input)}")
    IO.puts("Part 1: #{DayOne.partTwo(input)}")
  end

  defp dayInput(day) do
    {:ok, input} = File.read("../#{day}/input")
    String.trim(input)
  end
end
