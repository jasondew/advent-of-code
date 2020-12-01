defmodule DayOne do
  @doc """
    Part 1

    iex> DayOne.part_one("1721\\n979\\n366\\n299\\n675\\n1456")
    514579
  """

  def part_one(input, target \\ 2020) do
    {:ok, a, b} =
      input
      |> to_integers()
      |> find_pair_summing_to(target)

    a * b
  end

  @doc """
    Part 2

    iex> DayOne.part_two("1721\\n979\\n366\\n299\\n675\\n1456")
    241861950
  """

  def part_two(input, target \\ 2020) do
    {:ok, a, b, c} =
      input
      |> to_integers()
      |> find_triple_summing_to(target)

    a * b * c
  end

  ## PRIVATE FUNCTIONS ##

  defp find_pair_summing_to([], _target), do: {:error, :not_found}

  defp find_pair_summing_to([value | rest], target) do
    case Enum.find(rest, &(value + &1 == target)) do
      nil ->
        find_pair_summing_to(rest, target)

      other_value ->
        {:ok, value, other_value}
    end
  end

  defp find_triple_summing_to([], _target), do: {:error, :not_found}
  defp find_triple_summing_to([_ | []], _target), do: {:error, :not_found}
  defp find_triple_summing_to([_, _ | []], _target), do: {:error, :not_found}

  defp find_triple_summing_to([value | rest], target) do
    case find_pair_summing_to(rest, target - value) do
      {:ok, a, b} -> {:ok, value, a, b}
      {:error, :not_found} -> find_triple_summing_to(rest, target)
    end
  end

  defp to_integers(input, delimiter \\ "\n") do
    input
    |> String.split(delimiter)
    |> Enum.map(&String.to_integer/1)
  end
end
