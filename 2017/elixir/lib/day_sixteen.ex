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

    Enum.reduce(1..1_000, initial_state, fn index, programs ->
      if rem(index, 100) == 0, do: IO.puts(".")
      dance(programs, steps)
    end)
  end

  defp dance(programs, steps) do
    Enum.reduce(steps, programs, fn step, programs ->
      step.(programs)
    end)
  end

  defp parse_step("s" <> size_string) do
    count = String.to_integer(size_string)

    fn programs ->
      size = 16

      String.slice(programs, (size - count)..size) <>
        String.slice(programs, 0..(size - count - 1))
    end
  end

  defp parse_step("x" <> indexes) do
    [index_a, index_b] = String.split(indexes, "/") |> Enum.map(&String.to_integer/1)
    {first_index, last_index} = Enum.min_max([index_a, index_b])

    fn programs ->
      first_part =
        if first_index == 0 do
          ""
        else
          String.slice(programs, 0..(first_index - 1))
        end

      first_part <>
        String.at(programs, last_index) <>
        String.slice(programs, (first_index + 1)..(last_index - 1)) <>
        String.at(programs, first_index) <>
        String.slice(programs, (last_index + 1)..String.length(programs))
    end
  end

  defp parse_step("p" <> program_names) do
    [program_a, program_b] = String.split(program_names, "/")

    fn programs ->
      updated_programs = String.replace(programs, program_a, "x")
      updated_programs = String.replace(updated_programs, program_b, program_a)
      String.replace(updated_programs, "x", program_b)
    end
  end

  defp parse(input) do
    input
    |> String.split(",")
    |> Enum.map(&parse_step/1)
  end
end
