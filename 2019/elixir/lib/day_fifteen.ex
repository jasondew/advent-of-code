defmodule DayFifteen do
  @directions %{north: 1, south: 2, east: 3, west: 4}
  @doc """
    Part 1
  """
  def part_one(input) do
    input
    |> Intcode.new()
    |> steps_to_sensor(%{}, {0, 0}, 1)
    |> elem(0)
  end

  @doc """
    Part 2
  """
  def part_two(input) do
    map =
      input
      |> Intcode.new()
      |> steps_to_sensor(%{}, {0, 0}, 1)
      |> elem(1)

    sensor_position =
      map
      |> Enum.find(fn {_position, type} -> type == :sensor end)
      |> elem(0)

    max_steps(map, sensor_position)
  end

  ## PRIVATE FUNCTIONS

  defp max_steps(map, position, steps \\ 0, visited \\ []) do
    @directions
    |> Enum.map(fn {direction, _direction_code} ->
      new_position = new_position(position, direction)

      if Enum.member?(visited, new_position) do
        steps
      else
        case Map.get(map, new_position) do
          :empty ->
            max_steps(map, new_position, steps + 1, [new_position | visited])

          :wall ->
            steps

          :sensor ->
            steps
        end
      end
    end)
    |> Enum.max()
  end

  defp steps_to_sensor(configuration, map, position, steps) do
    @directions
    |> Enum.map(fn {direction, direction_code} ->
      new_position = new_position(position, direction)

      case Map.get(map, new_position) do
        nil ->
          {outputs, configuration} =
            configuration
            |> Intcode.put_inputs([direction_code])
            |> Intcode.run()
            |> Intcode.pop_outputs()

          case outputs do
            [0] ->
              {nil, Map.put(map, new_position, :wall)}

            [1] ->
              steps_to_sensor(
                configuration,
                Map.put(map, new_position, :empty),
                new_position,
                steps + 1
              )

            [2] ->
              {steps, Map.put(map, new_position, :sensor)}
          end

        _type ->
          {nil, map}
      end
    end)
    |> Enum.reduce(fn {steps, map}, {min_steps, merged_map} ->
      {
        cond do
          is_nil(steps) && is_nil(min_steps) -> nil
          is_nil(min_steps) -> steps
          is_nil(steps) -> min_steps
          steps < min_steps -> steps
          true -> min_steps
        end,
        Map.merge(merged_map, map, fn _key, _value, value -> value end)
      }
    end)
  end

  defp new_position({x, y}, :north), do: {x, y + 1}
  defp new_position({x, y}, :south), do: {x, y - 1}
  defp new_position({x, y}, :east), do: {x + 1, y}
  defp new_position({x, y}, :west), do: {x - 1, y}
end
