defmodule ElixirSolutions.DaySixteen do
  require ElixirSolutions.SantaWatch, as: SantaWatch

  def part_one(input) do
    {samples, _instructions} = parse(input)
    Enum.count(samples, &behaves_like_three_or_more?/1)
  end

  def part_two(input) do
    {samples, instructions} = parse(input)

    opcodes =
      samples
      |> Enum.group_by(&elem(elem(&1, 0), 0))
      |> Enum.map(fn {id, samples} ->
        {id, find_opcode_match(SantaWatch.opcodes_by_name(), samples)}
      end)
      |> find_opcode_matches(%{})

    instructions
    |> Enum.reduce([0, 0, 0, 0], fn {id, a, b, c}, registers ->
      {_name, fun} = Map.get(opcodes, id)
      fun.(registers, a, b, c)
    end)
  end

  def find_opcode_matches(matches, matched) do
    {single_matches, remaining} =
      Enum.split_with(matches, fn {_id, matches} ->
        Enum.count(matches) == 1
      end)

    newly_matched = for {id, [name]} <- single_matches, into: %{}, do: {id, name}
    updated_matched = Map.merge(matched, newly_matched)

    if Enum.count(newly_matched) > 0 do
      newly_matched_names = Enum.map(newly_matched, &elem(&1, 1))

      updated_matches =
        Enum.map(remaining, fn {id, matches} ->
          {
            id,
            Enum.reject(matches, fn name ->
              Enum.member?(newly_matched_names, name)
            end)
          }
        end)

      find_opcode_matches(updated_matches, updated_matched)
    else
      updated_matched
    end
  end

  def find_opcode_match(opcodes, []) do
    opcodes
  end

  def find_opcode_match([opcode], _samples) do
    [opcode]
  end

  def find_opcode_match(opcodes, [sample | samples]) do
    find_opcode_match(Enum.filter(opcodes, &matching?(sample, elem(&1, 1))), samples)
  end

  def matching?({{_id, a, b, c}, registers_before, registers_after}, opcode) do
    opcode.(registers_before, a, b, c) == registers_after
  end

  def behaves_like_three_or_more?({{_opcode, a, b, c}, registers_before, registers_after}) do
    Enum.reduce_while(SantaWatch.all_opcodes(), 0, fn opcode, total ->
      if opcode.(registers_before, a, b, c) == registers_after do
        if total == 2 do
          {:halt, :ok}
        else
          {:cont, total + 1}
        end
      else
        {:cont, total}
      end
    end) == :ok
  end

  @doc ~S"""
    iex> parse("Before: [1, 1, 2, 2]
    ...>12 1 2 2
    ...>After:  [1, 1, 0, 2]
    ...>
    ...>Before: [1, 1, 2, 1]
    ...>10 3 2 0
    ...>After:  [1, 1, 2, 1]
    ...>
    ...>Before: [2, 0, 1, 1]
    ...>3 0 3 3
    ...>After:  [2, 0, 1, 1]
    ...>
    ...>
    ...>
    ...>8 0 0 2
    ...>5 2 2 2
    ...>6 3 1 1
    ...>8 0 0 3
    ...>")
    {
      [
        {{12, 1, 2, 2}, [1, 1, 2, 2], [1, 1, 0, 2]},
        {{10, 3, 2, 0}, [1, 1, 2, 1], [1, 1, 2, 1]},
        { {3, 0, 3, 3}, [2, 0, 1, 1], [2, 0, 1, 1]}
      ],
      [
        {8, 0, 0, 2},
        {5, 2, 2, 2},
        {6, 3, 1, 1},
        {8, 0, 0, 3}
      ]
    }
  """
  def parse(input) do
    [sample_strings, instruction_strings] = String.split(input, "\n\n\n", parts: 2)

    samples =
      sample_strings
      |> String.trim()
      |> String.split("\n\n")
      |> Enum.map(&parse_sample/1)

    instructions =
      instruction_strings
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(&parse_instruction/1)

    {samples, instructions}
  end

  defp parse_sample(string) do
    ["Before: " <> registers_before, instruction_string, "After:  " <> registers_after] =
      String.split(string, "\n", parts: 3)

    {parse_instruction(instruction_string), parse_registers(registers_before),
     parse_registers(registers_after)}
  end

  defp parse_registers(string) do
    string
    |> String.slice(1..10)
    |> String.split(", ")
    |> Enum.map(&String.to_integer/1)
  end

  defp parse_instruction(string) do
    [id, a, b, c] = string |> String.split(" ") |> Enum.map(&String.to_integer/1)
    {id, a, b, c}
  end
end
