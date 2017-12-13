defmodule DayThirteen do
  @doc ~S"""
    Day 13 solver
    
    Works on the example input:
      iex> DayThirteen.part_one("0: 3\n1: 2\n4: 4\n6: 4")
      24

      iex> DayThirteen.part_two("0: 3\n1: 2\n4: 4\n6: 4")
      10
  """

  def part_one(input) do
    input
    |> parse()
    |> caught_in()
    |> Enum.reduce(0, fn [depth, range], sum -> sum + depth * range end)
  end

  def part_two(input) do
    scanners = parse(input)
    Enum.find(0..10_000_000, fn(delay) -> ! caught?(scanners, delay) end)
  end

  defp caught?(scanners, delay) do
    Enum.find(scanners, fn([depth, range]) -> position(depth + delay, range) == 0 end)
  end

  defp caught_in(scanners) do
    Enum.filter(scanners, fn [depth, range] -> position(depth, range) == 0 end)
  end

  defp parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn line ->
         line |> String.split(": ") |> Enum.map(&String.to_integer/1)
       end)
  end

  defp position(time, range) do
    modulus = range - 1
    x = rem(time, modulus * 2)

    if x < modulus do
      x
    else
      modulus * 2 - x
    end
  end
end
