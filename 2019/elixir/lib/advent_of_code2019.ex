defmodule AdventOfCode2019 do
  @moduledoc """
  Documentation for Advent of Code 2019.
  """

  def day_one do
    input = day_input("01")
    IO.puts("Day 1, part 1 answer: #{DayOne.part_one(input)}")
    IO.puts("Day 1, part 2 answer: #{DayOne.part_two(input)}")
  end

  def day_two do
    input = day_input("02")
    IO.puts("Day 2, part 1 answer: #{DayTwo.part_one(input)}")
    IO.puts("Day 2, part 2 answer: #{DayTwo.part_two(input)}")
  end

  def day_three do
    input = day_input("03")
    IO.puts("Day 3, part 1 answer: #{DayThree.part_one(input)}")
    IO.puts("Day 3, part 2 answer: #{DayThree.part_two(input)}")
  end

  def day_four do
    input = day_input("04")
    IO.puts("Day 4, part 1 answer: #{DayFour.part_one(input)}")
    IO.puts("Day 4, part 2 answer: #{DayFour.part_two(input)}")
  end

  def day_five do
    input = day_input("05")
    IO.puts("Day 5, part 1 answer: #{DayFive.part_one(input)}")
    IO.puts("Day 5, part 2 answer: #{DayFive.part_two(input)}")
  end

  def day_six do
    input = day_input("06")
    IO.puts("Day 6, part 1 answer: #{DaySix.part_one(input)}")
    IO.puts("Day 6, part 2 answer: #{DaySix.part_two(input)}")
  end

  def day_seven do
    input = day_input("07")
    IO.puts("Day 7, part 1 answer: #{DaySeven.part_one(input)}")
    IO.puts("Day 7, part 2 answer: #{DaySeven.part_two(input)}")
  end

  def day_eight do
    input = day_input("08")
    IO.puts("Day 8, part 1 answer: #{DayEight.part_one(input, 25, 6)}")
    IO.puts("Day 8, part 2 answer:")

    input
    |> DayEight.part_two(25, 6)
    |> Enum.each(fn row ->
      row
      |> Enum.map(fn
        0 -> ".."
        1 -> "##"
      end)
      |> IO.puts()
    end)
  end

  def day_nine do
    input = day_input("09")
    IO.puts("Day 9, part 1 answer: #{DayNine.part_one(input)}")
    IO.puts("Day 9, part 2 answer: #{DayNine.part_two(input)}")
  end

  def day_ten do
    input = day_input("10")
    IO.puts("Day 10, part 1 answer: #{DayTen.part_one(input)}")
    IO.puts("Day 10, part 2 answer: #{DayTen.part_two(input)}")
  end

  def day_eleven do
    input = day_input("11")
    IO.puts("Day 11, part 1 answer: #{DayEleven.part_one(input)}")
    IO.puts("Day 11, part 2 answer:")

    input
    |> DayEleven.part_two()
    |> Enum.each(fn row ->
      row
      |> Enum.map(fn
        "." -> ".."
        "#" -> "##"
      end)
      |> IO.puts()
    end)
  end

  def day_twelve do
    input = day_input("12")
    IO.puts("Day 12, part 1 answer: #{DayTwelve.part_one(input)}")
    IO.puts("Day 12, part 2 answer: #{DayTwelve.part_two(input)}")
  end

  ## PRIVATE FUNCTIONS ##

  defp day_input(day, trim \\ true) do
    {:ok, input} = File.read("../inputs/#{day}")
    if trim, do: String.trim(input), else: input
  end
end
