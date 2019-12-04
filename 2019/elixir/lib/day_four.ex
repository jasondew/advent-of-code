defmodule DayFour do
  @doc """
    Part 1
  """
  def part_one(input) do
    [min, max | []] = Conversions.to_integers(input, "-")

    Enum.count(min..max, fn candidate ->
      digits = Integer.digits(candidate)
      contains_adjacent_digits?(digits) and non_decreasing_digits?(digits)
    end)
  end

  @doc """
    Part 2
  """
  def part_two(input) do
    [min, max | []] = Conversions.to_integers(input, "-")

    Enum.count(min..max, fn candidate ->
      digits = Integer.digits(candidate)

      contains_only_pairs_of_adjacent_digits?(digits) and
        non_decreasing_digits?(digits)
    end)
  end

  ## PRIVATE FUNCTIONS ##

  defp contains_adjacent_digits?(digits) do
    digits
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.any?(fn
      [x, x] -> true
      _ -> false
    end)
  end

  defp contains_only_pairs_of_adjacent_digits?(digits) do
    digits
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.filter(fn
      [x, x] -> true
      _ -> false
    end)
    |> Enum.reduce(%{}, fn x, map ->
      Map.update(map, x, 1, &(&1 + 1))
    end)
    |> Map.values()
    |> Enum.any?(&(&1 == 1))
  end

  defp non_decreasing_digits?(digits) do
    digits == Enum.sort(digits)
  end
end
