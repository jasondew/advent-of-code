defmodule DayTwo do
  @doc """
    Part 1
  """

  def part_one(input) do
    run(input, 12, 2)
  end

  @doc """
    Part 2
  """

  def part_two(input) do
    all_nouns_and_verbs =
      for noun <- 0..99, verb <- 0..99 do
        {noun, verb}
      end

    {noun, verb} =
      Enum.find(all_nouns_and_verbs, fn {noun, verb} ->
        run(input, noun, verb) == 19_690_720
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
    |> step(0)
    |> List.first()
  end

  @doc """
    iex> DayTwo.step([1,9,10,3,2,3,11,0,99,30,40,50], 0)
    [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
  """

  def step(input, position) do
    [operation, arg1_position, arg2_position, result_position] = Enum.slice(input, position, 4)
    arg1 = Enum.at(input, arg1_position)
    arg2 = Enum.at(input, arg2_position)

    case operation do
      1 ->
        input
        |> List.replace_at(result_position, arg1 + arg2)
        |> step(position + 4)

      2 ->
        input
        |> List.replace_at(result_position, arg1 * arg2)
        |> step(position + 4)

      99 ->
        input
    end
  end
end
