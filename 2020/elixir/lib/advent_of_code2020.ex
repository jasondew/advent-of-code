defmodule AdventOfCode2020 do
  @moduledoc """
  Documentation for AdventOfCode2020.
  """

  def day_one() do
    input = read_input("01")

    IO.puts("Part 1: #{DayOne.part_one(input)}")
    IO.puts("Part 2: #{DayOne.part_two(input)}")
  end

  def day_two() do
    input = read_input("02")

    IO.puts("Part 1: #{DayTwo.part_one(input)}")
    IO.puts("Part 2: #{DayTwo.part_two(input)}")

    Benchee.run(
      %{
        part_one: fn -> DayTwo.part_one(input) end,
        part_two: fn -> DayTwo.part_two(input) end
      },
      formatters: [
        {Benchee.Formatters.Console, comparison: false}
      ]
    )

    :ok
  end

  ## PRIVATE FUNCTIONS

  def read_input(day) do
    "../inputs/#{day}"
    |> File.read!()
    |> String.trim()
  end
end
