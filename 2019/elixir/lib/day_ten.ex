defmodule DayTen do
  @doc """
    Part 1

    iex> DayTen.part_one(".#..#\\n.....\\n#####\\n....#\\n...##")
    8

    iex> DayTen.part_one(Enum.join([
    ...> "......#.#.",
    ...> "#..#.#....",
    ...> "..#######.",
    ...> ".#.#.###..",
    ...> ".#..#.....",
    ...> "..#....#.#",
    ...> "#..#....#.",
    ...> ".##.#..###",
    ...> "##...#..#.",
    ...> ".#....####"
    ...> ], "\\n"))
    33
  """
  def part_one(input) do
    input
    |> asteroid_angles_and_distances()
    |> Enum.map(fn {_asteroid, other_asteroids} ->
      other_asteroids
      |> Enum.uniq_by(& &1.angle)
      |> Enum.count()
    end)
    |> Enum.max()
  end

  @doc """
    Part 2
    iex> DayTen.part_two(Enum.join([
    ...> ".#..##.###...#######",
    ...> "##.############..##.",
    ...> ".#.######.########.#",
    ...> ".###.#######.####.#.",
    ...> "#####.##.#.##.###.##",
    ...> "..#####..#.#########",
    ...> "####################",
    ...> "#.####....###.#.#.##",
    ...> "##.#################",
    ...> "#####.##.###..####..",
    ...> "..######..##.#######",
    ...> "####.##.####...##..#",
    ...> ".#####..#.######.###",
    ...> "##...#.##########...",
    ...> "#.##########.#######",
    ...> ".####.#.###.###.#.##",
    ...> "....##.##.###..#####",
    ...> ".#.#.###########.###",
    ...> "#.#.#.#####.####.###",
    ...> "###.##.####.##.#..##"
    ...> ], "\\n"))
    802
  """
  def part_two(input) do
    {_optimal_asteroid, angles_and_distances} =
      input
      |> asteroid_angles_and_distances()
      |> Enum.max_by(fn {_asteroid, other_asteroids} ->
        other_asteroids
        |> Enum.uniq_by(& &1.angle)
        |> Enum.count()
      end)

    {_angle, [%{to: {x, y}} | _rest]} =
      angles_and_distances
      |> Enum.group_by(& &1.angle)
      |> Enum.sort_by(fn {angle, _group} -> angle end)
      |> Enum.at(199)

    x * 100 + y
  end

  ## PRIVATE FUNCTIONS ##

  defp asteroid_angles_and_distances(input) do
    asteroids = parse(input)

    asteroids
    |> Enum.map(fn {x, y} = asteroid ->
      {
        asteroid,
        asteroids
        |> List.delete(asteroid)
        |> Enum.map(fn {other_x, other_y} = other_asteroid ->
          %{
            to: other_asteroid,
            angle:
              rotate(
                :math.atan2(other_y - y, other_x - x) * 180 / :math.pi(),
                90
              ),
            distance:
              :math.sqrt(:math.pow(other_y - y, 2) + :math.pow(other_x - x, 2))
          }
        end)
      }
    end)
  end

  defp rotate(angle, degrees) do
    raw_angle = angle + degrees

    cond do
      raw_angle >= 360.0 -> raw_angle - 360.0
      raw_angle < 0 -> raw_angle + 360.0
      true -> raw_angle
    end
  end

  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.with_index()
    |> Enum.flat_map(fn {row_string, row} ->
      row_string
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.reduce([], fn
        {"#", column}, acc -> [{column, row} | acc]
        {_, _}, acc -> acc
      end)
      |> Enum.reverse()
    end)
  end
end
