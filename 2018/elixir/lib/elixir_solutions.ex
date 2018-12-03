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
  end

  defp day_input(day) do
    {:ok, input} = File.read("../inputs/#{day}")
    String.trim(input)
  end
end
