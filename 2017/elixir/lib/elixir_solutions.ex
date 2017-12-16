defmodule ElixirSolutions do
  @moduledoc """
  Advent of Code 2017 Elixir Solutions
  """

  def day_one do
    input = day_input("01")
    IO.puts("Part 1: #{DayOne.part_one(input)}")
    IO.puts("Part 2: #{DayOne.part_two(input)}")
  end

  def day_six do
    input = day_input("06")
    IO.puts("Part 1: #{DaySix.part_one(input)}")
  end

  def day_eight do
    input = day_input("08")
    IO.puts("Part 1: #{DayEight.part_one(input)}")
  end

  def day_thirteen do
    input = day_input("13")
    IO.puts("Part 1: #{DayThirteen.part_one(input)}")
    IO.puts("Part 2: #{DayThirteen.part_two(input)}")
  end

  def day_sixteen do
    input = day_input("16")
    IO.puts("Part 2: #{DaySixteen.part_two("abcdefghijklmnop", input)}")
  end

  defp day_input(day) do
    {:ok, input} = File.read("../inputs/#{day}")
    String.trim(input)
  end
end
