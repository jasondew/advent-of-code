defmodule DaySix do
  @doc """
    iex> DaySix.part_one("abc\\n\\na\\nb\\nc\\n\\nab\\nac\\n\\na\\na\\na\\na\\n\\nb\\n")
    11
  """
  def part_one(input) do
    input
    |> String.split("\n\n")
    |> Enum.map(fn group ->
      group
      |> String.split("\n")
      |> Enum.join("")
      |> String.graphemes()
      |> MapSet.new()
      |> MapSet.size()
    end)
    |> Enum.sum()
  end

  @doc """
    iex> DaySix.part_two("abc\\n\\na\\nb\\nc\\n\\nab\\nac\\n\\na\\na\\na\\na\\n\\nb\\n")
    6
  """
  def part_two(input) do
    input
    |> String.trim()
    |> String.split("\n\n")
    |> Enum.map(fn group_string ->
      group = String.split(group_string, "\n")
      group_count = Enum.count(group)

      Enum.reduce(group, %{}, fn person, map ->
        person
        |> String.graphemes()
        |> Enum.reduce(map, fn question, map ->
          Map.update(map, question, 1, &(&1 + 1))
        end)
      end)
      |> Enum.count(fn {_question, answer_count} -> answer_count == group_count end)
    end)
    |> Enum.sum()
  end
end
