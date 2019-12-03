defmodule AdventOfCode2019 do
  @moduledoc """
  Documentation for Advent of Code 2019.
  """

  def day_one_answers do
    input = day_input("01") |> to_integers()
    IO.puts("Day 1, part 1 answer: #{DayOne.part_one(input)}")
    IO.puts("Day 1, part 2 answer: #{DayOne.part_two(input)}")
  end

  def day_two_answers do
    input = day_input("02") |> to_integers(",")
    IO.puts("Day 2, part 1 answer: #{DayTwo.part_one(input)}")
    IO.puts("Day 2, part 2 answer: #{DayTwo.part_two(input)}")
  end

  ## PRIVATE FUNCTIONS ##

  defp day_input(day, trim \\ true) do
    {:ok, input} = File.read("../inputs/#{day}")
    if trim, do: String.trim(input), else: input
  end

  defp to_integers(input, delimiter \\ "\n") do
    input
    |> String.split(delimiter)
    |> Enum.map(&String.to_integer/1)
  end
end