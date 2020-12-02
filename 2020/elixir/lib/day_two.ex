defmodule DayTwo do
  @doc """
    Part 1

    iex> DayTwo.part_one("1-3 a: abcde\\n1-3 b: cdefg\\n2-9 c: ccccccccc\\n")
    2
  """

  def part_one(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.count(fn line ->
      [policy, password] = String.split(line, ": ")
      [range, letter] = String.split(policy, " ")
      [min, max] = String.split(range, "-") |> Enum.map(&String.to_integer/1)

      count = String.split(password, "") |> Enum.count(&(&1 == letter))
      count >= min and count <= max
    end)
  end

  @doc """
    Part 2

    iex> DayTwo.part_two("1-3 a: abcde\\n1-3 b: cdefg\\n2-9 c: ccccccccc\\n")
    1
  """

  def part_two(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.count(fn line ->
      [policy, password] = String.split(line, ": ")
      [range, letter] = String.split(policy, " ")
      [left, right] = String.split(range, "-") |> Enum.map(&String.to_integer/1)

      left_match = String.at(password, left - 1) == letter
      right_match = String.at(password, right - 1) == letter

      (left_match and not right_match) or (right_match and not left_match)
    end)
  end
end
