defmodule DayTwo do
  @doc """
    Part 1
  """

  def part_one(input) do
    input
    |> Conversions.to_integers(",")
    |> run(12, 2)
  end

  @doc """
    Part 2
  """

  def part_two(input) do
    intcode = Conversions.to_integers(input, ",")

    all_nouns_and_verbs =
      for noun <- 0..99, verb <- 0..99 do
        {noun, verb}
      end

    {noun, verb} =
      Enum.find(all_nouns_and_verbs, fn {noun, verb} ->
        run(intcode, noun, verb) == 19_690_720
      end)

    noun * 100 + verb
  end

  @doc """
    iex> DayTwo.run([1,9,10,3,2,3,11,0,99,30,40,50], 9, 10)
    3500
  """

  def run(input, noun, verb) do
    input
    |> List.replace_at(1, noun)
    |> List.replace_at(2, verb)
    |> IntcodeVM.run()
    |> Map.get(:intcode)
    |> List.first()
  end
end
