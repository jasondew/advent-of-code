defmodule ElixirSolutions.DayThree do
  defmodule Rectangle do
    defstruct [:id, :top, :left, :width, :height]
  end

  @doc ~S"""
    Part 1

    #1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2

    iex> ElixirSolutions.DayThree.part_one("#1 @ 1,3: 4x4\n2 @ 3,1: 4x4\n3 @ 5,5: 2x2")
    4
  """

  def part_one(input) do
    input
    |> String.split("\n")
    |> Enum.reduce(%{}, fn encoded_rectangle, sparse_square ->
      rectangle = decode_rectangle(encoded_rectangle)
      x_span = rectangle.left..(rectangle.left + rectangle.width - 1)
      y_span = rectangle.top..(rectangle.top + rectangle.height - 1)

      Enum.reduce(y_span, sparse_square, fn y, ys ->
        Enum.reduce(x_span, ys, fn x, xs ->
          Map.update(xs, {x, y}, 1, &(&1 + 1))
        end)
      end)
    end)
    |> Enum.count(fn {_, count} -> count > 1 end)
  end

  defp decode_rectangle(encoded_rectangle) do
    encoded_rectangle
    |> String.split(["#", " @ ", ",", ": ", "x"], trim: true)
    |> Enum.map(&String.to_integer/1)
    |> decode_rectangle_parts
  end

  defp decode_rectangle_parts([id, left, top, width, height]) do
    %Rectangle{id: id, top: top, left: left, width: width, height: height}
  end
end
