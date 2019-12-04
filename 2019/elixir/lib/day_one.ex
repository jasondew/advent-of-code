defmodule DayOne do
  @doc """
    Part 1

    iex> DayOne.part_one("12\\n14\\n1969\\n100756")
    34_241
  """

  def part_one(input) do
    input
    |> Conversions.to_integers()
    |> Enum.map(&fuel_required/1)
    |> Enum.sum()
  end

  @doc """
    Part 2

    iex> DayOne.part_two("14\\n1969\\n100756")
    51_314
  """

  def part_two(input) do
    input
    |> Conversions.to_integers()
    |> Enum.map(&fuel_required_including_fuel_mass/1)
    |> Enum.sum()
  end

  ## PRIVATE FUNCTIONS ##

  defp fuel_required(mass), do: trunc(mass / 3) - 2

  defp fuel_required_including_fuel_mass(mass) do
    fuel_mass = fuel_required(mass)

    if fuel_mass > 8 do
      fuel_mass + fuel_required_including_fuel_mass(fuel_mass)
    else
      fuel_mass
    end
  end
end
