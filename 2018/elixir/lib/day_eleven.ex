defmodule ElixirSolutions.DayEleven do
  @doc ~s"""
    Part 1

    iex> ElixirSolutions.DayEleven.part_one("18")
    {33, 45}

    iex> ElixirSolutions.DayEleven.part_one("42")
    {21, 61}
  """
  def part_one(input) do
    serial_number = String.to_integer(input)

    grid =
      for x <- 1..300,
          y <- 1..300,
          into: %{},
          do: {{x, y}, power_level(x, y, serial_number)}

    squares =
      for x <- 1..298,
          y <- 1..298,
          into: %{},
          do: {{x, y}, square_power_level(grid, x, y)}

    {max_coordinate, _max_power_level} = Enum.max_by(squares, &elem(&1, 1))

    max_coordinate
  end

  @doc ~s"""
  Part 2

  """
  # iex> ElixirSolutions.DayEleven.part_two("18")
  # {90, 269, 16}
  #    iex> ElixirSolutions.DayEleven.part_two("42")
  #    {232, 251, 12}
  #  """
  def part_two(input) do
    grid_size = 300
    serial_number = String.to_integer(input)

    grid =
      for x <- 1..grid_size,
          y <- 1..grid_size,
          into: %{},
          do: {{x, y}, power_level(x, y, serial_number)}

    single_squares =
      for {coordinate, power_level} <- grid, into: %{}, do: {coordinate, power_level}

    # only need to keep previous size for each coordinate
    # can keep running maximum inside inner loop

    Enum.reduce(2..grid_size, {single_squares, {{0, 0, 0}, 0}}, fn size, {squares, maxes} ->
      Enum.reduce(1..(grid_size - size + 1), {squares, maxes}, fn x, {squares, maxes} ->
        Enum.reduce(1..(grid_size - size + 1), {squares, maxes}, fn y, {squares, maxes} ->
          {_max_location_size, max_power_level} = maxes
          previous_size_power_level = Map.get(squares, {x, y})

          current_size_power_level =
            square_power_level(grid, previous_size_power_level, x, y, size)

          updated_squares = Map.put(squares, {x, y}, current_size_power_level)

          if current_size_power_level > max_power_level do
            {updated_squares, {{x, y, size + 1}, current_size_power_level}}
          else
            {updated_squares, maxes}
          end
        end)
      end)
      |> (fn {_, maxes} = pair ->
            IO.inspect(maxes, label: to_string(size))
            pair
          end).()
    end)
    |> elem(1)
    |> elem(0)
  end

  defp square_power_level(grid, previous_size_power_level, x, y, size) do
    # size 4 at {x, y}
    #
    #      x  x+1 x+2 x+3 x+4
    #   y  o   .   .   .   |
    # y+1  .   .   .   .   |
    # y+2  .   .   .   .   |
    # y+3  .   .   .   .   |
    # y+4  -   -   -   -   +
    #
    # {x+4, y..y+3} + {x..x+3, y+4} + {x+4, y+4}

    extra_column_power_level =
      Enum.reduce(x..(x + size - 1), 0, fn x, total -> total + Map.get(grid, {x, y + size}, 0) end)

    extra_row_power_level =
      Enum.reduce(y..(y + size - 1), 0, fn y, total -> total + Map.get(grid, {x + size, y}, 0) end)

    extra_corner_power_level = Map.get(grid, {x + size, y + size}, 0)

    previous_size_power_level + extra_column_power_level + extra_row_power_level +
      extra_corner_power_level
  end

  defp square_power_level(grid, x, y, size \\ 3) do
    Enum.reduce(x..(x + size - 1), 0, fn x, total ->
      Enum.reduce(y..(y + size - 1), total, fn y, total ->
        total + Map.get(grid, {x, y}, 0)
      end)
    end)
  end

  @doc ~s"""
    iex> ElixirSolutions.DayEleven.power_level(122, 79, 57)
    -5

    iex> ElixirSolutions.DayEleven.power_level(217, 196, 39)
    0

    iex> ElixirSolutions.DayEleven.power_level(101, 153, 71)
    4
  """
  def power_level(x, y, serial_number) do
    hundreds_digit((x + 10) * (x * y + 10 * y + serial_number)) - 5
  end

  defp hundreds_digit(number), do: number |> Integer.digits() |> Enum.reverse() |> Enum.at(2, 0)
end
