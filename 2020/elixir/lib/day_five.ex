defmodule DayFive do
  def part_one(input) do
    input
    |> String.split("\n")
    |> Enum.map(&seat_id/1)
    |> Enum.max()
  end

  def part_two(input) do
    seat_ids =
      input
      |> String.split("\n")
      |> Enum.map(&seat_id/1)

    [prev, _next] =
      seat_ids
      |> Enum.sort()
      |> Enum.chunk_every(2, 1, :discard)
      |> Enum.find(fn [prev, next] -> next - prev > 1 end)

    prev + 1
  end

  @doc """
    Seat ID calculation

    iex> DayFive.seat_id("FBFBBFFRLR")
    357

    iex> DayFive.seat_id("BFFFBBFRRR")
    567

    iex> DayFive.seat_id("FFFBBBFRRR")
    119

    iex> DayFive.seat_id("BBFFBBFRLL")
    820
  """
  def seat_id(code) do
    code
    |> String.graphemes()
    |> Enum.map(fn
      "F" -> "0"
      "B" -> "1"
      "L" -> "0"
      "R" -> "1"
    end)
    |> Enum.join()
    |> String.to_integer(2)
  end

  ## PRIVATE FUNCTIONS
end
