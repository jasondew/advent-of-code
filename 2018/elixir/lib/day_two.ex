defmodule ElixirSolutions.DayTwo do
  @doc ~S"""
    Part 1

    abcdef contains no letters that appear exactly two or three times.
    bababc contains two a and three b, so it counts for both.
    abbcde contains two b, but no letter appears exactly three times.
    abcccd contains three c, but no letter appears exactly two times.
    aabcdd contains two a and two d, but it only counts once.
    abcdee contains two e.
    ababab contains three a and three b, but it only counts once.

      iex> ElixirSolutions.DayTwo.part_one("abcdef bababc abbcde abcccd aabcdd abcdee ababab")
      12
  """

  def part_one(input) do
    ids = String.split(input)

    counts =
      List.foldl(ids, %{pairs: 0, triples: 0}, fn id, totals ->
        with counts = letter_counts(id),
             updated_pair_count =
               if(
                 Enum.any?(counts, fn {_, count} -> count == 2 end),
                 do: totals.pairs + 1,
                 else: totals.pairs
               ),
             updated_triple_count =
               if(
                 Enum.any?(counts, fn {_, count} -> count == 3 end),
                 do: totals.triples + 1,
                 else: totals.triples
               ) do
          %{pairs: updated_pair_count, triples: updated_triple_count}
        end
      end)

    counts[:pairs] * counts[:triples]
  end

  defp letter_counts(string) do
    string
    |> String.graphemes()
    |> Enum.group_by(& &1)
    |> Enum.reduce(%{}, fn {letter, group}, map ->
      put_in(map[letter], Enum.count(group))
    end)
  end

  @doc ~S"""
    Part 2

    Finds the common characters in the pair of IDs that differ by exactly one character

    iex> ElixirSolutions.DayTwo.part_two("abcde fghij klmno pqrst fguij axcye wvxyz")
    "fgij"
  """

  def part_two(input) do
    ids = String.split(input)

    pair =
      Enum.find_value(ids, fn id ->
        Enum.find_value(ids, fn other_id ->
          if hamming_distance(id, other_id) == 1 do
            {id, other_id}
          else
            nil
          end
        end)
      end)

    case pair do
      {a, b} -> remove_mismatched_characters(a, b)
      nil -> nil
    end
  end

  defp hamming_distance(string, other_string) do
    String.graphemes(string)
    |> Enum.zip(String.graphemes(other_string))
    |> Enum.count(fn {a, b} -> a != b end)
  end

  defp remove_mismatched_characters(string, other_string) do
    String.graphemes(string)
    |> Enum.zip(String.graphemes(other_string))
    |> Enum.filter(fn {a, b} -> a == b end)
    |> Enum.map(fn {a, _} -> a end)
    |> List.to_string()
  end
end
