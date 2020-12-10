defmodule DayNine do
  @doc ~S[
    iex> DayNine.part_one("""
    ...> 35
    ...> 20
    ...> 15
    ...> 25
    ...> 47
    ...> 40
    ...> 62
    ...> 55
    ...> 65
    ...> 95
    ...> 102
    ...> 117
    ...> 150
    ...> 182
    ...> 127
    ...> 219
    ...> 299
    ...> 277
    ...> 309
    ...> 576
    ...> """, 5)
    127
  ]
  def part_one(input, window \\ 25) do
    values = parse(input)

    {first_group, rest} = Enum.split(values, window)
    queue = :queue.from_list(first_group)

    Enum.reduce_while(rest, queue, fn value, queue ->
      if any_pair_sums_to(queue, value) do
        queue = :queue.drop(queue)

        {:cont, :queue.in(value, queue)}
      else
        {:halt, value}
      end
    end)
  end

  @doc ~S[
    iex> DayNine.part_two("""
    ...> 35
    ...> 20
    ...> 15
    ...> 25
    ...> 47
    ...> 40
    ...> 62
    ...> 55
    ...> 65
    ...> 95
    ...> 102
    ...> 117
    ...> 150
    ...> 182
    ...> 127
    ...> 219
    ...> 299
    ...> 277
    ...> 309
    ...> 576
    ...> """, 127)
    62
  ]
  def part_two(input, target \\ 1_930_745_883) do
    [first_value | rest] = parse(input)
    queue = :queue.from_list([first_value])

    queue =
      Enum.reduce_while(rest, queue, fn candidate, queue ->
        if candidate > target do
          {:halt, nil}
        else
          queue = :queue.in(candidate, queue)
          sum = queue_sum(queue)

          cond do
            sum == target ->
              {:halt, queue}

            sum > target ->
              queue = drop_while_greater_than(queue, target)

              if queue_sum(queue) == target do
                {:halt, queue}
              else
                {:cont, :queue.drop(queue)}
              end

            sum < target ->
              {:cont, queue}
          end
        end
      end)

    {min, max} = queue |> :queue.to_list() |> Enum.min_max()
    min + max
  end

  ## PRIVATE FUNCTIONS

  defp drop_while_greater_than(queue, target) do
    if queue_sum(queue) > target do
      drop_while_greater_than(:queue.drop(queue), target)
    else
      queue
    end
  end

  def queue_sum(queue), do: Enum.sum(:queue.to_list(queue))

  defp any_pair_sums_to(queue, target) do
    queue
    |> :queue.to_list()
    |> Enum.sort()
    |> search(target)
  end

  defp search([_value], _target), do: nil

  defp search([value | values], target) do
    Enum.reduce_while(values, nil, fn other_value, _found ->
      sum = value + other_value

      cond do
        sum == target -> {:halt, true}
        sum > target -> {:halt, nil}
        sum < target -> {:cont, nil}
      end
    end) || search(values, target)
  end

  defp parse(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.to_integer/1)
  end
end
