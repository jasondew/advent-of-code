defmodule DayNineteen do
  @doc """
  Part 1
  """
  def part_one(input, field_size \\ 50) do
    configuration = Intcode.new(input)

    for(x <- 1..field_size, y <- 1..field_size, do: {x - 1, y - 1})
    |> Task.async_stream(&is_affected(configuration, &1))
    |> Enum.count(&(&1 == {:ok, true}))
  end

  defp is_affected(configuration, {x, y}) do
    configuration
    |> Intcode.put_inputs([x, y])
    |> Intcode.run()
    |> Intcode.pop_outputs()
    |> elem(0)
    |> case do
      [0] -> false
      [1] -> true
      other -> raise other
    end
  end
end
