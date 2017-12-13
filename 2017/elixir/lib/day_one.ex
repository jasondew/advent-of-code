defmodule DayOne do
  @doc ~S"""
    Part 1 solver

    1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.

      iex> DayOne.part_one("1122")
      3

    1111 produces 4 because each digit (all 1) matches the next.

      iex> DayOne.part_one("1111")
      4

    1234 produces 0 because no digit matches the next.

      iex> DayOne.part_one("1234")
      0

    91212129 produces 9 because the only digit that matches the next one is the last digit, 9.

      iex> DayOne.part_one("91212129")
      9
  """

  def part_one(input) do
    repeating_sum(input, 1)
  end

  @doc ~S"""
    Part 2 solver

    1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.

      iex> DayOne.part_two("1212")
      6

    1221 produces 0, because every comparison is between a 1 and a 2.

      iex> DayOne.part_two("1221")
      0

    123425 produces 4, because both 2s match each other, but no other digit has a match.

      iex> DayOne.part_two("123425")
      4

    123123 produces 12.

      iex> DayOne.part_two("123123")
      12

    12131415 produces 4.

      iex> DayOne.part_two("12131415")
      4
  """

  def part_two(input) do
    repeating_sum(input, div(String.length(input), 2))
  end

  defp repeating_sum(input, offset_amount) do
    charlist = String.to_charlist(input)

    Enum.zip(charlist, offset(charlist, offset_amount))
    |> Enum.map(fn {a, b} ->
         if a == b do
           String.to_integer(to_string([a]))
         else
           0
         end
       end)
    |> Enum.sum()
  end

  defp offset(charlist, amount) do
    Enum.slice(charlist, amount, length(charlist)) ++ Enum.take(charlist, amount)
  end
end
