defmodule DayEleven do
  @doc """
    Part 1
  """
  def part_one(input) do
    input
    |> Conversions.to_integers(",")
    |> Intcode.new()
    |> run({0, 0}, :north, %{})
    |> Enum.count()
  end

  @doc """
    Part 2
  """
  def part_two(input) do
    input
    |> Conversions.to_integers(",")
    |> Intcode.new()
    |> run({0, 0}, :north, %{{0, 0} => 1})
    |> to_ascii_bitmap()
  end

  ## PRIVATE FUNCTIONS ##

  defp run(configuration, location, heading, paints) do
    current_color = Map.get(paints, location, 0)

    configuration =
      %{configuration | inputs: [current_color], outputs: []}
      |> Intcode.run()

    case configuration do
      %{
        state: :waiting_for_input,
        outputs: [direction_to_turn, color_to_paint | []]
      } ->
        new_heading = turn(heading, direction_to_turn)

        run(
          configuration,
          move(location, new_heading),
          new_heading,
          Map.put(paints, location, color_to_paint)
        )

      %{
        state: :done,
        outputs: [_direction_to_turn, color_to_paint | []]
      } ->
        Map.put(paints, location, color_to_paint)
    end
  end

  defp move({x, y}, :north), do: {x, y + 1}
  defp move({x, y}, :south), do: {x, y - 1}
  defp move({x, y}, :east), do: {x + 1, y}
  defp move({x, y}, :west), do: {x - 1, y}

  defp turn(:north, 0), do: :west
  defp turn(:north, 1), do: :east
  defp turn(:south, 0), do: :east
  defp turn(:south, 1), do: :west
  defp turn(:east, 0), do: :north
  defp turn(:east, 1), do: :south
  defp turn(:west, 0), do: :south
  defp turn(:west, 1), do: :north

  defp to_ascii_bitmap(point_colors) do
    points = Map.keys(point_colors)
    {min_x, max_x} = points |> Enum.map(fn {x, _y} -> x end) |> Enum.min_max()
    {min_y, max_y} = points |> Enum.map(fn {_x, y} -> y end) |> Enum.min_max()

    height = max_y - min_y
    width = max_x - min_x
    bitmap = for _ <- 0..height, do: for(_ <- 0..width, do: ".")

    point_colors
    |> Enum.reduce(bitmap, fn
      {{x, y}, 1}, bitmap ->
        List.update_at(bitmap, -1 * y, &List.replace_at(&1, x - 1, "#"))

      _, bitmap ->
        bitmap
    end)
  end
end
