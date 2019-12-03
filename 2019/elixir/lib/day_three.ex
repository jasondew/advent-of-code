defmodule DayThree do
  defmodule Point do
    defstruct ~w[x y]a

    def inspect(%Point{x: x, y: y}) do
      "(#{x}, #{y})"
    end

    def origin(), do: %__MODULE__{x: 0, y: 0}

    def add(%Point{x: x, y: y}, {:U, magnitude}),
      do: %Point{x: x, y: y - magnitude}

    def add(%Point{x: x, y: y}, {:D, magnitude}),
      do: %Point{x: x, y: y + magnitude}

    def add(%Point{x: x, y: y}, {:L, magnitude}),
      do: %Point{x: x - magnitude, y: y}

    def add(%Point{x: x, y: y}, {:R, magnitude}),
      do: %Point{x: x + magnitude, y: y}

    def manhattan_distance(%Point{x: x1, y: y1}, %Point{x: x2, y: y2}) do
      abs(x2 - x1) + abs(y2 - y1)
    end
  end

  defmodule LineSegment do
    defstruct ~w[start stop]a

    def new(position, vector) do
      %__MODULE__{start: position, stop: Point.add(position, vector)}
    end

    def inspect(%LineSegment{start: start, stop: stop}) do
      "#{Point.inspect(start)} <-> #{Point.inspect(stop)}"
    end

    def length(%LineSegment{
          start: %Point{x: ax, y: ay},
          stop: %Point{x: bx, y: by}
        }) do
      abs(ax - bx) + abs(ay - by)
    end

    def contains(
          %LineSegment{start: %Point{x: ax, y: ay}, stop: %Point{x: bx, y: by}},
          %Point{x: x, y: y}
        ) do
      x >= min(ax, bx) and x <= max(ax, bx) and
        y >= min(ay, by) and y <= max(ay, by)
    end

    def distance_to(%LineSegment{start: %Point{x: x, y: ay}}, %Point{x: x, y: y}) do
      abs(ay - y)
    end

    def distance_to(%LineSegment{start: %Point{x: ax, y: y}}, %Point{x: x, y: y}) do
      abs(ax - x)
    end

    def equation(%LineSegment{start: %Point{x: ax}, stop: %Point{x: ax}}) do
      {:vertical, ax}
    end

    def equation(%LineSegment{start: %Point{y: ay}, stop: %Point{y: ay}}) do
      {:horizontal, ay}
    end

    def equation(%LineSegment{
          start: %Point{x: ax, y: ay},
          stop: %Point{x: bx, y: by}
        }) do
      m = (by - ay) / (bx - ax)
      b = by - m * bx
      {m, b}
    end
  end

  @doc """
    Part 1

    iex> DayThree.part_one("R8,U5,L5,D3\\nU7,R6,D4,L4")
    6

    iex> DayThree.part_one("R75,D30,R83,U83,L12,D49,R71,U7,L72\\nU62,R66,U55,R34,D71,R55,D58,R83")
    159

    iex> DayThree.part_one("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
    135
  """
  def part_one(input) do
    [line_one_path, line_two_path] =
      parse_input(input) |> Enum.map(&trace_path/1)

    line_one_path
    |> find_intersections(line_two_path)
    |> Enum.map(fn point -> Point.manhattan_distance(point, Point.origin()) end)
    |> Enum.min()
  end

  @doc """
    Part 2

    iex> DayThree.part_two("R8,U5,L5,D3\\nU7,R6,D4,L4")
    30

    iex> DayThree.part_two("R75,D30,R83,U83,L12,D49,R71,U7,L72\\nU62,R66,U55,R34,D71,R55,D58,R83")
    610

    iex> DayThree.part_two("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
    410
  """
  def part_two(input) do
    [line_one_path, line_two_path] =
      parse_input(input) |> Enum.map(&trace_path/1)

    line_one_path
    |> find_intersections(line_two_path)
    |> Enum.map(fn point ->
      signal_distance(point, line_one_path) +
        signal_distance(point, line_two_path)
    end)
    |> Enum.min()
  end

  ## PRIVATE FUNCTIONS ##

  defp signal_distance(point, line_segments) do
    {:found, total} =
      Enum.reduce(
        line_segments,
        {:looking, 0},
        fn
          line_segment, {:looking, total} ->
            if LineSegment.contains(line_segment, point) do
              {:found, total + LineSegment.distance_to(line_segment, point)}
            else
              {:looking, total + LineSegment.length(line_segment)}
            end

          _, {:found, total} ->
            {:found, total}
        end
      )

    total
  end

  defp trace_path(vectors) do
    {_final_position, path} =
      vectors
      |> Enum.reduce({Point.origin(), []}, fn vector, {position, path} ->
        next_segment = LineSegment.new(position, vector)
        {next_segment.stop, [next_segment | path]}
      end)

    Enum.reverse(path)
  end

  defp find_intersections(line_one_path, [_ | line_two_path]) do
    for line_segment_one <- line_one_path, line_segment_two <- line_two_path do
      find_intersection(line_segment_one, line_segment_two)
    end
    |> Enum.reject(&Kernel.is_nil/1)
  end

  defp find_intersection(line_segment_one, line_segment_two) do
    equation_one = LineSegment.equation(line_segment_one)
    equation_two = LineSegment.equation(line_segment_two)

    intersection =
      case {equation_one, equation_two} do
        {{:vertical, x}, {:vertical, x}} ->
          %Point{x: x, y: minimum_y(line_segment_one, line_segment_two)}

        {{:vertical, _}, {:vertical, _}} ->
          nil

        {{:vertical, x}, {:horizontal, y}} ->
          %Point{x: x, y: y}

        {{:vertical, x}, {m, b}} ->
          y = m * x + b
          %Point{x: x, y: y}

        {{:horizontal, y}, {:horizontal, y}} ->
          %Point{x: minimum_x(line_segment_one, line_segment_two), y: y}

        {{:horizontal, _}, {:horizontal, _}} ->
          nil

        {{:horizontal, y}, {:vertical, x}} ->
          %Point{x: x, y: y}

        {{:horizontal, y}, {m, b}} ->
          x = (y - b) / m
          %Point{x: x, y: y}

        {{m, b}, {m, b}} ->
          %Point{x: 0, y: b}

        {{m1, b1}, {m2, b2}} ->
          x = (b2 - b1) / (m1 - m2)
          y = m1 * x + b1
          %Point{x: x, y: y}
      end

    if in_bounds(intersection, line_segment_one) and
         in_bounds(intersection, line_segment_two) do
      intersection
    end
  end

  defp in_bounds(nil, _), do: false

  defp in_bounds(%Point{x: x, y: y}, %LineSegment{
         start: %Point{x: ax, y: ay},
         stop: %Point{x: bx, y: by}
       }) do
    min_x = min(ax, bx)
    max_x = max(ax, bx)

    min_y = min(ay, by)
    max_y = max(ay, by)

    x >= min_x and x <= max_x and
      y >= min_y and y <= max_y
  end

  defp minimum_y(
         %LineSegment{start: %Point{y: ay1}, stop: %Point{y: ay2}},
         %LineSegment{start: %Point{y: by1}, stop: %Point{y: by2}}
       ) do
    min(min(ay1, ay2), min(by1, by2))
  end

  defp minimum_x(
         %LineSegment{start: %Point{x: ax1}, stop: %Point{x: ax2}},
         %LineSegment{start: %Point{x: bx1}, stop: %Point{x: bx2}}
       ) do
    min(min(ax1, ax2), min(bx1, bx2))
  end

  def parse_input(input) do
    input
    |> String.split("\n")
    |> Enum.map(&to_vectors/1)
  end

  defp to_vectors(input) do
    input
    |> String.split(",")
    |> Enum.map(&to_vector/1)
  end

  defp to_vector(<<direction::binary-size(1)>> <> magnitude) do
    {String.to_atom(direction), String.to_integer(magnitude)}
  end
end
