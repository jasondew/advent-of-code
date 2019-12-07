defmodule AdventOfCode2019 do
  @moduledoc """
  Documentation for Advent of Code 2019.
  """

  def day_one_answers do
    input = day_input("01")
    IO.puts("Day 1, part 1 answer: #{DayOne.part_one(input)}")
    IO.puts("Day 1, part 2 answer: #{DayOne.part_two(input)}")
  end

  def day_two_answers do
    input = day_input("02")
    IO.puts("Day 2, part 1 answer: #{DayTwo.part_one(input)}")
    IO.puts("Day 2, part 2 answer: #{DayTwo.part_two(input)}")
  end

  def day_three_answers do
    input = day_input("03")
    IO.puts("Day 3, part 1 answer: #{DayThree.part_one(input)}")
    IO.puts("Day 3, part 2 answer: #{DayThree.part_two(input)}")
  end

  def day_four_answers do
    input = day_input("04")
    IO.puts("Day 4, part 1 answer: #{DayFour.part_one(input)}")
    IO.puts("Day 4, part 2 answer: #{DayFour.part_two(input)}")
  end

  def day_five_answers do
    input = day_input("05")
    IO.puts("Day 5, part 1 answer: #{DayFive.part_one(input)}")
    IO.puts("Day 5, part 2 answer: #{DayFive.part_two(input)}")
  end

  def day_six_answers do
    input = day_input("06")
    IO.puts("Day 6, part 1 answer: #{DaySix.part_one(input)}")
    IO.puts("Day 6, part 2 answer: #{DaySix.part_two(input)}")
  end

  def day_seven_answers do
    input = day_input("07")
    IO.puts("Day 7, part 1 answer: #{DaySeven.part_one(input)}")
    IO.puts("Day 7, part 2 answer: #{DaySeven.part_two(input)}")
  end

  ## PRIVATE FUNCTIONS ##

  defp day_input(day, trim \\ true) do
    {:ok, input} = File.read("../inputs/#{day}")
    if trim, do: String.trim(input), else: input
  end
end
