defmodule DayEight do
  @doc """
    Part 1

    iex> DayEight.part_one("123456789012", 3, 2)
    1
  """
  def part_one(input, width, height) do
    input
    |> parse(width, height)
    |> checksum()
  end

  @doc """
    Part 2

    iex> DayEight.part_two("0222112222120000", 2, 2)
    [[0, 1], [1, 0]]
  """
  def part_two(input, width, height) do
    input
    |> parse(width, height)
    |> flatten()
  end

  ## PRIVATE FUNCTIONS ##

  defp flatten(image) do
    image
  end

  defp checksum(image) do
    layer = Enum.min_by(image, &count(&1, 0))

    count(layer, 1) * count(layer, 2)
  end

  defp count(layer, value) do
    layer
    |> Enum.map(fn row -> Enum.count(row, &(&1 == value)) end)
    |> Enum.sum()
  end

  defp parse(input, width, height) do
    input
    |> String.graphemes()
    |> Enum.map(&String.to_integer/1)
    |> Enum.chunk_every(width)
    |> Enum.chunk_every(height)
  end
end
