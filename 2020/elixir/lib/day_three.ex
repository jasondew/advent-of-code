defmodule DayThree do
  @doc """
    Part 1

    iex> DayThree.part_one("..##.......\\n#...#...#..\\n.#....#..#.\\n..#.#...#.#\\n.#...##..#.\\n..#.##.....\\n.#.#.#....#\\n.#........#\\n#.##...#...\\n#...##....#\\n.#..#...#.#\\n")
    7
  """
  def part_one(input) do
    map = parse_map(input)
    collision_count(map, right: 3, down: 1)
  end

  @doc """
    Part 2

    iex> DayThree.part_two("..##.......\\n#...#...#..\\n.#....#..#.\\n..#.#...#.#\\n.#...##..#.\\n..#.##.....\\n.#.#.#....#\\n.#........#\\n#.##...#...\\n#...##....#\\n.#..#...#.#\\n")
    336
  """
  def part_two(input) do
    map = parse_map(input)

    collision_count(map, right: 1, down: 1) *
      collision_count(map, right: 3, down: 1) *
      collision_count(map, right: 5, down: 1) *
      collision_count(map, right: 7, down: 1) *
      collision_count(map, right: 1, down: 2)
  end

  ## PRIVATE FUNCTIONS

  defp collision_count(map, direction) do
    collision_count(map, direction, {0, 0}, 0)
  end

  defp collision_count(map, direction, {x, y}, trees) do
    if y >= map.row_count do
      trees
    else
      collision_count(
        map,
        direction,
        {Integer.mod(x + direction[:right], map.column_count), y + direction[:down]},
        if(tree?(map, x, y), do: trees + 1, else: trees)
      )
    end
  end

  defp tree?(map, x, y) do
    case map.data
         |> Enum.at(y)
         |> Enum.at(x) do
      :tree -> true
      :open -> false
    end
  end

  defp parse_map(input) do
    data =
      input
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(fn line ->
        line
        |> String.trim()
        |> String.graphemes()
        |> Enum.map(fn
          "#" -> :tree
          "." -> :open
        end)
      end)

    row_count = Enum.count(data)
    column_count = data |> List.first() |> Enum.count()

    %{data: data, row_count: row_count, column_count: column_count}
  end
end
