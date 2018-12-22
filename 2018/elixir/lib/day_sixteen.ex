defmodule ElixirSolutions.DaySixteen do
  use Bitwise

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
        {id, find_opcode_match(opcodes_by_name(), samples)}
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

  def opcodes_by_name() do
    [
      addr: &addr/4,
      addi: &addi/4,
      mulr: &mulr/4,
      muli: &muli/4,
      banr: &banr/4,
      bani: &bani/4,
      borr: &borr/4,
      bori: &bori/4,
      setr: &setr/4,
      seti: &seti/4,
      gtir: &gtir/4,
      gtri: &gtri/4,
      gtrr: &gtrr/4,
      eqir: &eqir/4,
      eqri: &eqri/4,
      eqrr: &eqrr/4
    ]
  end

  def all_opcodes(), do: for({_name, fun} <- opcodes_by_name(), do: fun)

  def behaves_like_three_or_more?({{_opcode, a, b, c}, registers_before, registers_after}) do
    Enum.reduce_while(all_opcodes(), 0, fn opcode, total ->
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

  def get(registers, id) when is_integer(id) and id >= 0 and id <= 3 do
    Enum.at(registers, id)
  end

  def set(registers, id, value) when is_integer(id) and id >= 0 and id <= 3 do
    List.replace_at(registers, id, value)
  end

  @doc ~S"""
    addr (add register) stores into register C the result of adding register A and register B.
    iex> addr([0, 1, 2, 3], 0, 1, 3)
    [0, 1, 2, 1]
  """
  def addr(registers, a, b, c) do
    set(registers, c, get(registers, a) + get(registers, b))
  end

  @doc ~S"""
    addi (add immediate) stores into register C the result of adding register A and value B.
    iex> addi([0, 1, 2, 3], 1, 7, 3)
    [0, 1, 2, 8]
  """
  def addi(registers, a, b, c) do
    set(registers, c, get(registers, a) + b)
  end

  @doc ~S"""
    mulr (multiply register) stores into register C the result of multiplying register A and register B.
    iex> mulr([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 2]
  """
  def mulr(registers, a, b, c) do
    set(registers, c, get(registers, a) * get(registers, b))
  end

  @doc ~S"""
    muli (multiply immediate) stores into register C the result of multiplying register A and value B.
    iex> muli([0, 1, 2, 3], 1, 7, 3)
    [0, 1, 2, 7]
  """
  def muli(registers, a, b, c) do
    set(registers, c, get(registers, a) * b)
  end

  @doc ~S"""
    banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    iex> banr([0, 1, 2, 3], 0, 2, 3)
    [0, 1, 2, 0]
  """
  def banr(registers, a, b, c) do
    set(registers, c, get(registers, a) &&& get(registers, b))
  end

  @doc ~S"""
    bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    iex> bani([0, 1, 2, 3], 2, 3, 3)
    [0, 1, 2, 2]
  """
  def bani(registers, a, b, c) do
    set(registers, c, get(registers, a) &&& b)
  end

  @doc ~S"""
    borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    iex> borr([0, 1, 2, 3], 0, 1, 3)
    [0, 1, 2, 1]
  """
  def borr(registers, a, b, c) do
    set(registers, c, get(registers, a) ||| get(registers, b))
  end

  @doc ~S"""
    bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    iex> bori([0, 1, 2, 3], 3, 9, 3)
    [0, 1, 2, 11]
  """
  def bori(registers, a, b, c) do
    set(registers, c, get(registers, a) ||| b)
  end

  @doc ~S"""
    setr (set register) copies the contents of register A into register C. (Input B is ignored.)
    iex> setr([0, 1, 2, 3], 1, 42, 3)
    [0, 1, 2, 1]
  """
  def setr(registers, a, _b, c) do
    set(registers, c, get(registers, a))
  end

  @doc ~S"""
    seti (set immediate) stores value A into register C. (Input B is ignored.)
    iex> seti([0, 1, 2, 3], 8, 42, 0)
    [8, 1, 2, 3]
  """
  def seti(registers, a, _b, c) do
    set(registers, c, a)
  end

  @doc ~S"""
    gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    iex> gtir([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 0]

    iex> gtir([0, 1, 2, 3], 1, 0, 3)
    [0, 1, 2, 1]
  """
  def gtir(registers, a, b, c) do
    set(registers, c, if(a > get(registers, b), do: 1, else: 0))
  end

  @doc ~S"""
    gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    iex> gtri([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 0]

    iex> gtri([0, 1, 2, 3], 2, 1, 3)
    [0, 1, 2, 1]
  """
  def gtri(registers, a, b, c) do
    set(registers, c, if(get(registers, a) > b, do: 1, else: 0))
  end

  @doc ~S"""
    gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    iex> gtrr([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 0]

    iex> gtrr([0, 1, 2, 3], 2, 1, 3)
    [0, 1, 2, 1]
  """
  def gtrr(registers, a, b, c) do
    set(
      registers,
      c,
      if(get(registers, a) > get(registers, b), do: 1, else: 0)
    )
  end

  @doc ~S"""
    eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    iex> eqir([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 1]

    iex> eqir([0, 1, 2, 3], 0, 1, 3)
    [0, 1, 2, 0]
  """
  def eqir(registers, a, b, c) do
    set(registers, c, if(a == get(registers, b), do: 1, else: 0))
  end

  @doc ~S"""
    eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    iex> eqri([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 1]

    iex> eqri([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 0]
  """
  def eqri(registers, a, b, c) do
    set(registers, c, if(get(registers, a) == b, do: 1, else: 0))
  end

  @doc ~S"""
    eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    iex> eqrr([0, 1, 2, 3], 1, 1, 3)
    [0, 1, 2, 1]

    iex> eqrr([0, 1, 2, 3], 1, 2, 3)
    [0, 1, 2, 0]
  """
  def eqrr(registers, a, b, c) do
    set(
      registers,
      c,
      if(get(registers, a) == get(registers, b), do: 1, else: 0)
    )
  end
end
