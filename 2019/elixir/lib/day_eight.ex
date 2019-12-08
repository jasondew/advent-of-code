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
    Enum.reduce(image, &composite/2)
  end

  defp composite(layer, image) do
    image
    |> Enum.zip(layer)
    |> Enum.map(fn {front_row, back_row} ->
      front_row
      |> Enum.zip(back_row)
      |> Enum.map(fn {front, back} -> composite_pixel(front, back) end)
    end)
  end

  defp composite_pixel(0, _back), do: 0
  defp composite_pixel(1, _back), do: 1
  defp composite_pixel(2, back), do: back
  defp composite_pixel(pixel, _), do: raise("invalid pixel: #{pixel}")

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
