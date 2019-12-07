defmodule DaySix do
  @doc """
    Part 1

    iex> DaySix.part_one("COM)B\\nB)C\\nC)D\\nD)E\\nE)F\\nB)G\\nG)H\\nD)I\\nE)J\\nJ)K\\nK)L")
    42
  """
  def part_one(input) do
    orbit_map = build_orbit_map(input)

    orbit_map
    |> Map.keys()
    |> Enum.reduce(0, fn object, checksum ->
      checksum + Enum.count(orbits(orbit_map, object))
    end)
  end

  @doc """
    Part 2

    iex> DaySix.part_two("COM)B\\nB)C\\nC)D\\nD)E\\nE)F\\nB)G\\nG)H\\nD)I\\nE)J\\nJ)K\\nK)L\\nK)YOU\\nI)SAN")
    4
  """
  def part_two(input) do
    orbit_map = build_orbit_map(input)

    my_orbits = orbits(orbit_map, "YOU")
    santas_orbits = orbits(orbit_map, "SAN")
    common_object = Enum.find(my_orbits, &Enum.member?(santas_orbits, &1))

    depth(my_orbits, common_object) + depth(santas_orbits, common_object)
  end

  ## PRIVATE FUNCTIONS ##

  defp build_orbit_map(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn string ->
      string
      |> String.split(")")
      |> Enum.reverse()
      |> List.to_tuple()
    end)
    |> Enum.into(%{})
  end

  defp orbits(orbit_map, object, orbits \\ [])

  defp orbits(_orbit_map, "COM", orbits) do
    Enum.reverse(orbits)
  end

  defp orbits(orbit_map, object, orbits) do
    orbitee = Map.get(orbit_map, object)

    orbits(orbit_map, orbitee, [orbitee | orbits])
  end

  defp depth([], _object), do: -1
  defp depth([object | _orbits], object), do: 0

  defp depth([_non_matching_object | orbits], object),
    do: 1 + depth(orbits, object)
end
