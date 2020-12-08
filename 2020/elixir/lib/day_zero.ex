defmodule DayZero do
  @doc """
    iex> DayZero.part_one("abc\\ndef\\n")
    2
  """
  def part_one(input) do
    input
    |> parse()
    |> Enum.count()
  end

  @doc """
    iex> DayZero.part_two("abc\\n")
    1
  """
  def part_two(input) do
    input
    |> parse()
    |> Enum.count()
  end

  ## PRIVATE FUNCTIONS

  defp parse(input) do
    input
    |> String.trim()
    |> String.split("\n")
  end
end
