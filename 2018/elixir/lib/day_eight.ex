defmodule ElixirSolutions.DayEight do
  defmodule Node do
    defstruct [:name, :children, :metadata]
  end

  @doc ~S"""
    Part 1
    iex> ElixirSolutions.DayEight.part_one("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")
    138
  """
  def part_one(input) do
    input |> parse() |> all_metadata() |> Enum.sum()
  end

  defp all_metadata(node) do
    Enum.concat(node.metadata, Enum.flat_map(node.children, &all_metadata(&1)))
  end

  @doc ~S"""
    Part 2
    iex> ElixirSolutions.DayEight.part_two("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")
    66
  """
  def part_two(input) do
    input |> parse() |> value()
  end

  defp value(nil), do: 0

  defp value(%Node{children: [], metadata: metadata}), do: Enum.sum(metadata)

  defp value(%Node{children: children, metadata: metadata}) do
    Enum.reduce(metadata, 0, fn child_number, total ->
      total + value(Enum.at(children, child_number - 1))
    end)
  end

  defp parse(input) do
    {root, []} =
      input
      |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> parse_node(?A)

    root
  end

  defp parse_node(data, name) do
    [child_count, metadata_count | remaining_data] = data

    {children, remaining_data} =
      if child_count > 0 do
        Enum.reduce(1..child_count, {[], remaining_data}, fn count, {children, remaining_data} ->
          {node, remaining_data} = parse_node(remaining_data, name + count)
          {[node | children], remaining_data}
        end)
      else
        {[], remaining_data}
      end

    {metadata, remaining_data} = Enum.split(remaining_data, metadata_count)

    node = %Node{
      name: List.to_string([name]),
      children: Enum.reverse(children),
      metadata: metadata
    }

    {node, remaining_data}
  end
end
