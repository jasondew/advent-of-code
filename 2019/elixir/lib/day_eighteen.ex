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
    map = parse(input)
    {start, :start} = Enum.find(map, fn {_, type} -> type == :start end)

    keys =
      Enum.reduce(map, %{}, fn
        {location, {:key, key}}, keys -> Map.put(keys, key, location)
        _, keys -> keys
      end)

    sorted_key_names = keys |> Map.keys() |> Enum.sort()

    key_tree =
      key_tree(map, keys, start)
      |> Enum.sort_by(fn {{from, to}, {_, _, kc}} ->
        if from == :start,
          do: [Enum.count(kc), to],
          else: [from, Enum.count(kc), to]
      end)

    :ets.new(:cache, [:named_table])

    key_tree
    |> shortest_tour(:start, sorted_key_names, [])
    |> elem(0)
  end

  ## PRIVATE FUNCTIONS ##

  defp shortest_tour(_key_tree, _key, all_keys, all_keys), do: {0, []}

  defp shortest_tour(key_tree, key, all_keys, keys_collected) do
    case :ets.lookup(:cache, {key, keys_collected}) do
      [] ->
        min =
          key_tree
          |> reachable_keys(key, keys_collected)
          |> Enum.map(fn {{_key, next_key},
                          {distance, _keys_required, keys_collected_during_path}} ->
            case shortest_tour(
                   key_tree,
                   next_key,
                   all_keys,
                   keys_collected
                   |> Enum.concat(keys_collected_during_path)
                   |> Enum.sort()
                   |> Enum.uniq()
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

        :ets.insert(:cache, {{key, keys_collected}, min})
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
    Enum.reduce(Map.put(keys, :start, start), %{}, fn {key, location}, cache ->
      Enum.reduce(
        Map.delete(keys, key),
        cache,
        fn {other_key, other_location}, cache ->
          case shortest_path(map, location, other_location) do
            :no_path ->
              cache

            result ->
              Map.put(cache, {key, other_key}, result)
          end
        end
      )
    end)
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
