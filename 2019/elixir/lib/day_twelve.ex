defmodule DayTwelve do
  @doc """
    Part 1

    iex> DayTwelve.part_one("<x=-1, y=0, z=2>\\n<x=2, y=-10, z=-7>\\n<x=4, y=-8, z=8>\\n<x=3, y=5, z=-1>", 10)
    179
  """
  def part_one(input, steps \\ 1_000) do
    Enum.reduce(1..steps, parse(input), fn _step, moons ->
      moons
      |> apply_gravity()
      |> apply_velocities()
    end)
    |> Enum.map(&total_energy/1)
    |> Enum.sum()
  end

  ## PRIVATE FUNCTIONS

  defp total_energy(%{position: {x, y, z}, velocity: {vx, vy, vz}}) do
    (abs(x) + abs(y) + abs(z)) * (abs(vx) + abs(vy) + abs(vz))
  end

  defp apply_velocities(moons) do
    Enum.map(moons, fn %{position: {x, y, z}, velocity: {vx, vy, vz}} ->
      %{position: {x + vx, y + vy, z + vz}, velocity: {vx, vy, vz}}
    end)
  end

  defp apply_gravity(moons) do
    Enum.map(moons, fn moon ->
      Enum.reduce(moons -- [moon], moon, fn other_moon, moon ->
        apply_gravity(moon, other_moon)
      end)
    end)
  end

  defp apply_gravity([], moons_done), do: moons_done

  defp apply_gravity(
         %{position: {x1, y1, z1}, velocity: {vx1, vy1, vz1}} = moon1,
         %{position: {x2, y2, z2}}
       ) do
    %{
      moon1
      | velocity: {
          apply_gravity(x1, x2, vx1),
          apply_gravity(y1, y2, vy1),
          apply_gravity(z1, z2, vz1)
        }
    }
  end

  defp apply_gravity(x1, x2, vx) do
    cond do
      x1 > x2 -> vx - 1
      x1 < x2 -> vx + 1
      x1 == x2 -> vx
    end
  end

  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn line ->
      position =
        line
        |> String.trim(">")
        |> String.split(", ")
        |> Enum.map(fn coordinate ->
          coordinate
          |> String.split("=")
          |> List.last()
          |> String.to_integer()
        end)
        |> List.to_tuple()

      %{position: position, velocity: {0, 0, 0}}
    end)
  end
end
