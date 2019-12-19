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

    #iex> DaySixteen.part_two("02935109699940807407585447034323")
    78725270

    #iex> DaySixteen.part_two("03081770884921959731165446850517")
    53553731
  """
  def part_two(input, phases \\ 100) do
    start = input |> String.slice(0, 7) |> String.to_integer()
    stop = String.length(input) * 10_000 - start
    digits = input |> parse() |> digits(start, stop)

    1..phases
    |> Enum.reduce(digits, fn _, digits ->
      digits
      |> Enum.reverse()
      |> Enum.reduce({[], nil}, fn digit, {digits, last_digit} ->
        if last_digit do
          next_digit = abs(digit + last_digit) |> rem(10)
          {[next_digit | digits], next_digit}
        else
          {[digit], digit}
        end
      end)
      |> elem(0)
    end)
    |> Enum.take(8)
    |> Integer.undigits()
  end

  @doc """
    iex> DaySixteen.digits(1..8, 7, 3)
    [8, 1, 2]

    iex> DaySixteen.digits(1..3, 2, 5)
    [3, 1, 2, 3, 1]
  """
  def digits(seed, from, amount) do
    from = rem(from, Enum.count(seed))
    slice = Enum.slice(seed, from, amount)
    amount = amount - Enum.count(slice)

    if amount > 0 do
      slice ++ digits(seed, 0, amount)
    else
      slice
    end
  end

  @doc """
    iex> DaySixteen.coefficient(0, 0)
    1

    iex> DaySixteen.coefficient(1, 6)
    -1

    iex> DaySixteen.coefficient(2, 11)
    0

    iex> DaySixteen.coefficient(3, 18)
    0
  """
  def coefficient(digit, index) do
    {modulo, map} = pattern(digit)
    index = rem(index, modulo)

    Enum.reduce_while(map, 0, fn {starts_at, value}, acc ->
      if starts_at > index do
        {:halt, acc}
      else
        {:cont, value}
      end
    end)
  end

  @doc """
    digit 0: 1 0 -1 0
    iex> DaySixteen.pattern(0)
    {4, %{0 => 1, 1 => 0, 2 => -1, 3 => 0}}

    digit 1: 0 1 1 0 0 -1 -1 0 0
    iex> DaySixteen.pattern(1)
    {9, %{0 => 0, 1 => 1, 3 => 0, 5 => -1, 7 => 0}}

    digit 2: 0 0 1 1 1 0 0 0 -1 -1 -1 0 0 0
    iex> DaySixteen.pattern(2)
    {14, %{0 => 0, 2 => 1, 5 => 0, 8 => -1, 11 => 0}}

    digit 3: 0 0 0 1 1 1 1 0 0 0 0 -1 -1 -1 -1 0 0 0 0
    iex> DaySixteen.pattern(3)
    {19, %{0 => 0, 3 => 1, 7 => 0, 11 => -1, 15 => 0}}
  """
  def pattern(0) do
    {4, %{0 => 1, 1 => 0, 2 => -1, 3 => 0}}
  end

  def pattern(digit) do
    {
      (digit + 1) * 5 - 1,
      %{
        0 => 0,
        digit => 1,
        (2 * digit + 1) => 0,
        (3 * digit + 2) => -1,
        (4 * digit + 3) => 0
      }
    }
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
