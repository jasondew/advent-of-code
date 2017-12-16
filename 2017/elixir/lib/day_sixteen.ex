defmodule DaySixteen do
  @doc ~S"""
    Day 16 solver

    iex> DaySixteen.part_one("abcde", "s1,x3/4,pe/b")
    "baedc"
  """

  def part_one(programs, input) do
    steps = parse(input)
    dance(programs, steps)
  end

  def part_two(programs, input) do
    steps = parse(input)
    initial_state = programs
    Enum.reduce(1..1000000000, initial_state, fn(_, programs) ->
      dance(programs, steps)
    end)
  end

  defp dance(programs, steps) do
    Enum.reduce(steps, programs, fn(step, programs) ->
      dance_step(programs, step)
    end)
  end

  defp dance_step(programs, "s" <> size_string) do
    size = String.length(programs)
    count = String.to_integer(size_string)

    String.slice(programs, (size - count)..size) <>
      String.slice(programs, 0..(size - count - 1))
  end

  defp dance_step(programs, "x" <> indexes) do
    [index_a, index_b] = String.split(indexes, "/") |> Enum.map(&String.to_integer/1)
    {first_index, last_index} = Enum.min_max([index_a, index_b])

    first_part = if first_index == 0 do
      ""
    else
      String.slice(programs, 0..(first_index - 1))
    end

    first_part <>
      String.at(programs, last_index) <>
      String.slice(programs, (first_index + 1)..(last_index - 1)) <>
      String.at(programs, first_index) <>
      String.slice(programs, (last_index + 1)..(String.length(programs)))
  end

  defp dance_step(programs, "p" <> program_names) do
    [program_a, program_b] = String.split(program_names, "/")
    updated_programs = String.replace(programs, program_a, "x")
    updated_programs = String.replace(updated_programs, program_b, program_a)
    String.replace(updated_programs, "x", program_b)
  end

  defp parse(input) do
    String.split(input, ",")
  end
end
