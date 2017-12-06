defmodule ElixirSolutions do
  @moduledoc """
  Advent of Code 2017 Elixir Solutions
  """

  def dayOne do
    input = dayInput("01")
    IO.puts("Part 1: #{DayOne.partOne(input)}")
    IO.puts("Part 2: #{DayOne.partTwo(input)}")
  end

  def daySix do
    input = dayInput("06")
    IO.puts("Part 1: #{DaySix.partOne(input)}")
  end

  defp dayInput(day) do
    {:ok, input} = File.read("../inputs/#{day}")
    String.trim(input)
  end

end
