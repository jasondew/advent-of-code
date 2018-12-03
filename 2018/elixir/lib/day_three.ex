defmodule ElixirSolutions.DayThree do
  defmodule Rectangle do
    defstruct [:id, :top, :left, :width, :height]

    def x_range(rectangle), do: rectangle.left..Rectangle.right(rectangle)
    def y_range(rectangle), do: rectangle.top..Rectangle.bottom(rectangle)
    def right(rectangle), do: rectangle.left + rectangle.width - 1
    def bottom(rectangle), do: rectangle.top + rectangle.height - 1
  end

  defimpl String.Chars, for: Rectangle do
    def to_string(rectangle) do
      "[#{rectangle.id}] " <>
        "x: (#{rectangle.left}..#{Rectangle.right(rectangle)}) " <>
        "y: (#{rectangle.top}..#{Rectangle.bottom(rectangle)})"
    end
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

  @doc ~S"""
    Part 2

    iex> ElixirSolutions.DayThree.part_two("#1 @ 1,3: 4x4\n2 @ 3,1: 4x4\n3 @ 5,5: 2x2")
    3

    iex> ElixirSolutions.DayThree.part_two("#828 @ 305,748: 27x10\n140 @ 314,738: 17x23")
    nil
  """

  def part_two(input) do
    rectangles =
      input
      |> String.split("\n")
      |> Enum.map(&decode_rectangle/1)

    Enum.find_value(rectangles, fn rectangle ->
      if Enum.all?(rectangles, fn other_rectangle ->
           rectangle.id == other_rectangle.id or not overlapping?(rectangle, other_rectangle)
         end) do
        rectangle.id
      end
    end)
  end

  defp overlapping?(rect1, rect2) do
    overlapping_ranges?(Rectangle.x_range(rect1), Rectangle.x_range(rect2)) and
      overlapping_ranges?(Rectangle.y_range(rect1), Rectangle.y_range(rect2))
  end

  defp overlapping_ranges?(x1..y1 = range1, x2..y2 = range2) do
    x1 in range2 or y1 in range2 or x2 in range1 or y2 in range1
  end
end
