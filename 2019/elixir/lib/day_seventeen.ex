defmodule DaySeventeen do
  @doc """
    Part 1
  """
  def part_one(input) do
    input
    |> map()
    |> points()
    |> elem(0)
    |> intersections()
    |> Enum.reduce(0, fn {x, y}, sum -> sum + x * y end)
  end

  @functions ~w[A B C]a

  @doc """
    Part 2
  """
  def part_two(input) do
    {points, starting_point} =
      input
      |> map()
      |> points()

    {_count, path, dictionary} =
      points
      |> path([starting_point], :north)
      |> reduce_all(@functions)
      |> Enum.filter(fn {size, _path, _dictionary} -> size <= 20 end)
      |> List.first()

    input
    |> String.replace_leading("1", "2")
    |> Intcode.new(inputs(path, dictionary))
    |> Intcode.run()
    |> Intcode.pop_outputs()
    |> elem(0)
    |> List.last()
  end

  ## PRIVATE FUNCTIONS ##

  defp inputs(path, dictionary) do
    [
      path,
      function_definitions(dictionary),
      "n\n"
    ]
    |> Enum.join("\n")
    |> String.to_charlist()
  end

  defp function_definitions(dictionary) do
    @functions
    |> Enum.map(fn function ->
      dictionary
      |> Map.get(function)
      |> stringify_path()
    end)
    |> Enum.join("\n")
  end

  defp reduce_all(path, functions, dictionary \\ %{})

  defp reduce_all(path, [], dictionary) do
    string_path = stringify_path(path)
    [{String.length(string_path), string_path, dictionary}]
  end

  defp reduce_all(path, [function | functions], dictionary) do
    path
    |> possible_reductions()
    |> Enum.map(fn {run, positions} ->
      reduce_all(
        replace(path, run, positions, function),
        functions,
        Map.put(dictionary, function, run)
      )
    end)
    |> Enum.concat()
  end

  defp possible_reductions(path) do
    2..7
    |> Enum.map(fn size ->
      path
      |> group(size)
      |> Enum.filter(fn {run, positions} ->
        String.length(stringify_path(run)) <= 20 and
          Enum.count(run, &is_atom/1) == 0 and
          Enum.count(positions) > 1
      end)
    end)
    |> Enum.concat()
  end

  defp replace(path, run, positions, function) do
    positions
    |> Enum.reduce({path, 0}, fn position, {path, offset} ->
      {prefix, rest} = Enum.split(path, position - offset)
      {^run, suffix} = Enum.split(rest, Enum.count(run))

      {
        Enum.concat(prefix, [function | suffix]),
        offset + Enum.count(run) - 1
      }
    end)
    |> elem(0)
  end

  def stringify_path(path) do
    path
    |> Enum.map(fn
      function when is_atom(function) -> "#{function}"
      {turn, count} -> "#{turn},#{count}"
    end)
    |> Enum.join(",")
  end

  defp group(path, size) do
    path
    |> Enum.chunk_every(size, 1, :discard)
    |> Enum.with_index()
    |> Enum.group_by(fn {group, _position} -> group end)
    |> Enum.map(fn {group, groups_and_positions} ->
      {
        group,
        groups_and_positions
        |> Enum.map(fn {_group, position} -> position end)
        |> Enum.reduce([], fn
          position, [] ->
            [position]

          position, [last_position | positions] ->
            if position - last_position >= size do
              [position, last_position | positions]
            else
              [last_position | positions]
            end
        end)
        |> Enum.reverse()
      }
    end)
  end

  defp path(points, [current_point | _] = visited, heading, runs \\ []) do
    next_point = next_point(current_point, heading)

    if available?(points, next_point) do
      [{turn, count} | rest_runs] = runs

      path(
        points,
        [next_point | visited],
        heading,
        [{turn, count + 1} | rest_runs]
      )
    else
      case turn(points, visited, current_point, heading) do
        {direction, next_point, new_heading} ->
          path(points, [next_point | visited], new_heading, [
            {direction, 1} | runs
          ])

        _ ->
          Enum.reverse(runs)
      end
    end
  end

  defp turn(points, visited, current_point, heading) do
    heading
    |> possible_next_headings()
    |> Enum.map(
      &{turn_direction(heading, &1), next_point(current_point, &1), &1}
    )
    |> Enum.find(fn {_direction, next_point, _new_heading} ->
      available_and_unvisited?(points, visited, next_point)
    end)
  end

  defp possible_next_headings(:north), do: ~w[east west]a
  defp possible_next_headings(:south), do: ~w[east west]a
  defp possible_next_headings(:east), do: ~w[north south]a
  defp possible_next_headings(:west), do: ~w[north south]a

  defp turn_direction(:north, :east), do: "R"
  defp turn_direction(:east, :south), do: "R"
  defp turn_direction(:south, :west), do: "R"
  defp turn_direction(:west, :north), do: "R"
  defp turn_direction(_, _), do: "L"

  defp next_point({x, y}, :north), do: {x, y - 1}
  defp next_point({x, y}, :south), do: {x, y + 1}
  defp next_point({x, y}, :west), do: {x - 1, y}
  defp next_point({x, y}, :east), do: {x + 1, y}

  defp available_and_unvisited?(points, visited, point) do
    available?(points, point) and not Enum.member?(visited, point)
  end

  defp available?(points, point), do: Enum.member?(points, point)

  defp map(intcode) do
    intcode
    |> Intcode.new()
    |> Intcode.run()
    |> Intcode.pop_outputs()
    |> elem(0)
    |> List.to_string()
  end

  defp points(map) do
    {points, starting_point, _y} =
      map
      |> String.split("\n")
      |> Enum.reduce({[], nil, 0}, fn line, {path_points, starting_point, y} ->
        {path_points, starting_point, _x} =
          line
          |> String.graphemes()
          |> Enum.reduce({path_points, starting_point, 0}, fn char,
                                                              {path_points,
                                                               starting_point,
                                                               x} ->
            cond do
              char == "#" -> {[{x, y} | path_points], starting_point, x + 1}
              char == "^" -> {path_points, {x, y}, x + 1}
              true -> {path_points, starting_point, x + 1}
            end
          end)

        {path_points, starting_point, y + 1}
      end)

    {Enum.reverse(points), starting_point}
  end

  defp intersections(points) do
    points
    |> Enum.filter(fn {x, y} ->
      Enum.member?(points, {x, y - 1}) && Enum.member?(points, {x, y + 1}) &&
        Enum.member?(points, {x - 1, y}) && Enum.member?(points, {x + 1, y})
    end)
  end
end
