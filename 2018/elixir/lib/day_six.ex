defmodule ElixirSolutions.DaySix do
  @type name :: String.t()
  @type location :: %{x: integer, y: integer}
  @type coordinate :: %{name: name, location: location}
  @type distance :: integer

  @doc ~S"""
    Part 1

    iex> ElixirSolutions.DaySix.part_one("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9")
    17
  """

  @spec part_one(String.t()) :: integer
  def part_one(input) do
    coordinates = parse(input)
    {min, max} = find_range(coordinates)

    distance_map =
      for x <- min.x..max.x,
          y <- min.y..max.y,
          into: %{},
          do: {%{x: x, y: y}, distances(coordinates, %{x: x, y: y})}

    distance_map
    |> find_closest_to_each_location
    #    |> inspect_map
    |> remove_edge_coordinates(min, max)
    |> area_by_coordinate_name
    |> Enum.filter(fn {name, _size} -> not Enum.member?(~w{. !}, name) end)
    |> Enum.max_by(&elem(&1, 1))
    |> elem(1)
  end

  @spec inspect_map([{location, name}]) :: [{location, name}]
  defp inspect_map(names_by_location) do
    IO.puts("\n")

    names_by_location
    |> Enum.sort_by(fn {location, _name} -> [location.y, location.x] end)
    |> Enum.chunk_by(fn {location, _name} -> location.y end)
    |> Enum.each(fn group ->
      group
      |> Enum.reduce("", fn {_location, name}, all -> "#{all}#{name}" end)
      |> IO.puts()
    end)

    IO.puts("\n")

    names_by_location
  end

  @spec area_by_coordinate_name([{location, name}]) :: %{name: integer}
  defp area_by_coordinate_name(names_by_location) do
    Enum.reduce(names_by_location, %{}, fn {_location, name}, counts ->
      Map.update(counts, name, 1, &(&1 + 1))
    end)
  end

  @spec remove_edge_coordinates([{location, name}], location, location) :: [{location, name}]
  defp remove_edge_coordinates(names_by_location, min, max) do
    names_to_be_removed =
      for(
        x <- [min.x, max.x],
        y <- [min.y, max.y],
        do:
          Enum.find(names_by_location, fn {location, _name} ->
            location == %{x: x, y: y}
          end)
      )
      |> Enum.map(&elem(&1, 1))

    Enum.map(names_by_location, fn {location, name} ->
      if Enum.member?(names_to_be_removed, name) do
        {location, "!"}
      else
        {location, name}
      end
    end)
  end

  @spec find_closest_to_each_location(%{location: [{name, distance}]}) :: %{location: name}
  defp find_closest_to_each_location(distance_map) do
    distance_map
    |> Enum.map(fn {location, coordinate_distances} ->
      minimum_distance =
        coordinate_distances
        |> Enum.map(&elem(&1, 1))
        |> Enum.min()

      minimum_coordinate_names =
        coordinate_distances
        |> Enum.filter(fn {_name, distance} -> distance == minimum_distance end)
        |> Enum.map(&elem(&1, 0))

      tie = Enum.count(minimum_coordinate_names) > 1

      {location, if(tie, do: ".", else: List.first(minimum_coordinate_names))}
    end)
  end

  @spec distances(list(coordinate), location) :: list({name, distance})
  defp distances(coordinates, location) do
    coordinates
    |> Enum.map(fn coordinate ->
      {coordinate.name, distance(coordinate.location, location)}
    end)
  end

  @spec distance(location, location) :: distance
  defp distance(location1, location2) do
    abs(location1.x - location2.x) + abs(location1.y - location2.y)
  end

  @spec parse(String.t()) :: list(coordinate)
  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn line ->
      line
      |> String.split(", ", parts: 2)
      |> Enum.map(&String.to_integer/1)
      |> (fn [x, y] -> %{x: x, y: y} end).()
    end)
    |> Enum.zip(?A..?z)
    |> Enum.map(fn {location, letter} ->
      %{name: List.to_string([letter]), location: location}
    end)
  end

  @spec find_range(list(coordinate)) :: {coordinate, coordinate}
  defp find_range(coordinates) do
    coordinates
    |> Enum.reduce({{nil, nil}, {nil, nil}}, fn %{location: %{x: x, y: y}},
                                                {{min_x, min_y}, {max_x, max_y}} ->
      {
        {
          if(is_nil(min_x) or x < min_x, do: x, else: min_x),
          if(is_nil(min_y) or y < min_y, do: y, else: min_y)
        },
        {
          if(is_nil(max_x) or x > max_x, do: x, else: max_x),
          if(is_nil(max_y) or y > max_y, do: y, else: max_y)
        }
      }
    end)
    |> (fn {{min_x, min_y}, {max_x, max_y}} ->
          {
            %{x: min_x, y: min_y},
            %{x: max_x, y: max_y}
          }
        end).()
  end

  # ----------------------------------------------------------------------------

  @doc ~S"""
    Part 2

    iex> ElixirSolutions.DaySix.part_two("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9", 32)
    16
  """

  def part_two(input, maximum \\ 10_000) do
    coordinates = parse(input)
    {min, max} = find_range(coordinates)

    distance_map =
      for x <- min.x..max.x,
          y <- min.y..max.y,
          into: %{},
          do: {%{x: x, y: y}, distances(coordinates, %{x: x, y: y})}

    distance_map
    |> Enum.map(fn {location, coordinate_distances} ->
      {location,
       Enum.reduce(coordinate_distances, 0, fn {_name, distance}, total -> total + distance end)}
    end)
    |> Enum.filter(fn {_location, total_distance} -> total_distance < maximum end)
    |> Enum.count()
  end
end
