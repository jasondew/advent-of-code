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

  def day_three() do
    input = read_input("03")

    IO.puts("Part 1: #{DayThree.part_one(input)}")
    IO.puts("Part 2: #{DayThree.part_two(input)}")

    Benchee.run(
      %{
        part_one: fn -> DayThree.part_one(input) end,
        part_two: fn -> DayThree.part_two(input) end
      },
      formatters: [
        {Benchee.Formatters.Console, comparison: false}
      ]
    )

    :ok
  end

  def day_four() do
    input = read_input("04")

    IO.puts("Part 1: #{DayFour.part_one(input)}")
    IO.puts("Part 2: #{DayFour.part_two(input)}")

    Benchee.run(
      %{
        part_one: fn -> DayFour.part_one(input) end,
        part_two: fn -> DayFour.part_two(input) end
      },
      formatters: [
        {Benchee.Formatters.Console, comparison: false}
      ]
    )

    :ok
  end

  def day_five() do
    input = read_input("05")

    IO.puts("Part 1: #{DayFive.part_one(input)}")
    IO.puts("Part 2: #{DayFive.part_two(input)}")

    Benchee.run(
      %{
        part_one: fn -> DayFive.part_one(input) end,
        part_two: fn -> DayFive.part_two(input) end
      },
      formatters: [
        {Benchee.Formatters.Console, comparison: false}
      ]
    )

    :ok
  end

  def day_six() do
    input = read_input("06")

    IO.puts("Part 1: #{DaySix.part_one(input)}")
    IO.puts("Part 2: #{DaySix.part_two(input)}")

    Benchee.run(
      %{
        part_one: fn -> DaySix.part_one(input) end,
        part_two: fn -> DaySix.part_two(input) end
      },
      formatters: [
        {Benchee.Formatters.Console, comparison: false}
      ]
    )

    :ok
  end

  def day_seven() do
    input = read_input("07")

    IO.puts("Part 1: #{DaySeven.part_one(input)}")
    IO.puts("Part 2: #{DaySeven.part_two(input)}")

    Benchee.run(
      %{
        part_one: fn -> DaySeven.part_one(input) end,
        part_two: fn -> DaySeven.part_two(input) end
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
