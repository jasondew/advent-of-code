defmodule ElixirSolutions do
  def day_one do
    input = day_input("01")
    IO.puts("Part 1: #{ElixirSolutions.DayOne.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayOne.part_two(input)}")
  end

  def day_two do
    input = day_input("02")
    IO.puts("Part 1: #{ElixirSolutions.DayTwo.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayTwo.part_two(input)}")
  end

  def day_three do
    input = day_input("03")
    IO.puts("Part 1: #{ElixirSolutions.DayThree.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayThree.part_two(input)}")
  end

  def day_four do
    input = day_input("04")
    IO.puts("Part 1: #{ElixirSolutions.DayFour.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayFour.part_two(input)}")
  end

  def day_five do
    input = day_input("05")
    IO.puts("Part 1: #{ElixirSolutions.DayFive.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayFive.part_two(input)}")
  end

  def day_six do
    input = day_input("06")
    IO.puts("Part 1: #{ElixirSolutions.DaySix.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DaySix.part_two(input)}")
  end

  def day_seven do
    input = day_input("07")
    IO.puts("Part 1: #{ElixirSolutions.DaySeven.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DaySeven.part_two(input)}")
  end

  def day_eight do
    input = day_input("08")
    IO.puts("Part 1: #{ElixirSolutions.DayEight.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayEight.part_two(input)}")
  end

  def day_nine do
    input = day_input("09")
    IO.puts("Part 1: #{ElixirSolutions.DayNine.part_one(input)}")
    IO.puts("Part 2: #{ElixirSolutions.DayNine.part_two(input)}")
  end

  defp day_input(day) do
    {:ok, input} = File.read("../inputs/#{day}")
    String.trim(input)
  end
end
