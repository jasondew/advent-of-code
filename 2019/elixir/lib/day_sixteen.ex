defmodule DaySixteen do
  @doc """
    Part 1

      iex> DaySixteen.part_one("12345678", 1)
      48226158

      iex> DaySixteen.part_one("12345678", 4)
      01029498

      iex> DaySixteen.part_one("80871224585914546619083218645595")
      24176176

      iex> DaySixteen.part_one("19617804207202209144916044189917")
      73745418

      iex> DaySixteen.part_one("69317163492948606335995924319873")
      52432133
  """
  def part_one(input, phases \\ 100) do
    1..phases
    |> Enum.reduce(parse(input), fn _, values ->
      phase(values, [0, 1, 0, -1])
    end)
    |> Enum.take(8)
    |> Integer.undigits()
  end

  @doc """
    Part 2

    iex> DaySixteen.part_two("03036732577212944063491565474664")
    84462026

    iex> DaySixteen.part_two("02935109699940807407585447034323")
    78725270

    iex> DaySixteen.part_two("03081770884921959731165446850517")
    53553731
  """
  def part_two(input, phases \\ 100) do
    input = parse(input)

    start =
      input
      |> Enum.slice(0, 7)
      |> Integer.undigits()

    input =
      for(_reps <- 1..10_000, x <- input, do: x)
      |> Enum.slice(start, 8)

    Enum.reduce(1..phases, input, fn _, values ->
      phase(values, [0, 1, 0, -1], start, start + 8)
    end)
    |> IO.inspect(label: "output")
    |> Integer.undigits()
  end

  ## PRIVATE FUNCTIONS

  defp phase(values, generator, start \\ 1, stop \\ nil) do
    stop = if is_nil(stop), do: length(values), else: stop

    start..stop
    |> Enum.map(fn round ->
      pattern = for x <- generator, _y <- 1..round, do: x

      for(
        _y <- 0..ceil(length(values) / length(pattern)),
        x <- pattern,
        do: x
      )
      |> output(values, start)
    end)
  end

  defp output(pattern, values, drop_count) do
    pattern
    |> Enum.drop(drop_count)
    |> Enum.zip(values)
    |> Enum.reduce(0, fn {x, y}, sum -> sum + x * y end)
    |> abs()
    |> rem(10)
  end

  defp parse(input) do
    input
    |> String.graphemes()
    |> Enum.map(&String.to_integer/1)
  end
end
