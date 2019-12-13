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

  def part_two(input) do
    moons = parse(input)

    x_period =
      moons
      |> Enum.map(fn %{position: {x, _, _}} -> x end)
      |> List.to_tuple()
      |> find_period()
      |> IO.inspect(label: "x")

    y_period =
      moons
      |> Enum.map(fn %{position: {_, y, _}} -> y end)
      |> List.to_tuple()
      |> find_period()
      |> IO.inspect(label: "y")

    z_period =
      moons
      |> Enum.map(fn %{position: {_, _, z}} -> z end)
      |> List.to_tuple()
      |> find_period()
      |> IO.inspect(label: "z")

    x_period
    |> lcm(y_period)
    |> lcm(z_period)
  end

  ## PRIVATE FUNCTIONS

  defp lcm(a, b), do: div(a, Integer.gcd(a, b)) * b

  defp find_period(initial_positions, max_iterations \\ 1_000_000) do
    Enum.reduce_while(
      2..max_iterations,
      {initial_positions, {0, 0, 0, 0}},
      fn iteration, {positions, velocities} ->
        {positions, velocities} = step_single_dimension(positions, velocities)

        if positions == initial_positions do
          {:halt, iteration}
        else
          {:cont, {positions, velocities}}
        end
      end
    )
  end

  defp step_single_dimension({a, b, c, d}, {va, vb, vc, vd}) do
    va =
      va
      |> apply_gravity(a, b)
      |> apply_gravity(a, c)
      |> apply_gravity(a, d)

    vb =
      vb
      |> apply_gravity(b, a)
      |> apply_gravity(b, c)
      |> apply_gravity(b, d)

    vc =
      vc
      |> apply_gravity(c, a)
      |> apply_gravity(c, b)
      |> apply_gravity(c, d)

    vd =
      vd
      |> apply_gravity(d, a)
      |> apply_gravity(d, b)
      |> apply_gravity(d, c)

    {
      {a + va, b + vb, c + vc, d + vd},
      {va, vb, vc, vd}
    }
  end

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

  defp apply_gravity(
         %{position: {x1, y1, z1}, velocity: {vx1, vy1, vz1}} = moon1,
         %{position: {x2, y2, z2}}
       ) do
    %{
      moon1
      | velocity: {
          apply_gravity(vx1, x1, x2),
          apply_gravity(vy1, y1, y2),
          apply_gravity(vz1, z1, z2)
        }
    }
  end

  defp apply_gravity(vx, x1, x2) do
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
