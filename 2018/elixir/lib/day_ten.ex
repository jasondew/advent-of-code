defmodule ElixirSolutions.DayTen do
  defmodule Point do
    defstruct [:x, :y, :vx, :vy]

    def step(point, times \\ 1) do
      %{point | x: point.x + times * point.vx, y: point.y + times * point.vy}
    end
  end

  @doc ~S"""
    Part 1

    iex> ElixirSolutions.DayTen.parts_one_and_two("position=< 9,  1> velocity=< 0,  2>\n position=< 7,  0> velocity=<-1,  0>\n position=< 3, -2> velocity=<-1,  1>\n position=< 6, 10> velocity=<-2, -1>\n position=< 2, -4> velocity=< 2,  2>\n position=<-6, 10> velocity=< 2, -2>\n position=< 1,  8> velocity=< 1, -1>\n position=< 1,  7> velocity=< 1,  0>\n position=<-3, 11> velocity=< 1, -2>\n position=< 7,  6> velocity=<-1, -1>\n position=<-2,  3> velocity=< 1,  0>\n position=<-4,  3> velocity=< 2,  0>\n position=<10, -3> velocity=<-1,  1>\n position=< 5, 11> velocity=< 1, -2>\n position=< 4,  7> velocity=< 0, -1>\n position=< 8, -2> velocity=< 0,  1>\n position=<15,  0> velocity=<-2,  0>\n position=< 1,  6> velocity=< 1,  0>\n position=< 8,  9> velocity=< 0, -1>\n position=< 3,  3> velocity=<-1,  1>\n position=< 0,  5> velocity=< 0, -1>\n position=<-2,  2> velocity=< 2,  0>\n position=< 5, -2> velocity=< 1,  2>\n position=< 1,  4> velocity=< 2,  1>\n position=<-2,  7> velocity=< 2, -2>\n position=< 3,  6> velocity=<-1, -1>\n position=< 5,  0> velocity=< 1,  0>\n position=<-6,  0> velocity=< 2,  0>\n position=< 5,  9> velocity=< 1, -2>\n position=<14,  7> velocity=<-2,  0>\n position=<-3,  6> velocity=< 2, -1>")
    %{min_generation: 3, min_area: 63}
  """
  def parts_one_and_two(input, max_generations \\ 10, print_min \\ false) do
    points = parse(input)

    {min_generation, min_area_points, min_area} =
      Enum.reduce(1..max_generations, {0, points, 100_000_000_000}, fn generation,
                                                                       {_min_generation,
                                                                        _min_points,
                                                                        min_area} = triplet ->
        points = Enum.map(points, &Point.step(&1, generation))
        area = plot_area(points)

        if area < min_area do
          {generation, points, area}
        else
          triplet
        end
      end)

    if print_min do
      plot(min_area_points)
    end

    %{min_generation: min_generation, min_area: min_area}
  end

  defp plot_area(points) do
    {x_min..x_max, y_min..y_max} = range(points)
    (x_max - x_min) * (y_max - y_min)
  end

  defp range(points) do
    x_range = points |> Enum.min_max_by(& &1.x) |> (fn {p1, p2} -> p1.x..p2.x end).()
    y_range = points |> Enum.min_max_by(& &1.y) |> (fn {p1, p2} -> p1.y..p2.y end).()

    {x_range, y_range}
  end

  defp plot(points) do
    {x_range, y_range} = range(points)

    Enum.each(y_range, fn y ->
      Enum.each(x_range, fn x ->
        IO.write(if Enum.any?(points, &(&1.x == x and &1.y == y)), do: "#", else: " ")
      end)

      IO.puts("")
    end)
  end

  # format:
  # "position=< 40458,  50379> velocity=<-4, -5>"
  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&parse_point/1)
  end

  defp parse_point(string) do
    [_position, x, y, _velocity, vx, vy] =
      String.split(string, [" ", "=", "<", ">", ","], trim: true)

    %Point{
      x: String.to_integer(x),
      y: String.to_integer(y),
      vx: String.to_integer(vx),
      vy: String.to_integer(vy)
    }
  end
end
