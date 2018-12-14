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

  @doc ~S"""
    iex> ElixirSolutions.DayTwelve.live_plant_index_sum(-3, ".#....##....#####...#######....#.#..##.")
    325
  """
  def live_plant_index_sum(initial_index, plants) do
    plants
    |> String.to_charlist()
    |> Enum.zip(initial_index..String.length(plants))
    |> Enum.reduce(0, fn {plant, index}, sum ->
      if plant == ?#, do: index + sum, else: sum
    end)
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

  # Part two code

  def part_two() do
    compute_live_plant_index_sum(49_999_999_966)
  end

  def part_two(input) do
    IO.inspect(input, label: String.pad_leading("0", 10))

    initial_state = {from_ascii(input), 0}

    {plants, initial_index} =
      1..500_000
      |> Enum.reduce(initial_state, fn generation, {plants, initial_index} ->
        plants = step(<<>>, <<0::1, 0::1, 0::1, plants::bitstring>>)
        {plants, initial_index} = trim_leading_zeros(plants, initial_index - 1)

        if Enum.member?([500, 5_000, 50_000, 500_000], generation) do
          IO.inspect(
            "#{to_ascii(plants)} => ii=#{to_string(initial_index)} s=#{bit_size(plants)} c=#{
              to_string(characteristic(plants, initial_index))
            }",
            label: String.pad_leading(to_string(generation), 10)
          )
        end

        {plants, initial_index}
      end)

    characteristic(plants, initial_index)
  end

  @doc ~S"""
    iex> ElixirSolutions.DayTwelve.compute_live_plant_index_sum(466)
    36768

    iex> ElixirSolutions.DayTwelve.compute_live_plant_index_sum(4966)
    347268

    iex> ElixirSolutions.DayTwelve.compute_live_plant_index_sum(49966)
    3452268

    iex> ElixirSolutions.DayTwelve.compute_live_plant_index_sum(499966)
    34502268
  """
  def compute_live_plant_index_sum(initial_index) do
    plant_indexes = initial_index..(initial_index + 132)
    indexes = 0..132

    Enum.reduce(Enum.zip(plant_indexes, indexes), 0, fn {plant_index, index}, sum ->
      add_index? =
        if index <= 122 do
          Integer.mod(index, 6) < 3
        else
          Enum.member?([125, 126, 127, 130, 131, 132], index)
        end

      if(add_index?, do: plant_index + sum, else: sum)
    end)
  end

  def trim_leading_zeros(<<0::1, rest::bitstring>>, initial_index),
    do: trim_leading_zeros(rest, initial_index + 1)

  def trim_leading_zeros(bits, initial_index), do: {bits, initial_index}

  def characteristic(bits, initial_index), do: characteristic(bits, initial_index, 0)
  def characteristic(<<>>, _, total), do: total

  def characteristic(<<bit::1, rest::bitstring>>, index, total) do
    characteristic(rest, index + 1, if(bit == 1, do: index + total, else: total))
  end

  def from_ascii(string) when is_binary(string) do
    convert_to_bits(<<>>, string)
  end

  def convert_to_bits(bits, ""), do: bits

  def convert_to_bits(bits, <<letter::binary-size(1)>> <> rest) do
    bit =
      case letter do
        "." -> <<0::1>>
        "#" -> <<1::1>>
      end

    convert_to_bits(<<bits::bitstring, bit::bitstring>>, rest)
  end

  def to_ascii(plants) when is_bitstring(plants) do
    convert_to_symbols([], plants)
  end

  def convert_to_symbols(symbols, <<>>), do: symbols |> Enum.reverse() |> List.to_string()

  def convert_to_symbols(symbols, <<first_bit::1, rest::bitstring>>) do
    symbol =
      case first_bit do
        0 -> "."
        1 -> "#"
      end

    convert_to_symbols([symbol | symbols], rest)
  end

  def step(done, <<first_bit::1, second_bit::1>>) do
    a = next(<<first_bit::1, second_bit::1, 0::1, 0::1, 0::1>>)

    if a == <<0::1>>, do: done, else: <<done::bitstring, a::bitstring>>
  end

  def step(done, <<first_bit::1, second_and_third_bits::2>>) do
    a = next(<<first_bit::1, second_and_third_bits::2, 0::1, 0::1>>)

    step(<<done::bitstring, a::bitstring>>, <<second_and_third_bits::2>>)
  end

  def step(done, <<first_bit::1, second_through_fourth_bits::3>>) do
    a = next(<<first_bit::1, second_through_fourth_bits::3, 0::1>>)

    step(<<done::bitstring, a::bitstring>>, <<second_through_fourth_bits::3>>)
  end

  def step(done, <<first_bit::1, second_through_fifth_bits::4, rest::bitstring>>) do
    a = next(<<first_bit::1, second_through_fifth_bits::4>>)
    b = <<second_through_fifth_bits::4>>

    step(<<done::bitstring, a::bitstring>>, <<b::bitstring, rest::bitstring>>)
  end

  # example rules
  #  def next(<<0::1, 0::1, 0::1, 1::1, 1::1>>), do: <<1::1>>
  #  def next(<<0::1, 0::1, 1::1, 0::1, 0::1>>), do: <<1::1>>
  #  def next(<<0::1, 1::1, 0::1, 0::1, 0::1>>), do: <<1::1>>
  #  def next(<<0::1, 1::1, 0::1, 1::1, 0::1>>), do: <<1::1>>
  #  def next(<<0::1, 1::1, 0::1, 1::1, 1::1>>), do: <<1::1>>
  #  def next(<<0::1, 1::1, 1::1, 0::1, 0::1>>), do: <<1::1>>
  #  def next(<<0::1, 1::1, 1::1, 1::1, 1::1>>), do: <<1::1>>
  #  def next(<<1::1, 0::1, 1::1, 0::1, 1::1>>), do: <<1::1>>
  #  def next(<<1::1, 0::1, 1::1, 1::1, 1::1>>), do: <<1::1>>
  #  def next(<<1::1, 1::1, 0::1, 1::1, 0::1>>), do: <<1::1>>
  #  def next(<<1::1, 1::1, 0::1, 1::1, 1::1>>), do: <<1::1>>
  #  def next(<<1::1, 1::1, 1::1, 0::1, 0::1>>), do: <<1::1>>
  #  def next(<<1::1, 1::1, 1::1, 0::1, 1::1>>), do: <<1::1>>
  #  def next(<<1::1, 1::1, 1::1, 1::1, 0::1>>), do: <<1::1>>
  #  def next(<<_::size(5)>>), do: <<0::1>>

  # my rules
  def next(<<1::1, 1::1, 1::1, 1::1, 1::1>>), do: <<1::1>>
  def next(<<1::1, 1::1, 1::1, 0::1, 1::1>>), do: <<1::1>>
  def next(<<1::1, 1::1, 1::1, 0::1, 0::1>>), do: <<1::1>>
  def next(<<1::1, 1::1, 0::1, 0::1, 1::1>>), do: <<1::1>>
  def next(<<1::1, 1::1, 0::1, 0::1, 0::1>>), do: <<1::1>>
  def next(<<1::1, 0::1, 1::1, 0::1, 1::1>>), do: <<1::1>>
  def next(<<1::1, 0::1, 1::1, 0::1, 0::1>>), do: <<1::1>>
  def next(<<1::1, 0::1, 0::1, 1::1, 0::1>>), do: <<1::1>>
  def next(<<0::1, 1::1, 1::1, 1::1, 0::1>>), do: <<1::1>>
  def next(<<0::1, 1::1, 1::1, 0::1, 1::1>>), do: <<1::1>>
  def next(<<0::1, 1::1, 1::1, 0::1, 0::1>>), do: <<1::1>>
  def next(<<0::1, 1::1, 0::1, 1::1, 1::1>>), do: <<1::1>>
  def next(<<0::1, 1::1, 0::1, 0::1, 0::1>>), do: <<1::1>>
  def next(<<0::1, 0::1, 1::1, 1::1, 0::1>>), do: <<1::1>>
  def next(<<0::1, 0::1, 1::1, 0::1, 1::1>>), do: <<1::1>>
  def next(<<0::1, 0::1, 0::1, 1::1, 0::1>>), do: <<1::1>>
  def next(<<_::size(5)>>), do: <<0::1>>
end
