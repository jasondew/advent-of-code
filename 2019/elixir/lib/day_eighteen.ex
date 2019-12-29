defmodule DayEighteen do
  @doc """
    iex> DayEighteen.part_one("#########\\n#b.A.@.a#\\n#########")
    8

    iex> DayEighteen.part_one("########################\\n#f.D.E.e.C.b.A.@.a.B.c.#\\n######################.#\\n#d.....................#\\n########################\\n")
    86

    iex> DayEighteen.part_one("########################\\n...............b.C.D.f#\\n.######################\\n.....@.a.B.c.d.A.e.F.g#\\n#######################\\n")
    132

    iex> DayEighteen.part_one("#################\\n#i.G..c...e..H.p#\\n########.########\\n#j.A..b...f..D.o#\\n########@########\\n#k.E..a...g..B.n#\\n########.########\\n#l.F..d...h..C.m#\\n#################")
    136

    iex> DayEighteen.part_one("########################\\n#@..............ac.GI.b#\\n###d#e#f################\\n###A#B#C################\\n###g#h#i################\\n########################")
    81
  """
  def part_one(input) do
    {map, keys, start} = parse(input)
    :ets.new(:part1_cache, [:named_table])

    map
    |> key_tree(keys, start)
    |> shortest_tour(:start, key_names(keys), [], [], :part1_cache)
    |> elem(0)
  end

  @doc """
    iex> DayEighteen.part_two("#######\\n#a.#Cd#\\n##...##\\n##.@.##\\n##...##\\n#cB#Ab#\\n#######")
    8

    iex> DayEighteen.part_two("###############\\n#d.ABC.#.....a#\\n######...######\\n######.@.######\\n######...######\\n#b.....#.....c#\\n###############")
    24
  """
  def part_two(input) do
    {map, keys, {x, y} = center} = parse(input)

    map =
      map
      |> Map.put({x, y}, :wall)
      |> Map.put({x - 1, y}, :wall)
      |> Map.put({x + 1, y}, :wall)
      |> Map.put({x, y - 1}, :wall)
      |> Map.put({x, y + 1}, :wall)

    1..4
    |> Enum.map(fn quadrant ->
      start = start(center, quadrant)

      key_tree = key_tree(map, keys, start)

      {required_key_names, assumed_key_names} =
        partition_keys(keys, center, quadrant)

      cache_name = String.to_atom("part2_quadrant#{quadrant}_cache")
      :ets.new(cache_name, [:named_table])

      shortest_tour(
        key_tree,
        :start,
        required_key_names,
        [],
        assumed_key_names,
        cache_name
      )
    end)
    |> Enum.map(&elem(&1, 0))
    |> Enum.sum()
  end

  ## PRIVATE FUNCTIONS ##

  defp start({x, y}, 1), do: {x - 1, y - 1}
  defp start({x, y}, 2), do: {x + 1, y - 1}
  defp start({x, y}, 3), do: {x - 1, y + 1}
  defp start({x, y}, 4), do: {x + 1, y + 1}

  defp partition_keys(keys, center, quadrant) do
    {required, assumed} =
      Enum.reduce(keys, {[], []}, fn {key_name, key_location},
                                     {required, assumed} ->
        if in_quadrant?(center, quadrant, key_location) do
          {[key_name | required], assumed}
        else
          {required, [key_name | assumed]}
        end
      end)

    {Enum.sort(required), Enum.sort(assumed)}
  end

  defp in_quadrant?({x, y}, 1, {kx, ky}), do: kx < x && ky < y
  defp in_quadrant?({x, y}, 2, {kx, ky}), do: kx > x && ky < y
  defp in_quadrant?({x, y}, 3, {kx, ky}), do: kx < x && ky > y
  defp in_quadrant?({x, y}, 4, {kx, ky}), do: kx > x && ky > y

  defp shortest_tour(
         _key_tree,
         _key,
         all_keys,
         all_keys,
         _keys_assumed,
         _cache
       ),
       do: {0, []}

  defp shortest_tour(
         key_tree,
         key,
         all_keys,
         keys_collected,
         keys_assumed,
         cache
       ) do
    case :ets.lookup(cache, {key, keys_collected}) do
      [] ->
        min =
          key_tree
          |> reachable_keys(key, Enum.concat(keys_collected, keys_assumed))
          |> Enum.map(fn {{_key, next_key},
                          {distance, _keys_required, keys_collected_during_path}} ->
            keys_collected =
              (keys_collected
               |> Enum.concat(keys_collected_during_path)
               |> Enum.sort()
               |> Enum.uniq()) -- keys_assumed

            case shortest_tour(
                   key_tree,
                   next_key,
                   all_keys,
                   keys_collected,
                   keys_assumed,
                   cache
                 ) do
              {rest_distance, path} ->
                {distance + rest_distance, [next_key | path]}
            end
          end)
          |> case do
            [] ->
              nil

            values ->
              Enum.min(values)
          end

        :ets.insert(cache, {{key, keys_collected}, min})
        min

      [{_cache_key, distance}] ->
        distance
    end
  end

  defp reachable_keys(key_tree, key, keys_collected) do
    Enum.filter(key_tree, fn
      {{^key, next_key},
       {_distance, keys_required, _keys_collected_during_path}} ->
        not Enum.member?(keys_collected, next_key) &&
          Enum.empty?(keys_required -- keys_collected)

      _ ->
        false
    end)
  end

  defp key_tree(map, keys, start) do
    Enum.reduce(Map.put(keys, :start, start), %{}, fn {key, location}, acc ->
      Enum.reduce(
        Map.delete(keys, key),
        acc,
        fn {other_key, other_location}, acc ->
          case shortest_path(map, location, other_location) do
            :no_path ->
              acc

            result ->
              Map.put(acc, {key, other_key}, result)
          end
        end
      )
    end)
  end

  defp key_names(keys) do
    keys
    |> Map.keys()
    |> Enum.sort()
  end

  defp shortest_path(_map, point, point), do: {0, []}

  defp shortest_path(map, from, to) do
    case astar(map, from, to) do
      [] ->
        :no_path

      path ->
        {Enum.count(path), keys_required(map, path), keys_attained(map, path)}
    end
  end

  defp keys_required(map, path) do
    Enum.reduce(path, [], fn point, keys ->
      case Map.get(map, point) do
        {:door, letter} -> [letter | keys]
        _ -> keys
      end
    end)
  end

  defp keys_attained(map, path) do
    Enum.reduce(path, [], fn point, keys ->
      case Map.get(map, point) do
        {:key, letter} -> [letter | keys]
        _ -> keys
      end
    end)
  end

  defp astar(map, from, to) do
    Astar.astar(
      {
        fn point -> neighbors(map, point) end,
        fn _, _ -> 1 end,
        fn {x1, y1}, {x2, y2} -> abs(x1 - x2) + abs(y1 - y2) end
      },
      from,
      to
    )
  end

  defp neighbors(map, {x, y}) do
    [
      {x + 1, y},
      {x - 1, y},
      {x, y + 1},
      {x, y - 1}
    ]
    |> Enum.reduce([], fn point, points ->
      case Map.get(map, point, :wall) do
        :wall -> points
        _ -> [point | points]
      end
    end)
  end

  defp parse(input) do
    map = parse_map(input)
    {start, :start} = Enum.find(map, fn {_, type} -> type == :start end)

    keys =
      Enum.reduce(map, %{}, fn
        {location, {:key, key}}, keys -> Map.put(keys, key, location)
        _, keys -> keys
      end)

    {map, keys, start}
  end

  defp parse_map(input) do
    input
    |> String.split("\n")
    |> Enum.reduce({%{}, 0}, fn line, {map, y} ->
      map =
        line
        |> String.graphemes()
        |> Enum.reduce({map, 0}, fn char, {map, x} ->
          type =
            case char do
              "#" ->
                :wall

              "." ->
                :open

              "@" ->
                :start

              letter ->
                if letter == String.downcase(letter) do
                  {:key, letter |> String.to_atom()}
                else
                  {:door, letter |> String.downcase() |> String.to_atom()}
                end
            end

          {Map.put(map, {x, y}, type), x + 1}
        end)
        |> elem(0)

      {map, y + 1}
    end)
    |> elem(0)
  end
end
