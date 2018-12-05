defmodule ElixirSolutions.DayFive do
  @doc ~S"""
    Part 1

    iex> ElixirSolutions.DayFive.part_one("dabAcCaCBAcCcaDA")
    10
  """

  def part_one(input) do
    input
    |> String.graphemes()
    |> remove_all_reactive_pairs()
    |> Enum.count()
  end

  defp remove_all_reactive_pairs(units) do
    case remove_reactive_pairs(units) do
      {True, units} -> remove_all_reactive_pairs(units)
      {False, units} -> units
    end
  end

  defp remove_reactive_pairs([first_unit | units]) do
    units
    |> Enum.reduce({[], first_unit, False}, fn unit, {seen_units, previous_unit, found_one} ->
      if react?(unit, previous_unit) do
        {seen_units, nil, True}
      else
        if previous_unit do
          {[previous_unit | seen_units], unit, found_one}
        else
          {seen_units, unit, found_one}
        end
      end
    end)
    |> (fn {seen_units, last_unit, found_one} -> {found_one, [last_unit | seen_units]} end).()
  end

  defp react?(a, b) do
    not is_nil(a) and not is_nil(b) and a != b and String.downcase(a) == String.downcase(b)
  end

  @doc ~S"""
    iex> ElixirSolutions.DayFive.part_two("dabAcCaCBAcCcaDA")
    4
  """

  def part_two(input) do
    input
    |> String.downcase()
    |> String.graphemes()
    |> Enum.uniq()
    |> Enum.map(fn unit_type ->
      input
      |> remove_units_of_type(unit_type)
      |> String.graphemes()
      |> remove_all_reactive_pairs()
      |> Enum.count()
    end)
    |> Enum.min()
  end

  defp remove_units_of_type(string, unit_type) do
    string
    |> String.replace(String.upcase(unit_type), "")
    |> String.replace(String.downcase(unit_type), "")
  end
end
