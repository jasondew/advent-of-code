defmodule ElixirSolutions.DayTwelve do
  @doc ~S"""
    iex> input = "initial state: #..#.#..##......###...###
    ...>
    ...>...## => #
    ...>..#.. => #
    ...>.#... => #
    ...>.#.#. => #
    ...>.#.## => #
    ...>.##.. => #
    ...>.#### => #
    ...>#.#.# => #
    ...>#.### => #
    ...>##.#. => #
    ...>##.## => #
    ...>###.. => #
    ...>###.# => #
    ...>####. => #"
    iex> ElixirSolutions.DayTwelve.part_one(input)
    325
  """
  def part_one(input) do
    {state, rules} = parse(input)

    1..20
    |> Enum.reduce(state, fn _, state -> next_generation(state, rules) end)
    |> live_plant_index_sum()
  end

  def part_two(input) do
    {state, rules} = parse(input)

    1..50_000_000_000
    |> Enum.reduce(state, fn generation, state ->
      if Integer.mod(generation, 100_000_000) == 0, do: IO.write(".")
      next_generation(state, rules)
    end)
    |> live_plant_index_sum()
  end

  @doc ~S"""
    iex> state = {-3, ".#....##....#####...#######....#.#..##."}
    iex> ElixirSolutions.DayTwelve.live_plant_index_sum(state)
    325
  """
  def live_plant_index_sum({initial_index, plants}) do
    plants
    |> String.to_charlist()
    |> Enum.zip(initial_index..String.length(plants))
    |> Enum.reduce(0, fn {plant, index}, sum ->
      if plant == ?#, do: index + sum, else: sum
    end)
  end

  @doc ~S"""
    iex> state = {0, "#..#.#..##......###...###"}
    iex> rules = %{
    ...> "...##" => "#",
    ...> "..#.." => "#",
    ...> ".#..." => "#",
    ...> ".#.#." => "#",
    ...> ".#.##" => "#",
    ...> ".##.." => "#",
    ...> ".####" => "#",
    ...> "#.#.#" => "#",
    ...> "#.###" => "#",
    ...> "##.#." => "#",
    ...> "##.##" => "#",
    ...> "###.." => "#",
    ...> "###.#" => "#",
    ...> "####." => "#"
    ...> }
    iex> ElixirSolutions.DayTwelve.next_generation(state, rules)
    {-1, ".#...#....#.....#..#..#..#"}
    iex> Enum.reduce(1..3, state, fn _, state ->
    ...>   ElixirSolutions.DayTwelve.next_generation(state, rules)
    ...> end)
    {-3, "..#.#...#..#.#....#..#..#...#"}
    iex> Enum.reduce(1..20, state, fn _, state ->
    ...>   ElixirSolutions.DayTwelve.next_generation(state, rules)
    ...> end)
    {-6, "....#....##....#####...#######....#.#..##"}
  """
  def next_generation({initial_index, plants}, rules) do
    next_plants =
      plants
      |> each_with_neighbors()
      |> Enum.map(&Map.get(rules, &1, "."))
      |> Enum.join()
      |> String.trim_trailing(".")

    if String.starts_with?(next_plants, ".....") do
      {initial_index, String.slice(next_plants, 1, String.length(next_plants))}
    else
      {initial_index - 1, next_plants}
    end
  end

  @doc ~S"""
    iex> ElixirSolutions.DayTwelve.each_with_neighbors("#..#")
    [
      "...#.",
      "..#..",
      ".#..#",
      "#..#.",
      "..#..",
      ".#..."
    ]
  """
  def each_with_neighbors(plants) do
    padded_plants = "...#{plants}..."
    length = String.length(padded_plants)

    0..(length - 5)
    |> Enum.map(fn offset ->
      String.slice(padded_plants, offset, 5)
    end)
  end

  @doc ~S"""
    iex> ElixirSolutions.DayTwelve.parse("initial state: #.##.##\n\n..### => .\n##..# => #\n#..## => .")
    {{0, "#.##.##"}, %{"..###" => ".", "##..#" => "#", "#..##" => "."}}
  """
  def parse(input) do
    [initial_state_line, _blank_line | rule_lines] = String.split(input, "\n")
    [_label, initial_state] = String.split(initial_state_line, ": ")

    rules =
      rule_lines
      |> Enum.map(&List.to_tuple(String.split(&1, " => ")))
      |> Map.new()

    {{0, initial_state}, rules}
  end
end
